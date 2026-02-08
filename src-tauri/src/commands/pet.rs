use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::events::{EVENT_COINS_CHANGED, EVENT_PET_STATE_CHANGED};
use crate::models::{CoinBalance, PetEvent, PetQuest, PetState};

const MAX_PET_EVENTS: usize = 30;
const MAX_SPECIES_ID_CHARS: usize = 48;
const ALLOWED_ANIMATIONS: &[&str] = &[
    "idle",
    "working",
    "break",
    "celebrating",
    "evolving",
    "clicked",
];
const ALLOWED_SKINS: &[&str] = &["classic", "neon", "pixel", "plush"];
const ALLOWED_SCENES: &[&str] = &["meadow", "forest", "space", "cozy_room"];

fn validate_variant(value: String, allowed: &[&str], label: &str) -> Result<String, String> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(value)
    } else {
        Err(format!("Invalid {}: {}", label, value))
    }
}

fn normalize_species_id(value: String) -> Result<String, String> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err("Species id cannot be empty".to_string());
    }
    if normalized.chars().count() > MAX_SPECIES_ID_CHARS {
        return Err("Species id is too long".to_string());
    }
    if !normalized
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        return Err("Species id has invalid characters".to_string());
    }
    Ok(normalized)
}

fn normalize_evolution_thresholds(input: Option<Vec<u32>>) -> Vec<u32> {
    let mut values = input.unwrap_or_else(|| vec![0, 5, 15]);
    if values.len() < 3 {
        values = vec![0, 5, 15];
    }
    let stage1 = values[1].max(1);
    let stage2 = values[2].max(stage1 + 1);
    vec![0, stage1, stage2]
}

fn stage_for_total_pomodoros(total_pomodoros: u32, thresholds: &[u32]) -> u32 {
    if thresholds.len() < 3 {
        if total_pomodoros >= 15 {
            2
        } else if total_pomodoros >= 5 {
            1
        } else {
            0
        }
    } else if total_pomodoros >= thresholds[2] {
        2
    } else if total_pomodoros >= thresholds[1] {
        1
    } else {
        0
    }
}

fn clamp_metric(value: i32) -> u32 {
    value.clamp(0, 100) as u32
}

fn apply_care_decay(mut pet: PetState) -> PetState {
    let elapsed_hours = chrono::DateTime::parse_from_rfc3339(&pet.last_care_update_at)
        .ok()
        .and_then(|last| {
            let now = chrono::Utc::now();
            let diff = now.signed_duration_since(last.with_timezone(&chrono::Utc));
            i32::try_from(diff.num_hours()).ok()
        })
        .unwrap_or(0);

    if elapsed_hours <= 0 {
        return pet;
    }

    pet.energy = clamp_metric(pet.energy as i32 - elapsed_hours * 2);
    pet.hunger = clamp_metric(pet.hunger as i32 + elapsed_hours * 3);
    pet.cleanliness = clamp_metric(pet.cleanliness as i32 - elapsed_hours);
    pet.affection = clamp_metric(pet.affection as i32 - elapsed_hours / 2);
    pet.last_care_update_at = chrono::Utc::now().to_rfc3339();

    pet.mood = if pet.energy < 25 {
        "sleepy".to_string()
    } else if pet.hunger > 75 {
        "hungry".to_string()
    } else if pet.affection > 75 {
        "happy".to_string()
    } else if pet.cleanliness < 30 {
        "messy".to_string()
    } else {
        "content".to_string()
    };

    pet
}

fn load_pet(app: &AppHandle) -> Result<PetState, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let pet: PetState = store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(apply_care_decay(pet))
}

fn save_pet(app: &AppHandle, pet: &PetState) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("pet", json!(pet));
    Ok(())
}

fn quest_target_for_stage(stage: u32) -> u32 {
    match stage {
        0 => 1,
        1 => 2,
        _ => 3,
    }
}

fn quest_reward_for_stage(stage: u32) -> u32 {
    12 + (stage * 4)
}

#[derive(Clone, Copy)]
struct QuestTemplate {
    kind: &'static str,
    title: &'static str,
    description: &'static str,
}

