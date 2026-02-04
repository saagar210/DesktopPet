use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::{EVENT_PET_STATE_CHANGED, EVENT_SETTINGS_CHANGED},
    models::{Settings, SettingsPatch},
};

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings, String> {
    load_settings(&app)
}

#[tauri::command]
pub fn update_settings(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    patch: SettingsPatch,
) -> Result<Settings, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let patch_copy = patch.clone();
    let mut settings = load_settings(&app)?;
    patch.apply_to(&mut settings);

    store.set("settings", json!(settings));
    let _ = app.emit(EVENT_SETTINGS_CHANGED, &settings);

    if patch_copy.pet_skin.is_some() || patch_copy.pet_scene.is_some() {
        let mut pet: crate::models::PetState = store
            .get("pet")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();
        if let Some(skin) = patch_copy.pet_skin {
            pet.skin = skin;
        }
        if let Some(scene) = patch_copy.pet_scene {
            pet.scene = scene;
        }
        store.set("pet", json!(pet));
        let _ = app.emit(EVENT_PET_STATE_CHANGED, pet);
    }

    Ok(settings)
}
