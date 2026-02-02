use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::models::{CoinBalance, PetState, PomodoroSession};

const COINS_PER_POMODORO: u32 = 10;
const STAGE_1_THRESHOLD: u32 = 5;
const STAGE_2_THRESHOLD: u32 = 15;

#[tauri::command]
pub fn start_pomodoro(
    app: AppHandle,
    work_duration: u32,
    break_duration: u32,
) -> Result<PomodoroSession, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let session = PomodoroSession {
        id: uuid::Uuid::new_v4().to_string(),
        started_at: chrono::Utc::now().to_rfc3339(),
        completed_at: None,
        work_duration,
        break_duration,
    };

    let mut sessions: Vec<PomodoroSession> = store
        .get("sessions")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    sessions.push(session.clone());
    store.set("sessions", json!(sessions));

    let mut pet: PetState = store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    pet.animation_state = "working".to_string();
    store.set("pet", json!(pet));
    let _ = app.emit("pet-state-changed", &pet);

    Ok(session)
}

#[tauri::command]
pub fn complete_pomodoro(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    session_id: String,
) -> Result<PetState, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    // Mark session complete
    let mut sessions: Vec<PomodoroSession> = store
        .get("sessions")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    if let Some(s) = sessions.iter_mut().find(|s| s.id == session_id) {
        s.completed_at = Some(chrono::Utc::now().to_rfc3339());
    }
    store.set("sessions", json!(sessions));

    // Award coins
    let mut coins: CoinBalance = store
        .get("coins")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    coins.total += COINS_PER_POMODORO;
    store.set("coins", json!(coins));
    let _ = app.emit("coins-changed", &coins);

    // Update pet: increment pomodoros, check evolution
    let mut pet: PetState = store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    pet.total_pomodoros += 1;
    pet.animation_state = "celebrating".to_string();

    let new_stage = if pet.total_pomodoros >= STAGE_2_THRESHOLD {
        2
    } else if pet.total_pomodoros >= STAGE_1_THRESHOLD {
        1
    } else {
        0
    };

    if new_stage > pet.current_stage {
        pet.current_stage = new_stage;
        pet.animation_state = "evolving".to_string();
    }

    store.set("pet", json!(pet));
    let _ = app.emit("pet-state-changed", &pet);

    // Update daily goal for pomodoros
    let _ = crate::commands::goals::increment_goal_progress(&app, "pomodoros");

    Ok(pet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evolution_thresholds_ordering() {
        assert!(STAGE_1_THRESHOLD < STAGE_2_THRESHOLD);
    }

    #[test]
    fn evolution_stage1_at_5() {
        assert_eq!(STAGE_1_THRESHOLD, 5);
    }

    #[test]
    fn evolution_stage2_at_15() {
        assert_eq!(STAGE_2_THRESHOLD, 15);
    }

    #[test]
    fn coins_per_pomodoro_is_10() {
        assert_eq!(COINS_PER_POMODORO, 10);
    }

    #[test]
    fn evolution_logic_stage0_to_stage1() {
        let total_pomodoros = STAGE_1_THRESHOLD;
        let new_stage = if total_pomodoros >= STAGE_2_THRESHOLD {
            2
        } else if total_pomodoros >= STAGE_1_THRESHOLD {
            1
        } else {
            0
        };
        assert_eq!(new_stage, 1);
    }

    #[test]
    fn evolution_logic_stage1_to_stage2() {
        let total_pomodoros = STAGE_2_THRESHOLD;
        let new_stage = if total_pomodoros >= STAGE_2_THRESHOLD {
            2
        } else if total_pomodoros >= STAGE_1_THRESHOLD {
            1
        } else {
            0
        };
        assert_eq!(new_stage, 2);
    }

    #[test]
    fn evolution_logic_below_stage1() {
        let total_pomodoros = STAGE_1_THRESHOLD - 1;
        let new_stage = if total_pomodoros >= STAGE_2_THRESHOLD {
            2
        } else if total_pomodoros >= STAGE_1_THRESHOLD {
            1
        } else {
            0
        };
        assert_eq!(new_stage, 0);
    }

    #[test]
    fn evolution_logic_between_stages() {
        let total_pomodoros = STAGE_1_THRESHOLD + 3;
        let new_stage = if total_pomodoros >= STAGE_2_THRESHOLD {
            2
        } else if total_pomodoros >= STAGE_1_THRESHOLD {
            1
        } else {
            0
        };
        assert_eq!(new_stage, 1);
    }

    #[test]
    fn evolution_logic_above_stage2() {
        let total_pomodoros = STAGE_2_THRESHOLD + 100;
        let new_stage = if total_pomodoros >= STAGE_2_THRESHOLD {
            2
        } else if total_pomodoros >= STAGE_1_THRESHOLD {
            1
        } else {
            0
        };
        assert_eq!(new_stage, 2);
    }
}