const QUEST_TEMPLATES: [QuestTemplate; 2] = [
    QuestTemplate {
        kind: "focus_sessions",
        title: "Steady Focus",
        description: "Complete calm focus sessions with your companion.",
    },
    QuestTemplate {
        kind: "care_actions",
        title: "Gentle Care",
        description: "Do small care actions to keep your companion cozy.",
    },
];

fn create_quest_for_pet(pet: &PetState) -> PetQuest {
    let template = QUEST_TEMPLATES[(pet.total_pomodoros as usize) % QUEST_TEMPLATES.len()];
    let stage_target = quest_target_for_stage(pet.current_stage);
    let target_sessions = if template.kind == "care_actions" {
        stage_target + 1
    } else {
        stage_target
    };
    let reward = if template.kind == "care_actions" {
        quest_reward_for_stage(pet.current_stage).saturating_sub(2).max(8)
    } else {
        quest_reward_for_stage(pet.current_stage)
    };

    PetQuest {
        id: uuid::Uuid::new_v4().to_string(),
        kind: template.kind.to_string(),
        title: template.title.to_string(),
        description: template.description.to_string(),
        target_sessions,
        completed_sessions: 0,
        reward_coins: reward,
        created_at: chrono::Utc::now().to_rfc3339(),
    }
}

fn load_events(app: &AppHandle) -> Result<Vec<PetEvent>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("pet_events")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

fn save_events(app: &AppHandle, events: &[PetEvent]) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("pet_events", json!(events));
    Ok(())
}

fn save_bounded_events(
    app: &AppHandle,
    mut events: Vec<PetEvent>,
) -> Result<Vec<PetEvent>, String> {
    events.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    if events.len() > MAX_PET_EVENTS {
        events.truncate(MAX_PET_EVENTS);
    }
    save_events(app, &events)?;
    Ok(events)
}

fn append_event(
    app: &AppHandle,
    kind: &str,
    description: String,
    resolved: bool,
) -> Result<(), String> {
    let mut events = load_events(app)?;
    events.push(PetEvent {
        id: uuid::Uuid::new_v4().to_string(),
        kind: kind.to_string(),
        description,
        created_at: chrono::Utc::now().to_rfc3339(),
        resolved,
    });
    let _ = save_bounded_events(app, events)?;
    Ok(())
}

fn load_active_quest(app: &AppHandle) -> Result<Option<PetQuest>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("pet_active_quest")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or(None))
}

fn save_active_quest(app: &AppHandle, quest: Option<&PetQuest>) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("pet_active_quest", json!(quest));
    Ok(())
}

#[tauri::command]
pub fn get_pet_state(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
) -> Result<PetState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let pet = load_pet(&app)?;
    save_pet(&app, &pet)?;
    Ok(pet)
}

#[tauri::command]
pub fn set_pet_animation(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    animation: String,
) -> Result<PetState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let animation = validate_variant(animation, ALLOWED_ANIMATIONS, "animation state")?;
    let mut pet = load_pet(&app)?;
    pet.animation_state = animation;
    save_pet(&app, &pet)?;
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);
    Ok(pet)
}

#[tauri::command]
pub fn pet_interact(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    action: String,
) -> Result<PetState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let mut pet = load_pet(&app)?;

    match action.as_str() {
        "pet" => {
            pet.affection = clamp_metric(pet.affection as i32 + 8);
            pet.energy = clamp_metric(pet.energy as i32 - 1);
            pet.animation_state = "clicked".to_string();
        }
        "feed" => {
            pet.hunger = clamp_metric(pet.hunger as i32 - 25);
            pet.affection = clamp_metric(pet.affection as i32 + 4);
            pet.animation_state = "celebrating".to_string();
        }
        "play" => {
            pet.affection = clamp_metric(pet.affection as i32 + 10);
            pet.energy = clamp_metric(pet.energy as i32 - 8);
            pet.cleanliness = clamp_metric(pet.cleanliness as i32 - 4);
            pet.animation_state = "celebrating".to_string();
        }
        "nap" => {
            pet.energy = clamp_metric(pet.energy as i32 + 20);
            pet.hunger = clamp_metric(pet.hunger as i32 + 8);
            pet.animation_state = "break".to_string();
        }
        "clean" => {
            pet.cleanliness = clamp_metric(pet.cleanliness as i32 + 22);
            pet.affection = clamp_metric(pet.affection as i32 + 3);
            pet.animation_state = "idle".to_string();
        }
        "train" => {
            pet.energy = clamp_metric(pet.energy as i32 - 12);
            pet.hunger = clamp_metric(pet.hunger as i32 + 12);
            pet.affection = clamp_metric(pet.affection as i32 + 6);
            pet.animation_state = "working".to_string();
        }
        _ => return Err("Unknown pet interaction".to_string()),
    }

    pet.last_interaction = Some(action.clone());
    pet.last_care_update_at = chrono::Utc::now().to_rfc3339();
    pet = apply_care_decay(pet);

    let event_message = format!(
        "Action '{}' -> mood {}, energy {}%, hunger {}%, affection {}%",
        action, pet.mood, pet.energy, pet.hunger, pet.affection
    );
    let _ = append_event(&app, "interaction", event_message, true);

    save_pet(&app, &pet)?;
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);
    let _ = advance_care_quest(&app, 1);
    Ok(pet)
}

