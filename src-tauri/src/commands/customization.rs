use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::{EVENT_PET_STATE_CHANGED, EVENT_SETTINGS_CHANGED},
    models::{CustomizationLoadout, PetState, Settings},
};

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

fn load_pet(app: &AppHandle) -> Result<PetState, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

#[tauri::command]
pub fn get_customization_loadouts(app: AppHandle) -> Result<Vec<CustomizationLoadout>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let loadouts = store
        .get("customization_loadouts")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(loadouts)
}

#[tauri::command]
pub fn save_customization_loadout(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    loadout: CustomizationLoadout,
) -> Result<Vec<CustomizationLoadout>, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let mut loadouts: Vec<CustomizationLoadout> = store
        .get("customization_loadouts")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    if let Some(existing) = loadouts.iter_mut().find(|item| item.name == loadout.name) {
        *existing = loadout;
    } else {
        loadouts.push(loadout);
    }

    loadouts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    store.set("customization_loadouts", json!(loadouts));
    Ok(loadouts)
}

#[tauri::command]
pub fn apply_customization_loadout(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    name: String,
) -> Result<CustomizationLoadout, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let loadouts: Vec<CustomizationLoadout> = store
        .get("customization_loadouts")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let loadout = loadouts
        .into_iter()
        .find(|item| item.name == name)
        .ok_or_else(|| "Loadout not found".to_string())?;

    let mut settings = load_settings(&app)?;
    settings.ui_theme = loadout.ui_theme.clone();
    settings.pet_skin = loadout.pet_skin.clone();
    settings.pet_scene = loadout.pet_scene.clone();
    store.set("settings", json!(settings));

    let mut pet = load_pet(&app)?;
    pet.skin = loadout.pet_skin.clone();
    pet.scene = loadout.pet_scene.clone();
    pet.accessories = loadout.accessories.clone();
    store.set("pet", json!(pet));

    let _ = app.emit(EVENT_SETTINGS_CHANGED, settings);
    let _ = app.emit(EVENT_PET_STATE_CHANGED, pet);
    Ok(loadout)
}
