use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::{
        EVENT_ANALYTICS_CHANGED, EVENT_COINS_CHANGED, EVENT_GOALS_CHANGED, EVENT_PET_STATE_CHANGED,
        EVENT_PROFILE_CHANGED, EVENT_SETTINGS_CHANGED, EVENT_TIMER_RUNTIME_CHANGED,
    },
    models::{AppDiagnostics, AppSnapshot, CURRENT_SCHEMA_VERSION},
};

const MAX_TASKS: usize = 2_000;
const MAX_SESSIONS: usize = 10_000;
const MAX_SUMMARIES: usize = 365;
const MAX_LOADOUTS: usize = 200;
const MAX_PET_EVENTS: usize = 200;
const MAX_GUARDRAIL_EVENTS: usize = 500;

fn cap_len<T>(values: &mut Vec<T>, max_len: usize) {
    if values.len() > max_len {
        values.truncate(max_len);
    }
}

fn sanitize_snapshot(mut snapshot: AppSnapshot) -> AppSnapshot {
    snapshot.schema_version = CURRENT_SCHEMA_VERSION;

    crate::commands::settings::sanitize_settings(&mut snapshot.settings);

    snapshot.tasks.retain(|task| !task.title.trim().is_empty());
    for task in &mut snapshot.tasks {
        task.title = task.title.trim().chars().take(140).collect();
    }

    snapshot.timer_runtime.total_seconds =
        snapshot.timer_runtime.total_seconds.clamp(60, 3 * 60 * 60);
    snapshot.timer_runtime.seconds_left = snapshot
        .timer_runtime
        .seconds_left
        .min(snapshot.timer_runtime.total_seconds);

    cap_len(&mut snapshot.tasks, MAX_TASKS);
    cap_len(&mut snapshot.sessions, MAX_SESSIONS);
    cap_len(&mut snapshot.summaries, MAX_SUMMARIES);
    cap_len(&mut snapshot.customization_loadouts, MAX_LOADOUTS);
    cap_len(&mut snapshot.pet_events, MAX_PET_EVENTS);
    cap_len(&mut snapshot.focus_guardrail_events, MAX_GUARDRAIL_EVENTS);

    snapshot
}

fn load_snapshot(app: &AppHandle) -> Result<AppSnapshot, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let schema_version = store
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or(CURRENT_SCHEMA_VERSION);

    Ok(sanitize_snapshot(AppSnapshot {
        schema_version,
        exported_at: chrono::Utc::now().to_rfc3339(),
        pet: store
            .get("pet")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        coins: store
            .get("coins")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        tasks: store
            .get("tasks")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        goals: store
            .get("goals")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        sessions: store
            .get("sessions")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        settings: store
            .get("settings")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        timer_runtime: store
            .get("timer_runtime")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        progress: store
            .get("user_progress")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        summaries: store
            .get("daily_summaries")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        customization_loadouts: store
            .get("customization_loadouts")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        pet_events: store
            .get("pet_events")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        pet_active_quest: store
            .get("pet_active_quest")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        focus_guardrail_events: store
            .get("focus_guardrail_events")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
    }))
}

fn save_snapshot(app: &AppHandle, snapshot: AppSnapshot) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let snapshot = sanitize_snapshot(snapshot);

    // Persist only canonical keys and flush immediately for explicit import/reset flows.
    store.clear();
    store.set("schema_version", json!(CURRENT_SCHEMA_VERSION));
    store.set("pet", json!(snapshot.pet));
    store.set("coins", json!(snapshot.coins));
    store.set("tasks", json!(snapshot.tasks));
    store.set("goals", json!(snapshot.goals));
    store.set("sessions", json!(snapshot.sessions));
    store.set("settings", json!(snapshot.settings));
    store.set("timer_runtime", json!(snapshot.timer_runtime));
    store.set("user_progress", json!(snapshot.progress));
    store.set("daily_summaries", json!(snapshot.summaries));
    store.set(
        "customization_loadouts",
        json!(snapshot.customization_loadouts),
    );
    store.set("pet_events", json!(snapshot.pet_events));
    store.set("pet_active_quest", json!(snapshot.pet_active_quest));
    store.set(
        "focus_guardrail_events",
        json!(snapshot.focus_guardrail_events),
    );
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

fn emit_snapshot_refresh_events(app: &AppHandle, snapshot: &AppSnapshot) {
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &snapshot.pet);
    let _ = app.emit(EVENT_COINS_CHANGED, &snapshot.coins);
    let _ = app.emit(EVENT_GOALS_CHANGED, &snapshot.goals);
    let _ = app.emit(EVENT_SETTINGS_CHANGED, &snapshot.settings);
    let _ = app.emit(EVENT_TIMER_RUNTIME_CHANGED, &snapshot.timer_runtime);
    let _ = app.emit(EVENT_PROFILE_CHANGED, &snapshot.progress);
    let _ = app.emit(EVENT_ANALYTICS_CHANGED, &snapshot.summaries);
}