#[tauri::command]
pub fn set_pet_customization(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    skin: Option<String>,
    scene: Option<String>,
) -> Result<PetState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let mut pet = load_pet(&app)?;
    if let Some(skin) = skin {
        pet.skin = validate_variant(skin, ALLOWED_SKINS, "pet skin")?;
    }
    if let Some(scene) = scene {
        pet.scene = validate_variant(scene, ALLOWED_SCENES, "pet scene")?;
    }
    save_pet(&app, &pet)?;
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);
    Ok(pet)
}

#[tauri::command]
pub fn set_pet_species(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    species_id: String,
    evolution_thresholds: Option<Vec<u32>>,
) -> Result<PetState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let mut pet = load_pet(&app)?;
    pet.species_id = normalize_species_id(species_id)?;
    pet.evolution_thresholds = normalize_evolution_thresholds(evolution_thresholds);
    pet.current_stage = stage_for_total_pomodoros(pet.total_pomodoros, &pet.evolution_thresholds);
    pet.last_interaction = Some("species_switch".to_string());
    save_pet(&app, &pet)?;
    let _ = append_event(
        &app,
        "species",
        format!("Switched species to '{}'.", pet.species_id),
        true,
    );
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);
    Ok(pet)
}

#[tauri::command]
pub fn get_pet_events(app: AppHandle) -> Result<Vec<PetEvent>, String> {
    load_events(&app)
}

#[tauri::command]
pub fn get_pet_active_quest(app: AppHandle) -> Result<Option<PetQuest>, String> {
    load_active_quest(&app)
}

#[tauri::command]
pub fn resolve_pet_event(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    event_id: String,
) -> Result<Vec<PetEvent>, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let mut events = load_events(&app)?;
    if let Some(event) = events.iter_mut().find(|item| item.id == event_id) {
        event.resolved = true;
    }
    save_bounded_events(&app, events)
}

pub fn advance_focus_quest(
    app: &AppHandle,
    completed_sessions: u32,
) -> Result<Option<PetEvent>, String> {
    progress_active_quest(app, "focus_sessions", completed_sessions)
}

pub fn advance_care_quest(
    app: &AppHandle,
    completed_actions: u32,
) -> Result<Option<PetEvent>, String> {
    progress_active_quest(app, "care_actions", completed_actions)
}

fn progress_active_quest(
    app: &AppHandle,
    supported_kind: &str,
    delta: u32,
) -> Result<Option<PetEvent>, String> {
    let mut quest = match load_active_quest(app)? {
        Some(quest) => quest,
        None => return Ok(None),
    };

    if quest.kind != supported_kind {
        return Ok(None);
    }

    quest.completed_sessions = quest.completed_sessions.saturating_add(delta);

    if quest.completed_sessions < quest.target_sessions {
        save_active_quest(app, Some(&quest))?;
        let _ = append_event(
            app,
            "quest_progress",
            format!(
                "Quest progress: {} ({}/{})",
                quest.title, quest.completed_sessions, quest.target_sessions
            ),
            true,
        );
        return Ok(None);
    }

    save_active_quest(app, None)?;

    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let mut coins: CoinBalance = store
        .get("coins")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    coins.total += quest.reward_coins;
    store.set("coins", json!(coins));
    let _ = app.emit(EVENT_COINS_CHANGED, &coins);

    let mut pet = load_pet(app)?;
    pet.affection = clamp_metric(pet.affection as i32 + 12);
    pet.energy = clamp_metric(pet.energy as i32 + 4);
    pet.mood = "proud".to_string();
    pet.animation_state = "celebrating".to_string();
    pet.last_interaction = Some("quest_complete".to_string());
    save_pet(app, &pet)?;
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);

    let mut events = load_events(app)?;
    for event in events.iter_mut() {
        if event.kind == "quest" && !event.resolved {
            event.resolved = true;
        }
    }

    let completion = PetEvent {
        id: uuid::Uuid::new_v4().to_string(),
        kind: "quest_complete".to_string(),
        description: format!(
            "Quest complete: {} (+{} coins)",
            quest.title, quest.reward_coins
        ),
        created_at: chrono::Utc::now().to_rfc3339(),
        resolved: false,
    };
    events.push(completion.clone());
    let _ = save_bounded_events(app, events)?;
    Ok(Some(completion))
}

