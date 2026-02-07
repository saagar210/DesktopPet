use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{
    events::{EVENT_PET_STATE_CHANGED, EVENT_SETTINGS_CHANGED},
    models::{Settings, SettingsPatch},
};

const ALLOWED_PRESETS: &[&str] = &["short", "standard", "long"];
const ALLOWED_THEMES: &[&str] = &["sunrise", "dusk", "mint", "mono"];
const ALLOWED_SKINS: &[&str] = &["classic", "neon", "pixel", "plush"];
const ALLOWED_SCENES: &[&str] = &["meadow", "forest", "space", "cozy_room"];
const MAX_FOCUS_LIST_ITEMS: usize = 50;
const MAX_FOCUS_LIST_ENTRY_CHARS: usize = 120;

fn is_allowed(value: &str, allowed: &[&str]) -> bool {
    allowed.iter().any(|candidate| *candidate == value)
}

fn normalize_host_pattern(value: &str) -> Option<String> {
    let mut normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return None;
    }
    normalized = normalized
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .split('/')
        .next()
        .unwrap_or_default()
        .split('?')
        .next()
        .unwrap_or_default()
        .split('#')
        .next()
        .unwrap_or_default()
        .trim()
        .trim_matches('.')
        .to_string();
    if normalized.is_empty() {
        return None;
    }
    if normalized.chars().count() > MAX_FOCUS_LIST_ENTRY_CHARS {
        return None;
    }
    if !normalized
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-')
    {
        return None;
    }
    Some(normalized)
}

fn normalize_host_list(input: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    for host in input {
        if output.len() >= MAX_FOCUS_LIST_ITEMS {
            break;
        }
        let Some(normalized) = normalize_host_pattern(&host) else {
            continue;
        };
        if !output.contains(&normalized) {
            output.push(normalized);
        }
    }
    output
}

fn sanitize_loaded_settings(settings: &mut Settings) {
    if !is_allowed(&settings.timer_preset, ALLOWED_PRESETS) {
        settings.timer_preset = "standard".to_string();
    }
    if !is_allowed(&settings.ui_theme, ALLOWED_THEMES) {
        settings.ui_theme = "sunrise".to_string();
    }
    if !is_allowed(&settings.pet_skin, ALLOWED_SKINS) {
        settings.pet_skin = "classic".to_string();
    }
    if !is_allowed(&settings.pet_scene, ALLOWED_SCENES) {
        settings.pet_scene = "meadow".to_string();
    }
    settings.sound_volume = if settings.sound_volume.is_finite() {
        settings.sound_volume.clamp(0.0, 1.0)
    } else {
        0.7
    };
    settings.focus_allowlist = normalize_host_list(std::mem::take(&mut settings.focus_allowlist));
    settings.focus_blocklist = normalize_host_list(std::mem::take(&mut settings.focus_blocklist));
}

fn sanitize_patch(mut patch: SettingsPatch) -> Result<SettingsPatch, String> {
    if let Some(preset) = patch.timer_preset.as_deref() {
        if !is_allowed(preset, ALLOWED_PRESETS) {
            return Err(format!("Invalid timer preset: {}", preset));
        }
    }
    if let Some(theme) = patch.ui_theme.as_deref() {
        if !is_allowed(theme, ALLOWED_THEMES) {
            return Err(format!("Invalid UI theme: {}", theme));
        }
    }
    if let Some(skin) = patch.pet_skin.as_deref() {
        if !is_allowed(skin, ALLOWED_SKINS) {
            return Err(format!("Invalid pet skin: {}", skin));
        }
    }
    if let Some(scene) = patch.pet_scene.as_deref() {
        if !is_allowed(scene, ALLOWED_SCENES) {
            return Err(format!("Invalid pet scene: {}", scene));
        }
    }
    if let Some(volume) = patch.sound_volume {
        patch.sound_volume = Some(if volume.is_finite() {
            volume.clamp(0.0, 1.0)
        } else {
            0.7
        });
    }
    if let Some(allowlist) = patch.focus_allowlist.take() {
        patch.focus_allowlist = Some(normalize_host_list(allowlist));
    }
    if let Some(blocklist) = patch.focus_blocklist.take() {
        patch.focus_blocklist = Some(normalize_host_list(blocklist));
    }

    Ok(patch)
}

fn load_settings(app: &AppHandle) -> Result<Settings, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let mut settings = store
        .get("settings")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    sanitize_loaded_settings(&mut settings);
    Ok(settings)
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
    let patch = sanitize_patch(patch)?;
    let patch_copy = patch.clone();
    let mut settings = load_settings(&app)?;
    patch.apply_to(&mut settings);
    sanitize_loaded_settings(&mut settings);

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

#[cfg(test)]
mod tests {
    use super::{normalize_host_list, normalize_host_pattern};

    #[test]
    fn normalize_host_pattern_handles_url_and_case() {
        let host = normalize_host_pattern("https://Docs.YouTube.com/watch?v=1").unwrap();
        assert_eq!(host, "docs.youtube.com");
    }

    #[test]
    fn normalize_host_pattern_rejects_invalid_chars() {
        assert!(normalize_host_pattern("exa mple.com").is_none());
        assert!(normalize_host_pattern("exa*mple.com").is_none());
    }

    #[test]
    fn normalize_host_list_deduplicates() {
        let hosts = normalize_host_list(vec![
            "youtube.com".to_string(),
            "https://youtube.com".to_string(),
            "docs.youtube.com".to_string(),
        ]);
        assert_eq!(
            hosts,
            vec!["youtube.com".to_string(), "docs.youtube.com".to_string()]
        );
    }
}
