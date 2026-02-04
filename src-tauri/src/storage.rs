use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::models::CURRENT_SCHEMA_VERSION;

pub fn ensure_schema_version(app: &AppHandle) -> Result<u32, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let existing = store.get("schema_version").and_then(|v| v.as_u64());

    let current = existing
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or(CURRENT_SCHEMA_VERSION);

    if existing.is_none() || current != CURRENT_SCHEMA_VERSION {
        store.set("schema_version", json!(CURRENT_SCHEMA_VERSION));
    }

    Ok(current)
}
