use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::EVENT_TIMER_RUNTIME_CHANGED,
    models::{Settings, TimerRuntimeState},
};

const MIN_TOTAL_SECONDS: u32 = 60;
const MAX_TOTAL_SECONDS: u32 = 3 * 60 * 60;

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    crate::commands::settings::get_settings(app.clone())
}

fn work_seconds_for_preset(preset: &str) -> u32 {
    match preset {
        "short" => 15 * 60,
        "long" => 50 * 60,
        _ => 25 * 60,
    }
}

fn default_runtime_for_preset(preset: &str) -> TimerRuntimeState {
    let work_seconds = work_seconds_for_preset(preset);
    TimerRuntimeState {
        preset: preset.to_string(),
        seconds_left: work_seconds,
        total_seconds: work_seconds,
        last_updated_at: chrono::Utc::now().to_rfc3339(),
        ..Default::default()
    }
}

fn normalize_phase(phase: &str) -> String {
    match phase {
        "idle" | "work" | "break" | "celebrating" => phase.to_string(),
        _ => "idle".to_string(),
    }
}

fn normalize_preset(preset: &str, fallback: &str) -> String {
    match preset {
        "short" | "standard" | "long" => preset.to_string(),
        _ => fallback.to_string(),
    }
}

fn sanitize_runtime(mut runtime: TimerRuntimeState, default_preset: &str) -> TimerRuntimeState {
    runtime.phase = normalize_phase(&runtime.phase);
    runtime.preset = normalize_preset(&runtime.preset, default_preset);
    runtime.total_seconds = runtime
        .total_seconds
        .clamp(MIN_TOTAL_SECONDS, MAX_TOTAL_SECONDS);
    runtime.seconds_left = runtime.seconds_left.min(runtime.total_seconds);
    if matches!(runtime.phase.as_str(), "idle" | "celebrating") {
        runtime.session_id = None;
    }
    runtime
}

fn apply_elapsed(mut runtime: TimerRuntimeState) -> TimerRuntimeState {
    if runtime.paused || !matches!(runtime.phase.as_str(), "work" | "break") {
        return runtime;
    }

    let elapsed_secs = chrono::DateTime::parse_from_rfc3339(&runtime.last_updated_at)
        .ok()
        .and_then(|last| {
            let now = chrono::Utc::now();
            let diff = now.signed_duration_since(last.with_timezone(&chrono::Utc));
            u32::try_from(diff.num_seconds()).ok()
        })
        .unwrap_or(0);

    if elapsed_secs > 0 {
        runtime.seconds_left = runtime.seconds_left.saturating_sub(elapsed_secs);
        runtime.last_updated_at = chrono::Utc::now().to_rfc3339();
    }

    runtime
}

#[tauri::command]
pub fn get_timer_runtime(app: AppHandle) -> Result<TimerRuntimeState, String> {
    let settings = load_settings(&app)?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let runtime = store
        .get("timer_runtime")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_else(|| default_runtime_for_preset(&settings.timer_preset));
    let runtime = sanitize_runtime(runtime, &settings.timer_preset);

    Ok(apply_elapsed(runtime))
}

#[tauri::command]
pub fn save_timer_runtime(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    runtime: TimerRuntimeState,
) -> Result<TimerRuntimeState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let settings = load_settings(&app)?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let runtime = sanitize_runtime(
        TimerRuntimeState {
            last_updated_at: chrono::Utc::now().to_rfc3339(),
            ..runtime
        },
        &settings.timer_preset,
    );
    let runtime = TimerRuntimeState {
        last_updated_at: chrono::Utc::now().to_rfc3339(),
        ..runtime
    };
    store.set("timer_runtime", json!(runtime));
    let _ = app.emit(EVENT_TIMER_RUNTIME_CHANGED, &runtime);
    Ok(runtime)
}

#[tauri::command]
pub fn clear_timer_runtime(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
) -> Result<TimerRuntimeState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let settings = load_settings(&app)?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let runtime = default_runtime_for_preset(&settings.timer_preset);

    store.set("timer_runtime", json!(runtime));
    let _ = app.emit(EVENT_TIMER_RUNTIME_CHANGED, &runtime);
    Ok(runtime)
}

#[cfg(test)]
mod tests {
    use super::{normalize_phase, sanitize_runtime, TimerRuntimeState, MAX_TOTAL_SECONDS};

    #[test]
    fn normalize_phase_falls_back_to_idle() {
        assert_eq!(normalize_phase("work"), "work");
        assert_eq!(normalize_phase("unknown"), "idle");
    }

    #[test]
    fn sanitize_runtime_clamps_seconds() {
        let runtime = TimerRuntimeState {
            phase: "work".to_string(),
            seconds_left: u32::MAX,
            total_seconds: u32::MAX,
            paused: false,
            session_id: Some("s1".to_string()),
            sessions_completed: 0,
            preset: "standard".to_string(),
            last_updated_at: chrono::Utc::now().to_rfc3339(),
        };
        let sanitized = sanitize_runtime(runtime, "standard");
        assert_eq!(sanitized.total_seconds, MAX_TOTAL_SECONDS);
        assert_eq!(sanitized.seconds_left, MAX_TOTAL_SECONDS);
    }

    #[test]
    fn sanitize_runtime_clears_session_when_idle() {
        let runtime = TimerRuntimeState {
            phase: "invalid".to_string(),
            seconds_left: 10,
            total_seconds: 10,
            paused: false,
            session_id: Some("s1".to_string()),
            sessions_completed: 0,
            preset: "invalid".to_string(),
            last_updated_at: chrono::Utc::now().to_rfc3339(),
        };
        let sanitized = sanitize_runtime(runtime, "short");
        assert_eq!(sanitized.phase, "idle");
        assert_eq!(sanitized.preset, "short");
        assert!(sanitized.session_id.is_none());
    }
}
