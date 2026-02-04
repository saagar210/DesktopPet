use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::EVENT_FOCUS_GUARDRAILS_ALERT,
    models::{FocusGuardrailEvent, FocusGuardrailsStatus, Settings},
};

const MAX_GUARDRAIL_EVENTS: usize = 120;

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

fn match_list(hosts: &[String], patterns: &[String]) -> Vec<String> {
    hosts
        .iter()
        .filter(|host| patterns.iter().any(|pattern| host.contains(pattern)))
        .cloned()
        .collect()
}

fn nudge_level_for_count(count: usize) -> &'static str {
    if count >= 3 {
        "high"
    } else if count >= 1 {
        "medium"
    } else {
        "none"
    }
}

fn load_guardrail_events(app: &AppHandle) -> Result<Vec<FocusGuardrailEvent>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("focus_guardrail_events")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

fn save_guardrail_events(app: &AppHandle, events: &[FocusGuardrailEvent]) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("focus_guardrail_events", serde_json::json!(events));
    Ok(())
}

fn append_guardrail_event(
    app: &AppHandle,
    status: &FocusGuardrailsStatus,
    hosts: &[String],
) -> Result<(), String> {
    let mut events = load_guardrail_events(app)?;
    events.push(FocusGuardrailEvent {
        id: uuid::Uuid::new_v4().to_string(),
        phase: status.phase.clone(),
        hosts: hosts.to_vec(),
        matched_blocklist: status.matched_blocklist.clone(),
        nudge_level: status.nudge_level.clone(),
        recommended_action: status.recommended_action.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
    });
    events.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    if events.len() > MAX_GUARDRAIL_EVENTS {
        events.truncate(MAX_GUARDRAIL_EVENTS);
    }
    save_guardrail_events(app, &events)
}

#[cfg(test)]
mod tests {
    use super::{match_list, nudge_level_for_count};

    #[test]
    fn match_list_finds_partial_matches() {
        let hosts = vec!["docs.youtube.com".to_string(), "openai.com".to_string()];
        let patterns = vec!["youtube".to_string()];
        let matched = match_list(&hosts, &patterns);
        assert_eq!(matched, vec!["docs.youtube.com".to_string()]);
    }

    #[test]
    fn match_list_returns_empty_when_no_patterns() {
        let hosts = vec!["example.com".to_string()];
        let patterns = vec![];
        let matched = match_list(&hosts, &patterns);
        assert!(matched.is_empty());
    }

    #[test]
    fn nudge_level_thresholds() {
        assert_eq!(nudge_level_for_count(0), "none");
        assert_eq!(nudge_level_for_count(1), "medium");
        assert_eq!(nudge_level_for_count(2), "medium");
        assert_eq!(nudge_level_for_count(3), "high");
    }
}

#[tauri::command]
pub fn evaluate_focus_guardrails(
    app: AppHandle,
    phase: String,
    hosts: Option<Vec<String>>,
) -> Result<FocusGuardrailsStatus, String> {
    let settings = load_settings(&app)?;
    let hosts = hosts.unwrap_or_default();
    let matched_blocklist = match_list(&hosts, &settings.focus_blocklist);
    let matched_allowlist = match_list(&hosts, &settings.focus_allowlist);

    let phase_is_work = phase == "work";
    let should_apply = settings.focus_guardrails_enabled
        && (!settings.focus_guardrails_work_only || phase_is_work);
    let active = should_apply && !matched_blocklist.is_empty();
    let blocked_hosts_count = matched_blocklist.len() as u32;
    let nudge_level = nudge_level_for_count(matched_blocklist.len()).to_string();
    let recommended_action = if !should_apply {
        "none".to_string()
    } else if matched_blocklist.len() >= 3 {
        "pause_timer".to_string()
    } else if matched_blocklist.len() >= 1 {
        "show_nudge".to_string()
    } else {
        "none".to_string()
    };

    let message = if !settings.focus_guardrails_enabled {
        "Focus guardrails are disabled.".to_string()
    } else if settings.focus_guardrails_work_only && !phase_is_work {
        "Guardrails are configured for work sessions only.".to_string()
    } else if active {
        format!(
            "Guardrails active: {} potential distractions matched.",
            matched_blocklist.len()
        )
    } else {
        "Guardrails enabled and monitoring.".to_string()
    };

    Ok(FocusGuardrailsStatus {
        enabled: settings.focus_guardrails_enabled,
        active,
        phase,
        matched_blocklist,
        matched_allowlist,
        blocked_hosts_count,
        nudge_level,
        recommended_action,
        message,
    })
}

#[tauri::command]
pub fn apply_focus_guardrails_intervention(
    app: AppHandle,
    phase: String,
    hosts: Option<Vec<String>>,
) -> Result<FocusGuardrailsStatus, String> {
    let sampled_hosts = hosts.unwrap_or_default();
    let mut status = evaluate_focus_guardrails(
        app.clone(),
        phase,
        Some(sampled_hosts.clone()),
    )?;
    if status.recommended_action == "pause_timer" {
        status.message = "Guardrails intervention triggered: consider pausing and regrouping.".to_string();
        status.active = true;
    }
    if status.active {
        let _ = append_guardrail_event(&app, &status, &sampled_hosts);
        let _ = crate::progression::record_guardrail_intervention(&app, &status.nudge_level);
        let _ = app.emit(EVENT_FOCUS_GUARDRAILS_ALERT, &status);
    }
    Ok(status)
}

#[tauri::command]
pub fn get_focus_guardrail_events(
    app: AppHandle,
    limit: Option<u32>,
) -> Result<Vec<FocusGuardrailEvent>, String> {
    let mut events = load_guardrail_events(&app)?;
    let cap = limit.unwrap_or(30).clamp(1, 120) as usize;
    events.truncate(cap);
    Ok(events)
}