#[tauri::command]
pub fn roll_pet_event(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
) -> Result<PetEvent, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let pet = load_pet(&app)?;
    let active_quest = load_active_quest(&app)?;
    let event = if pet.hunger > 70 {
        PetEvent {
            id: uuid::Uuid::new_v4().to_string(),
            kind: "need".to_string(),
            description: "Your pet is hungry and asks for a snack.".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved: false,
        }
    } else if pet.energy < 30 {
        PetEvent {
            id: uuid::Uuid::new_v4().to_string(),
            kind: "rest".to_string(),
            description: "Your pet yawns and wants a quick nap.".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved: false,
        }
    } else if pet.affection < 40 {
        PetEvent {
            id: uuid::Uuid::new_v4().to_string(),
            kind: "bond".to_string(),
            description: "Your pet nudges you for attention.".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved: false,
        }
    } else if let Some(quest) = active_quest {
        PetEvent {
            id: uuid::Uuid::new_v4().to_string(),
            kind: "quest".to_string(),
            description: format!(
                "Active quest: {} ({}/{}) [{}]",
                quest.title, quest.completed_sessions, quest.target_sessions, quest.kind
            ),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved: false,
        }
    } else {
        let quest = create_quest_for_pet(&pet);
        save_active_quest(&app, Some(&quest))?;
        PetEvent {
            id: uuid::Uuid::new_v4().to_string(),
            kind: "quest".to_string(),
            description: format!(
                "Quest started: {} (0/{}) [{}] for +{} coins.",
                quest.title, quest.target_sessions, quest.kind, quest.reward_coins
            ),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved: false,
        }
    };

    let mut events = load_events(&app)?;
    events.push(event.clone());
    let _ = save_bounded_events(&app, events)?;
    Ok(event)
}

#[cfg(test)]
mod tests {
    use super::{
        normalize_evolution_thresholds, normalize_species_id, quest_reward_for_stage,
        quest_target_for_stage, validate_variant, ALLOWED_ANIMATIONS,
    };

    #[test]
    fn quest_target_scales_by_stage() {
        assert_eq!(quest_target_for_stage(0), 1);
        assert_eq!(quest_target_for_stage(1), 2);
        assert_eq!(quest_target_for_stage(2), 3);
        assert_eq!(quest_target_for_stage(4), 3);
    }

    #[test]
    fn quest_reward_scales_by_stage() {
        assert_eq!(quest_reward_for_stage(0), 12);
        assert_eq!(quest_reward_for_stage(1), 16);
        assert_eq!(quest_reward_for_stage(2), 20);
    }

    #[test]
    fn validate_variant_rejects_unknown() {
        let err =
            validate_variant("unknown".to_string(), ALLOWED_ANIMATIONS, "animation").unwrap_err();
        assert!(err.contains("Invalid animation"));
    }

    #[test]
    fn normalize_species_id_accepts_slug() {
        let normalized = normalize_species_id(" Axolotl-Blue ".to_string()).unwrap();
        assert_eq!(normalized, "axolotl-blue");
    }

    #[test]
    fn normalize_evolution_thresholds_repairs_invalid_input() {
        assert_eq!(normalize_evolution_thresholds(Some(vec![0, 0, 0])), vec![0, 1, 2]);
        assert_eq!(normalize_evolution_thresholds(Some(vec![0, 8, 4])), vec![0, 8, 9]);
    }
}
