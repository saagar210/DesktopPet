use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::models::PetState;

#[tauri::command]
pub fn get_pet_state(app: AppHandle) -> Result<PetState, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let pet: PetState = store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(pet)
}

#[tauri::command]
pub fn set_pet_animation(app: AppHandle, animation: String) -> Result<PetState, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let mut pet: PetState = store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    pet.animation_state = animation;
    store.set("pet", json!(pet));
    let _ = app.emit("pet-state-changed", &pet);
    Ok(pet)
}