#[tauri::command]
pub fn export_app_snapshot(app: AppHandle) -> Result<AppSnapshot, String> {
    load_snapshot(&app)
}

#[tauri::command]
pub fn import_app_snapshot(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    snapshot: AppSnapshot,
) -> Result<String, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let snapshot = sanitize_snapshot(snapshot);
    save_snapshot(&app, snapshot.clone())?;
    emit_snapshot_refresh_events(&app, &snapshot);
    Ok("Import complete".to_string())
}

#[tauri::command]
pub fn reset_app_state(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
) -> Result<String, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let snapshot = AppSnapshot::default();
    save_snapshot(&app, snapshot.clone())?;
    emit_snapshot_refresh_events(&app, &snapshot);
    Ok("App data reset to defaults".to_string())
}

#[tauri::command]
pub fn get_app_diagnostics(app: AppHandle) -> Result<AppDiagnostics, String> {
    let snapshot = load_snapshot(&app)?;

    Ok(AppDiagnostics {
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        schema_version: snapshot.schema_version,
        current_schema_version: CURRENT_SCHEMA_VERSION,
        exported_at: snapshot.exported_at,
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        tasks_count: u32::try_from(snapshot.tasks.len()).unwrap_or(u32::MAX),
        sessions_count: u32::try_from(snapshot.sessions.len()).unwrap_or(u32::MAX),
        summaries_count: u32::try_from(snapshot.summaries.len()).unwrap_or(u32::MAX),
        guardrail_events_count: u32::try_from(snapshot.focus_guardrail_events.len())
            .unwrap_or(u32::MAX),
        has_active_quest: snapshot.pet_active_quest.is_some(),
    })
}

#[cfg(test)]
mod tests {
    use super::sanitize_snapshot;
    use crate::models::{AppSnapshot, Task, TimerRuntimeState};

    #[test]
    fn sanitize_snapshot_trims_empty_tasks_and_caps_lengths() {
        let mut snapshot = AppSnapshot::default();
        snapshot.tasks = vec![
            Task {
                id: "1".to_string(),
                title: " ".to_string(),
                completed: false,
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
            Task {
                id: "2".to_string(),
                title: "  Keep this task  ".to_string(),
                completed: false,
                created_at: "2026-01-01T00:00:00Z".to_string(),
            },
        ];

        let sanitized = sanitize_snapshot(snapshot);
        assert_eq!(sanitized.tasks.len(), 1);
        assert_eq!(sanitized.tasks[0].title, "Keep this task");
    }

    #[test]
    fn sanitize_snapshot_clamps_runtime() {
        let snapshot = AppSnapshot {
            timer_runtime: TimerRuntimeState {
                phase: "work".to_string(),
                seconds_left: 999_999,
                total_seconds: 999_999,
                paused: false,
                session_id: Some("abc".to_string()),
                sessions_completed: 1,
                preset: "standard".to_string(),
                last_updated_at: "2026-01-01T00:00:00Z".to_string(),
            },
            ..AppSnapshot::default()
        };

        let sanitized = sanitize_snapshot(snapshot);
        assert_eq!(sanitized.timer_runtime.total_seconds, 3 * 60 * 60);
        assert_eq!(sanitized.timer_runtime.seconds_left, 3 * 60 * 60);
    }

    #[test]
    fn sanitize_snapshot_normalizes_settings() {
        let mut snapshot = AppSnapshot::default();
        snapshot.settings.timer_preset = "very-long".to_string();
        snapshot.settings.ui_theme = "neon".to_string();
        snapshot.settings.pet_skin = "retro".to_string();
        snapshot.settings.pet_scene = "moon".to_string();
        snapshot.settings.sound_volume = f32::NAN;
        snapshot.settings.focus_allowlist = vec![
            "https://Docs.YouTube.com/watch?v=1".to_string(),
            "docs.youtube.com".to_string(),
            "bad host".to_string(),
        ];
        snapshot.settings.focus_blocklist = vec![
            "Example.com".to_string(),
            "https://example.com/path".to_string(),
            "bad*host".to_string(),
        ];

        let sanitized = sanitize_snapshot(snapshot);
        assert_eq!(sanitized.settings.timer_preset, "standard");
        assert_eq!(sanitized.settings.ui_theme, "sunrise");
        assert_eq!(sanitized.settings.pet_skin, "classic");
        assert_eq!(sanitized.settings.pet_scene, "meadow");
        assert_eq!(sanitized.settings.sound_volume, 0.7);
        assert_eq!(
            sanitized.settings.focus_allowlist,
            vec!["docs.youtube.com".to_string()]
        );
        assert_eq!(
            sanitized.settings.focus_blocklist,
            vec!["example.com".to_string()]
        );
    }
}
