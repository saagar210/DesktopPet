use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::models::{CoinBalance, PetState, Settings, TimerRuntimeState, CURRENT_SCHEMA_VERSION};

fn normalized_schema(existing: Option<u64>) -> u32 {
    existing
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or(CURRENT_SCHEMA_VERSION)
}

pub fn ensure_schema_version(app: &AppHandle) -> Result<u32, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let existing = store.get("schema_version").and_then(|v| v.as_u64());
    let current = normalized_schema(existing);

    if existing.is_none() || current != CURRENT_SCHEMA_VERSION {
        store.set("schema_version", json!(CURRENT_SCHEMA_VERSION));
    }
    if store.get("settings").is_none() {
        store.set("settings", json!(Settings::default()));
    }
    if store.get("timer_runtime").is_none() {
        store.set("timer_runtime", json!(TimerRuntimeState::default()));
    }
    if store.get("pet").is_none() {
        store.set("pet", json!(PetState::default()));
    }
    if store.get("coins").is_none() {
        store.set("coins", json!(CoinBalance::default()));
    }

    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::normalized_schema;
    use crate::models::CURRENT_SCHEMA_VERSION;

    #[test]
    fn normalized_schema_uses_current_when_missing() {
        assert_eq!(normalized_schema(None), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn normalized_schema_uses_current_when_invalid() {
        assert_eq!(normalized_schema(Some(u64::MAX)), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn normalized_schema_uses_existing_when_valid() {
        assert_eq!(normalized_schema(Some(3)), 3);
    }
}
