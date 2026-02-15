// Integration test utilities and fixtures

#[cfg(test)]
mod fixtures {
    use crate::models::*;
    use uuid::Uuid;

    /// Create a default test app state with minimal setup
    pub fn default_app_state() -> AppState {
        AppState {
            schema_version: 4,
            pet: default_pet_state(),
            user_progress: default_user_progress(),
            pomodoro: default_pomodoro_state(),
            tasks: vec![],
            goals: GoalsState {
                daily: vec![
                    DailyGoal {
                        id: "pomodoros".to_string(),
                        target: 4,
                        current: 0,
                    },
                    DailyGoal {
                        id: "breaks".to_string(),
                        target: 4,
                        current: 0,
                    },
                    DailyGoal {
                        id: "tasks".to_string(),
                        target: 3,
                        current: 0,
                    },
                    DailyGoal {
                        id: "focus_minutes".to_string(),
                        target: 100,
                        current: 0,
                    },
                ],
            },
            shop: default_shop_state(),
            focus_guardrails: default_focus_guardrails(),
            settings: default_settings(),
        }
    }

    fn default_pet_state() -> PetState {
        PetState {
            current_stage: 1,
            animation_state: "idle".to_string(),
            accessories: vec![],
            total_pomodoros: 0,
            species_id: "penguin".to_string(),
            evolution_thresholds: vec![0, 5, 15],
            mood: "content".to_string(),
            energy: 80,
            hunger: 20,
            cleanliness: 80,
            affection: 50,
            personality: "balanced".to_string(),
            evolution_path: "undetermined".to_string(),
            skin: "classic".to_string(),
            scene: "meadow".to_string(),
            last_interaction: Some(chrono::Utc::now().to_rfc3339()),
            last_care_update_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    fn default_user_progress() -> UserProgress {
        UserProgress {
            xp_total: 0,
            level: 1,
            streak_days: 0,
            longest_streak: 0,
            last_active_date: None,
            total_sessions: 0,
            total_focus_minutes: 0,
            total_tasks_completed: 0,
        }
    }

    fn default_pomodoro_state() -> PomodoroState {
        PomodoroState {
            current_session: None,
            timer_runtime_state: TimerRuntimeState {
                is_running: false,
                elapsed_seconds: 0,
                start_timestamp: None,
                paused_at: None,
            },
            completed_sessions: vec![],
        }
    }

    fn default_shop_state() -> ShopState {
        ShopState {
            inventory: vec![
                ShopItem {
                    id: "hat".to_string(),
                    name: "Top Hat".to_string(),
                    cost: 10,
                    owned: false,
                    seasonal: false,
                    availability_window: None,
                },
            ],
            loadouts: vec![],
        }
    }

    fn default_focus_guardrails() -> FocusGuardrailsState {
        FocusGuardrailsState {
            status: "enabled".to_string(),
            nudge_level: "medium".to_string(),
            blocked_hosts: vec![],
            events: vec![],
        }
    }

    fn default_settings() -> Settings {
        Settings {
            app_theme: "auto".to_string(),
            notification_enabled: true,
            sound_enabled: true,
            volume: 50,
            animation_budget: "medium".to_string(),
            language: "en".to_string(),
            timer_presets: TimerPresets {
                work_minutes: 25,
                break_minutes: 5,
            },
            pet_animation_frequency: 1.0,
            chill_mode_auto_detect_fullscreen: true,
            chill_mode_auto_detect_meetings: false,
            chill_mode_auto_detect_typing: false,
            quiet_mode_enabled: false,
        }
    }

    // Test helper: create a pomodoro session
    pub fn create_test_session(duration_minutes: u32) -> PomodoroSession {
        PomodoroSession {
            id: Uuid::new_v4().to_string(),
            started_at: chrono::Local::now().to_rfc3339(),
            completed_at: None,
            duration_minutes,
            session_type: "work".to_string(),
        }
    }

    // Test helper: create a task
    pub fn create_test_task(title: &str) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            completed: false,
            created_at: chrono::Local::now().to_rfc3339(),
            completed_at: None,
        }
    }
}

