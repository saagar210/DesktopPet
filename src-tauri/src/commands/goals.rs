use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::events::EVENT_GOALS_CHANGED;
use crate::models::DailyGoal;

fn today() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

fn default_goals(date: &str) -> Vec<DailyGoal> {
    vec![
        DailyGoal {
            id: "pomodoros".to_string(),
            description: "Complete 4 pomodoros".to_string(),
            target: 4,
            progress: 0,
            date: date.to_string(),
        },
        DailyGoal {
            id: "breaks".to_string(),
            description: "Take 3 breaks".to_string(),
            target: 3,
            progress: 0,
            date: date.to_string(),
        },
        DailyGoal {
            id: "tasks".to_string(),
            description: "Complete 2 tasks".to_string(),
            target: 2,
            progress: 0,
            date: date.to_string(),
        },
        DailyGoal {
            id: "focus_minutes".to_string(),
            description: "Focus for 60 minutes".to_string(),
            target: 60,
            progress: 0,
            date: date.to_string(),
        },
    ]
}

fn load_goals(app: &AppHandle) -> Result<Vec<DailyGoal>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let goals: Vec<DailyGoal> = store
        .get("goals")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let date = today();
    if goals.is_empty() || goals[0].date != date {
        let new_goals = default_goals(&date);
        store.set("goals", json!(new_goals));
        Ok(store
            .get("goals")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default())
    } else {
        Ok(goals)
    }
}

#[tauri::command]
pub fn get_daily_goals(app: AppHandle) -> Result<Vec<DailyGoal>, String> {
    load_goals(&app)
}

pub fn increment_goal_progress(app: &AppHandle, goal_id: &str) -> Result<(), String> {
    save_goal_progress_delta(app, goal_id, 1)?;
    Ok(())
}

pub fn add_goal_progress(app: &AppHandle, goal_id: &str, delta: u32) -> Result<(), String> {
    save_goal_progress_delta(app, goal_id, delta)
}

fn save_goal_progress_delta(app: &AppHandle, goal_id: &str, delta: u32) -> Result<(), String> {
    let mut goals = load_goals(app)?;
    let mut newly_completed = false;

    if let Some(goal) = goals.iter_mut().find(|g| g.id == goal_id) {
        let was_complete = goal.progress >= goal.target;
        if !was_complete {
            goal.progress = (goal.progress + delta).min(goal.target);
            let now_complete = goal.progress >= goal.target;
            newly_completed = now_complete && !was_complete;
        }
    }

    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("goals", json!(goals));
    let _ = app.emit(EVENT_GOALS_CHANGED, &goals);

    if newly_completed {
        let _ = crate::progression::record_goal_completion(app);
    }

    Ok(())
}

#[tauri::command]
pub fn update_goal_progress(
    app: AppHandle,
    goal_id: String,
    progress: u32,
) -> Result<Vec<DailyGoal>, String> {
    let mut goals = load_goals(&app)?;
    let mut newly_completed = false;
    if let Some(goal) = goals.iter_mut().find(|g| g.id == goal_id) {
        let was_complete = goal.progress >= goal.target;
        goal.progress = progress.min(goal.target);
        let now_complete = goal.progress >= goal.target;
        newly_completed = now_complete && !was_complete;
    }
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("goals", json!(goals));
    let _ = app.emit(EVENT_GOALS_CHANGED, &goals);
    if newly_completed {
        let _ = crate::progression::record_goal_completion(&app);
    }
    Ok(goals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_goals_have_correct_count() {
        let goals = default_goals("2025-01-01");
        assert_eq!(goals.len(), 4);
        assert_eq!(goals[0].id, "pomodoros");
        assert_eq!(goals[1].id, "breaks");
        assert_eq!(goals[2].id, "tasks");
        assert_eq!(goals[3].id, "focus_minutes");
    }

    #[test]
    fn default_goals_start_at_zero() {
        let goals = default_goals("2025-01-01");
        for goal in &goals {
            assert_eq!(goal.progress, 0);
        }
    }

    #[test]
    fn default_goals_have_positive_targets() {
        let goals = default_goals("2025-06-01");
        for goal in &goals {
            assert!(goal.target > 0, "goal {} should have positive target", goal.id);
        }
    }

    #[test]
    fn default_goals_use_provided_date() {
        let date = "2025-12-25";
        let goals = default_goals(date);
        for goal in &goals {
            assert_eq!(goal.date, date);
        }
    }

    #[test]
    fn default_goals_have_descriptions() {
        let goals = default_goals("2025-01-01");
        for goal in &goals {
            assert!(!goal.description.is_empty(), "goal {} should have description", goal.id);
        }
    }

    #[test]
    fn default_goals_pomodoro_target_is_4() {
        let goals = default_goals("2025-01-01");
        let pomo = goals.iter().find(|g| g.id == "pomodoros").unwrap();
        assert_eq!(pomo.target, 4);
    }

    #[test]
    fn default_goals_breaks_target_is_3() {
        let goals = default_goals("2025-01-01");
        let breaks = goals.iter().find(|g| g.id == "breaks").unwrap();
        assert_eq!(breaks.target, 3);
    }

    #[test]
    fn default_goals_tasks_target_is_2() {
        let goals = default_goals("2025-01-01");
        let tasks = goals.iter().find(|g| g.id == "tasks").unwrap();
        assert_eq!(tasks.target, 2);
    }

    #[test]
    fn default_goals_focus_target_is_60() {
        let goals = default_goals("2025-01-01");
        let focus = goals.iter().find(|g| g.id == "focus_minutes").unwrap();
        assert_eq!(focus.target, 60);
    }

    #[test]
    fn today_returns_valid_date_format() {
        let date = today();
        assert_eq!(date.len(), 10);
        assert!(date.chars().nth(4).unwrap() == '-');
        assert!(date.chars().nth(7).unwrap() == '-');
    }
}
