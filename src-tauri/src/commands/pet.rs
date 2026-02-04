use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::events::{EVENT_COINS_CHANGED, EVENT_PET_STATE_CHANGED};
use crate::models::{CoinBalance, PetEvent, PetQuest, PetState};

const MAX_PET_EVENTS: usize = 30;

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

fn save_bounded_events(app: &AppHandle, mut events: Vec<PetEvent>) -> Result<Vec<PetEvent>, String> {
    events.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    if events.len() > MAX_PET_EVENTS {
        events.truncate(MAX_PET_EVENTS);
    }
    save_events(app, &events)?;
    Ok(events)
}

fn append_event(app: &AppHandle, kind: &str, description: String, resolved: bool) -> Result<(), String> {
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
pub fn get_pet_state(app: AppHandle) -> Result<PetState, String> {
    let pet = load_pet(&app)?;
    save_pet(&app, &pet)?;
    Ok(pet)
}

#[tauri::command]
pub fn set_pet_animation(app: AppHandle, animation: String) -> Result<PetState, String> {
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
        pet.skin = skin;
    }
    if let Some(scene) = scene {
        pet.scene = scene;
    }
    save_pet(&app, &pet)?;
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

pub fn advance_focus_quest(app: &AppHandle, completed_sessions: u32) -> Result<Option<PetEvent>, String> {
    let mut quest = match load_active_quest(app)? {
        Some(quest) => quest,
        None => return Ok(None),
    };

    quest.completed_sessions = quest
        .completed_sessions
        .saturating_add(completed_sessions);

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
            id: quest.id,
            kind: "quest".to_string(),
            description: format!(
                "Active quest: {} ({}/{})",
                quest.title, quest.completed_sessions, quest.target_sessions
            ),
            created_at: chrono::Utc::now().to_rfc3339(),
            resolved: false,
        }
    } else {
        let quest = PetQuest {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Focus Sprint".to_string(),
            description: "Complete focus sessions with your pet to earn bonus coins.".to_string(),
            target_sessions: quest_target_for_stage(pet.current_stage),
            completed_sessions: 0,
            reward_coins: quest_reward_for_stage(pet.current_stage),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        save_active_quest(&app, Some(&quest))?;
        PetEvent {
            id: quest.id,
            kind: "quest".to_string(),
            description: format!(
                "Quest started: {} (0/{}) for +{} coins.",
                quest.title, quest.target_sessions, quest.reward_coins
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
    use super::{quest_reward_for_stage, quest_target_for_stage};

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
}