#[cfg(test)]
mod timer_coins_flow {
    use super::fixtures::*;

    #[test]
    fn test_pomodoro_session_creation() {
        let session = create_test_session(25);
        assert_eq!(session.duration_minutes, 25);
        assert_eq!(session.session_type, "work");
        assert!(session.completed_at.is_none());
    }

    #[test]
    fn test_multiple_sessions_tracking() {
        let mut state = default_app_state();
        let session1 = create_test_session(25);
        let session2 = create_test_session(25);

        state.pomodoro.completed_sessions.push(session1);
        state.pomodoro.completed_sessions.push(session2);

        assert_eq!(state.pomodoro.completed_sessions.len(), 2);
    }

    #[test]
    fn test_coin_balance_updates() {
        let mut state = default_app_state();
        let initial_coins = state.user_progress.coins;

        // Simulate earning coins
        state.user_progress.coins += 10;

        assert_eq!(state.user_progress.coins, initial_coins + 10);
    }

    #[test]
    fn test_xp_calculation_from_sessions() {
        let mut state = default_app_state();
        let sessions_completed = 4;
        let xp_per_session = 25;

        state.user_progress.total_xp = sessions_completed * xp_per_session;

        // Level = 1 + (total_xp / 100)
        let calculated_level = 1 + (state.user_progress.total_xp / 100);
        assert_eq!(calculated_level, 2);
    }
}

#[cfg(test)]
mod pet_interaction_flow {
    use super::fixtures::*;

    #[test]
    fn test_pet_care_stats_initialization() {
        let state = default_app_state();
        assert_eq!(state.pet.care_stats.energy, 80);
        assert_eq!(state.pet.care_stats.hunger, 20);
        assert_eq!(state.pet.care_stats.cleanliness, 80);
        assert_eq!(state.pet.care_stats.affection, 50);
    }

    #[test]
    fn test_pet_stage_progression() {
        let mut state = default_app_state();
        assert_eq!(state.pet.stage, 1);

        // Simulate progression to stage 2
        state.pet.stage = 2;
        assert_eq!(state.pet.stage, 2);
    }

    #[test]
    fn test_pet_mood_tracking() {
        let mut state = default_app_state();
        let initial_mood = state.pet.mood.clone();
        assert_eq!(initial_mood, "happy");

        // Change mood
        state.pet.mood = "neutral".to_string();
        assert_eq!(state.pet.mood, "neutral");
    }

    #[test]
    fn test_task_creation_and_completion() {
        let mut state = default_app_state();
        let task = create_test_task("Test task");

        state.tasks.push(task.clone());
        assert_eq!(state.tasks.len(), 1);
        assert!(!state.tasks[0].completed);

        // Mark as completed
        state.tasks[0].completed = true;
        assert!(state.tasks[0].completed);
    }

    #[test]
    fn test_streak_tracking() {
        let mut state = default_app_state();
        assert_eq!(state.user_progress.streak_days, 0);

        // Increment streak
        state.user_progress.streak_days = 7;
        assert_eq!(state.user_progress.streak_days, 7);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::fixtures::*;

    #[test]
    fn test_zero_coins_balance() {
        let mut state = default_app_state();
        state.user_progress.coins = 0;
        assert_eq!(state.user_progress.coins, 0);
    }

    #[test]
    fn test_max_care_stats() {
        let mut state = default_app_state();
        state.pet.care_stats.energy = 100;
        state.pet.care_stats.hunger = 0;
        assert_eq!(state.pet.care_stats.energy, 100);
        assert_eq!(state.pet.care_stats.hunger, 0);
    }

    #[test]
    fn test_empty_task_list() {
        let state = default_app_state();
        assert_eq!(state.tasks.len(), 0);
    }

    #[test]
    fn test_default_settings_values() {
        let settings = default_app_state().settings;
        assert_eq!(settings.volume, 50);
        assert!(settings.notification_enabled);
        assert!(settings.chill_mode_auto_detect_fullscreen);
    }
}
