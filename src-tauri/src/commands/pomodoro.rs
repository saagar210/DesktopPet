use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::events::{EVENT_COINS_CHANGED, EVENT_PET_STATE_CHANGED};
use crate::models::{CoinBalance, PetState, PomodoroSession, UserProgress};

const COINS_PER_POMODORO: u32 = 10;
const STAGE_1_THRESHOLD: u32 = 5;
const STAGE_2_THRESHOLD: u32 = 15;
const MIN_WORK_DURATION_SECS: u32 = 5 * 60;
const MAX_WORK_DURATION_SECS: u32 = 2 * 60 * 60;
const MIN_BREAK_DURATION_SECS: u32 = 60;
const MAX_BREAK_DURATION_SECS: u32 = 30 * 60;

fn clamp_work_duration(seconds: u32) -> u32 {
    seconds.clamp(MIN_WORK_DURATION_SECS, MAX_WORK_DURATION_SECS)
}

fn clamp_break_duration(seconds: u32) -> u32 {
    seconds.clamp(MIN_BREAK_DURATION_SECS, MAX_BREAK_DURATION_SECS)
}

#[tauri::command]
pub fn start_pomodoro(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    work_duration: u32,
    break_duration: u32,
) -> Result<PomodoroSession, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let work_duration = clamp_work_duration(work_duration);
    let break_duration = clamp_break_duration(break_duration);

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
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);

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
    let session = sessions
        .iter_mut()
        .find(|s| s.id == session_id)
        .ok_or_else(|| "Session not found".to_string())?;
    if session.completed_at.is_some() {
        return Err("Session already completed".to_string());
    }
    session.completed_at = Some(chrono::Utc::now().to_rfc3339());
    let completed_work_duration = session.work_duration;
    store.set("sessions", json!(sessions));

    // Award coins
    let mut coins: CoinBalance = store
        .get("coins")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    coins.total += COINS_PER_POMODORO;
    store.set("coins", json!(coins));
    let _ = app.emit(EVENT_COINS_CHANGED, &coins);

    // Update pet: increment pomodoros, check evolution
    let mut pet: PetState = store
        .get("pet")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    pet.total_pomodoros += 1;
    pet.energy = pet.energy.saturating_sub(3);
    pet.hunger = (pet.hunger + 4).min(100);
    pet.affection = (pet.affection + 2).min(100);
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

    let progress: UserProgress = store
        .get("user_progress")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let focus_bias = progress.total_focus_minutes + (completed_work_duration / 60);
    let task_bias = progress.total_tasks_completed * 20;
    pet.evolution_path = if focus_bias > task_bias + 120 {
        "scholar".to_string()
    } else if task_bias > focus_bias {
        "guardian".to_string()
    } else {
        "companion".to_string()
    };
    pet.personality = match pet.evolution_path.as_str() {
        "scholar" => "focused".to_string(),
        "guardian" => "steady".to_string(),
        _ => "playful".to_string(),
    };

    store.set("pet", json!(pet));
    let _ = app.emit(EVENT_PET_STATE_CHANGED, &pet);

    // Update daily goal for pomodoros
    let _ = crate::commands::goals::increment_goal_progress(&app, "pomodoros");
    let _ = crate::commands::goals::add_goal_progress(
        &app,
        "focus_minutes",
        completed_work_duration / 60,
    );
    let _ =
        crate::progression::record_focus_session(&app, completed_work_duration, COINS_PER_POMODORO);
    let _ = crate::commands::pet::advance_focus_quest(&app, 1);

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
    fn work_duration_is_clamped() {
        assert_eq!(clamp_work_duration(60), MIN_WORK_DURATION_SECS);
        assert_eq!(clamp_work_duration(25 * 60), 25 * 60);
        assert_eq!(clamp_work_duration(24 * 60 * 60), MAX_WORK_DURATION_SECS);
    }

    #[test]
    fn break_duration_is_clamped() {
        assert_eq!(clamp_break_duration(0), MIN_BREAK_DURATION_SECS);
        assert_eq!(clamp_break_duration(5 * 60), 5 * 60);
        assert_eq!(clamp_break_duration(3 * 60 * 60), MAX_BREAK_DURATION_SECS);
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
