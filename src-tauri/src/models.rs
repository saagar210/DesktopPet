use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetState {
    pub current_stage: u32,
    pub animation_state: String,
    pub accessories: Vec<String>,
    pub total_pomodoros: u32,
}

impl Default for PetState {
    fn default() -> Self {
        Self {
            current_stage: 0,
            animation_state: "idle".to_string(),
            accessories: vec![],
            total_pomodoros: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroSession {
    pub id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub work_duration: u32,
    pub break_duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    pub total: u32,
    pub spent: u32,
}

impl Default for CoinBalance {
    fn default() -> Self {
        Self { total: 0, spent: 0 }
    }
}

impl CoinBalance {
    pub fn available(&self) -> u32 {
        self.total.saturating_sub(self.spent)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyGoal {
    pub id: String,
    pub description: String,
    pub target: u32,
    pub progress: u32,
    pub date: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub timer_preset: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timer_preset: "standard".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- CoinBalance ---

    #[test]
    fn coin_balance_available() {
        let b = CoinBalance { total: 50, spent: 20 };
        assert_eq!(b.available(), 30);
    }

    #[test]
    fn coin_balance_underflow() {
        let b = CoinBalance { total: 5, spent: 10 };
        assert_eq!(b.available(), 0);
    }

    #[test]
    fn coin_balance_zero() {
        let b = CoinBalance::default();
        assert_eq!(b.available(), 0);
        assert_eq!(b.total, 0);
        assert_eq!(b.spent, 0);
    }

    #[test]
    fn coin_balance_exact() {
        let b = CoinBalance { total: 30, spent: 30 };
        assert_eq!(b.available(), 0);
    }

    #[test]
    fn coin_balance_large_values() {
        let b = CoinBalance { total: 1_000_000, spent: 999_999 };
        assert_eq!(b.available(), 1);
    }

    #[test]
    fn coin_balance_serializes_camel_case() {
        let b = CoinBalance { total: 10, spent: 5 };
        let json = serde_json::to_value(&b).unwrap();
        assert_eq!(json.get("total").unwrap(), 10);
        assert_eq!(json.get("spent").unwrap(), 5);
    }

    #[test]
    fn coin_balance_roundtrip() {
        let original = CoinBalance { total: 42, spent: 7 };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: CoinBalance = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.total, original.total);
        assert_eq!(restored.spent, original.spent);
    }

    // --- PetState ---

    #[test]
    fn pet_default_stage() {
        let p = PetState::default();
        assert_eq!(p.current_stage, 0);
        assert_eq!(p.animation_state, "idle");
        assert!(p.accessories.is_empty());
        assert_eq!(p.total_pomodoros, 0);
    }

    #[test]
    fn pet_state_serializes_camel_case() {
        let p = PetState::default();
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("currentStage").is_some());
        assert!(json.get("animationState").is_some());
        assert!(json.get("totalPomodoros").is_some());
        assert!(json.get("current_stage").is_none());
        assert!(json.get("animation_state").is_none());
        assert!(json.get("total_pomodoros").is_none());
    }

    #[test]
    fn pet_state_with_accessories() {
        let p = PetState {
            current_stage: 1,
            animation_state: "working".to_string(),
            accessories: vec!["party_hat".to_string(), "bow_tie".to_string()],
            total_pomodoros: 7,
        };
        let json = serde_json::to_value(&p).unwrap();
        let acc = json.get("accessories").unwrap().as_array().unwrap();
        assert_eq!(acc.len(), 2);
        assert_eq!(acc[0], "party_hat");
        assert_eq!(acc[1], "bow_tie");
    }

    #[test]
    fn pet_state_roundtrip() {
        let original = PetState {
            current_stage: 2,
            animation_state: "celebrating".to_string(),
            accessories: vec!["sunglasses".to_string()],
            total_pomodoros: 20,
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: PetState = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.current_stage, 2);
        assert_eq!(restored.animation_state, "celebrating");
        assert_eq!(restored.accessories, vec!["sunglasses"]);
        assert_eq!(restored.total_pomodoros, 20);
    }

    #[test]
    fn pet_state_deserialize_from_camel_case_json() {
        let json = r#"{"currentStage":1,"animationState":"idle","accessories":[],"totalPomodoros":5}"#;
        let pet: PetState = serde_json::from_str(json).unwrap();
        assert_eq!(pet.current_stage, 1);
        assert_eq!(pet.total_pomodoros, 5);
    }

    // --- PomodoroSession ---

    #[test]
    fn pomodoro_session_serializes_camel_case() {
        let s = PomodoroSession {
            id: "abc".to_string(),
            started_at: "2025-01-01T00:00:00Z".to_string(),
            completed_at: None,
            work_duration: 1500,
            break_duration: 300,
        };
        let json = serde_json::to_value(&s).unwrap();
        assert!(json.get("startedAt").is_some());
        assert!(json.get("completedAt").is_some());
        assert!(json.get("workDuration").is_some());
        assert!(json.get("breakDuration").is_some());
        assert!(json.get("started_at").is_none());
        assert!(json.get("work_duration").is_none());
    }

    #[test]
    fn pomodoro_session_completed_at_none() {
        let s = PomodoroSession {
            id: "x".to_string(),
            started_at: "2025-01-01T00:00:00Z".to_string(),
            completed_at: None,
            work_duration: 1500,
            break_duration: 300,
        };
        let json = serde_json::to_value(&s).unwrap();
        assert!(json.get("completedAt").unwrap().is_null());
    }

    #[test]
    fn pomodoro_session_completed_at_some() {
        let s = PomodoroSession {
            id: "x".to_string(),
            started_at: "2025-01-01T00:00:00Z".to_string(),
            completed_at: Some("2025-01-01T00:25:00Z".to_string()),
            work_duration: 1500,
            break_duration: 300,
        };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(
            json.get("completedAt").unwrap().as_str().unwrap(),
            "2025-01-01T00:25:00Z"
        );
    }

    #[test]
    fn pomodoro_session_roundtrip() {
        let original = PomodoroSession {
            id: "session-1".to_string(),
            started_at: "2025-06-01T10:00:00Z".to_string(),
            completed_at: Some("2025-06-01T10:25:00Z".to_string()),
            work_duration: 1500,
            break_duration: 300,
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: PomodoroSession = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.completed_at, original.completed_at);
        assert_eq!(restored.work_duration, 1500);
    }

    // --- Task ---

    #[test]
    fn task_serializes_camel_case() {
        let t = Task {
            id: "test".to_string(),
            title: "Test".to_string(),
            completed: false,
            created_at: "2025-01-01".to_string(),
        };
        let json = serde_json::to_value(&t).unwrap();
        assert!(json.get("createdAt").is_some());
        assert!(json.get("created_at").is_none());
    }

    #[test]
    fn task_roundtrip() {
        let original = Task {
            id: "t1".to_string(),
            title: "Write tests".to_string(),
            completed: true,
            created_at: "2025-01-01T12:00:00Z".to_string(),
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: Task = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.id, "t1");
        assert_eq!(restored.title, "Write tests");
        assert!(restored.completed);
    }

    // --- DailyGoal ---

    #[test]
    fn daily_goal_serializes_camel_case() {
        let g = DailyGoal {
            id: "pomodoros".to_string(),
            description: "Complete 4 pomodoros".to_string(),
            target: 4,
            progress: 2,
            date: "2025-01-01".to_string(),
        };
        let json = serde_json::to_value(&g).unwrap();
        // DailyGoal fields are all single words or don't need renaming,
        // but verify the struct serializes correctly
        assert_eq!(json.get("id").unwrap().as_str().unwrap(), "pomodoros");
        assert_eq!(json.get("target").unwrap().as_u64().unwrap(), 4);
        assert_eq!(json.get("progress").unwrap().as_u64().unwrap(), 2);
    }

    #[test]
    fn daily_goal_roundtrip() {
        let original = DailyGoal {
            id: "tasks".to_string(),
            description: "Complete 2 tasks".to_string(),
            target: 2,
            progress: 1,
            date: "2025-06-15".to_string(),
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: DailyGoal = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.progress, 1);
        assert_eq!(restored.target, 2);
        assert_eq!(restored.date, "2025-06-15");
    }

    // --- Settings ---

    #[test]
    fn settings_default() {
        let s = Settings::default();
        assert_eq!(s.timer_preset, "standard");
    }

    #[test]
    fn settings_serializes_camel_case() {
        let s = Settings::default();
        let json = serde_json::to_value(&s).unwrap();
        assert!(json.get("timerPreset").is_some());
        assert!(json.get("timer_preset").is_none());
    }

    #[test]
    fn settings_roundtrip() {
        let original = Settings {
            timer_preset: "long".to_string(),
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: Settings = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.timer_preset, "long");
    }
}
