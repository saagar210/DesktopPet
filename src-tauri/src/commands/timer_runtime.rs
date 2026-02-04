use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::EVENT_TIMER_RUNTIME_CHANGED,
    models::{Settings, TimerRuntimeState},
};

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
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

    Ok(apply_elapsed(runtime))
}

#[tauri::command]
pub fn save_timer_runtime(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    runtime: TimerRuntimeState,
) -> Result<TimerRuntimeState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
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
