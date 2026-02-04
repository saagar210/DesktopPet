use serde::{Deserialize, Serialize};

pub const CURRENT_SCHEMA_VERSION: u32 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct PetState {
    pub current_stage: u32,
    pub animation_state: String,
    pub accessories: Vec<String>,
    pub total_pomodoros: u32,
    pub mood: String,
    pub energy: u32,
    pub hunger: u32,
    pub cleanliness: u32,
    pub affection: u32,
    pub personality: String,
    pub evolution_path: String,
    pub skin: String,
    pub scene: String,
    pub last_interaction: Option<String>,
    pub last_care_update_at: String,
}

impl Default for PetState {
    fn default() -> Self {
        Self {
            current_stage: 0,
            animation_state: "idle".to_string(),
            accessories: vec![],
            total_pomodoros: 0,
            mood: "content".to_string(),
            energy: 80,
            hunger: 20,
            cleanliness: 80,
            affection: 50,
            personality: "balanced".to_string(),
            evolution_path: "undetermined".to_string(),
            skin: "classic".to_string(),
            scene: "meadow".to_string(),
            last_interaction: None,
            last_care_update_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetEvent {
    pub id: String,
    pub kind: String,
    pub description: String,
    pub created_at: String,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PetQuest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_sessions: u32,
    pub completed_sessions: u32,
    pub reward_coins: u32,
    pub created_at: String,
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
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub timer_preset: String,
    pub notifications_enabled: bool,
    pub sounds_enabled: bool,
    pub sound_volume: f32,
    pub ui_theme: String,
    pub pet_skin: String,
    pub pet_scene: String,
    pub focus_guardrails_enabled: bool,
    pub focus_guardrails_work_only: bool,
    pub focus_allowlist: Vec<String>,
    pub focus_blocklist: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            timer_preset: "standard".to_string(),
            notifications_enabled: true,
            sounds_enabled: false,
            sound_volume: 0.7,
            ui_theme: "sunrise".to_string(),
            pet_skin: "classic".to_string(),
            pet_scene: "meadow".to_string(),
            focus_guardrails_enabled: false,
            focus_guardrails_work_only: true,
            focus_allowlist: vec![],
            focus_blocklist: vec![],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsPatch {
    pub timer_preset: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub sounds_enabled: Option<bool>,
    pub sound_volume: Option<f32>,
    pub ui_theme: Option<String>,
    pub pet_skin: Option<String>,
    pub pet_scene: Option<String>,
    pub focus_guardrails_enabled: Option<bool>,
    pub focus_guardrails_work_only: Option<bool>,
    pub focus_allowlist: Option<Vec<String>>,
    pub focus_blocklist: Option<Vec<String>>,
}

impl SettingsPatch {
    pub fn apply_to(self, settings: &mut Settings) {
        if let Some(timer_preset) = self.timer_preset {
            settings.timer_preset = timer_preset;
        }
        if let Some(notifications_enabled) = self.notifications_enabled {
            settings.notifications_enabled = notifications_enabled;
        }
        if let Some(sounds_enabled) = self.sounds_enabled {
            settings.sounds_enabled = sounds_enabled;
        }
        if let Some(sound_volume) = self.sound_volume {
            settings.sound_volume = sound_volume.clamp(0.0, 1.0);
        }
        if let Some(ui_theme) = self.ui_theme {
            settings.ui_theme = ui_theme;
        }
        if let Some(pet_skin) = self.pet_skin {
            settings.pet_skin = pet_skin;
        }
        if let Some(pet_scene) = self.pet_scene {
            settings.pet_scene = pet_scene;
        }
        if let Some(enabled) = self.focus_guardrails_enabled {
            settings.focus_guardrails_enabled = enabled;
        }
        if let Some(work_only) = self.focus_guardrails_work_only {
            settings.focus_guardrails_work_only = work_only;
        }
        if let Some(allowlist) = self.focus_allowlist {
            settings.focus_allowlist = allowlist;
        }
        if let Some(blocklist) = self.focus_blocklist {
            settings.focus_blocklist = blocklist;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerRuntimeState {
    pub phase: String,
    pub seconds_left: u32,
    pub total_seconds: u32,
    pub paused: bool,
    pub session_id: Option<String>,
    pub sessions_completed: u32,
    pub preset: String,
    pub last_updated_at: String,
}

impl Default for TimerRuntimeState {
    fn default() -> Self {
        Self {
            phase: "idle".to_string(),
            seconds_left: 25 * 60,
            total_seconds: 25 * 60,
            paused: false,
            session_id: None,
            sessions_completed: 0,
            preset: "standard".to_string(),
            last_updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProgress {
    pub xp_total: u32,
    pub level: u32,
    pub streak_days: u32,
    pub longest_streak: u32,
    pub last_active_date: Option<String>,
    pub total_sessions: u32,
    pub total_focus_minutes: u32,
    pub total_tasks_completed: u32,
}

impl Default for UserProgress {
    fn default() -> Self {
        Self {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct DailySummary {
    pub date: String,
    pub sessions_completed: u32,
    pub focus_minutes: u32,
    pub tasks_completed: u32,
    pub goals_completed: u32,
    pub coins_earned: u32,
    pub xp_earned: u32,
    pub guardrails_interventions: u32,
    pub high_nudges: u32,
}

impl Default for DailySummary {
    fn default() -> Self {
        Self {
            date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            sessions_completed: 0,
            focus_minutes: 0,
            tasks_completed: 0,
            goals_completed: 0,
            coins_earned: 0,
            xp_earned: 0,
            guardrails_interventions: 0,
            high_nudges: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomizationLoadout {
    pub name: String,
    pub ui_theme: String,
    pub pet_skin: String,
    pub pet_scene: String,
    pub accessories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusGuardrailsStatus {
    pub enabled: bool,
    pub active: bool,
    pub phase: String,
    pub matched_blocklist: Vec<String>,
    pub matched_allowlist: Vec<String>,
    pub blocked_hosts_count: u32,
    pub nudge_level: String,
    pub recommended_action: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusGuardrailEvent {
    pub id: String,
    pub phase: String,
    pub hosts: Vec<String>,
    pub matched_blocklist: Vec<String>,
    pub nudge_level: String,
    pub recommended_action: String,
    pub created_at: String,
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
        assert_eq!(p.mood, "content");
        assert_eq!(p.scene, "meadow");
    }

    #[test]
    fn pet_state_serializes_camel_case() {
        let p = PetState::default();
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("currentStage").is_some());
        assert!(json.get("animationState").is_some());
        assert!(json.get("totalPomodoros").is_some());
        assert!(json.get("mood").is_some());
        assert!(json.get("evolutionPath").is_some());
        assert!(json.get("lastInteraction").is_some());
        assert!(json.get("lastCareUpdateAt").is_some());
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
            mood: "focused".to_string(),
            energy: 70,
            hunger: 30,
            cleanliness: 75,
            affection: 62,
            personality: "balanced".to_string(),
            evolution_path: "companion".to_string(),
            skin: "classic".to_string(),
            scene: "meadow".to_string(),
            last_interaction: Some("pet".to_string()),
            last_care_update_at: "2025-01-01T00:00:00Z".to_string(),
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
            mood: "happy".to_string(),
            energy: 92,
            hunger: 18,
            cleanliness: 88,
            affection: 95,
            personality: "playful".to_string(),
            evolution_path: "scholar".to_string(),
            skin: "pixel".to_string(),
            scene: "space".to_string(),
            last_interaction: Some("train".to_string()),
            last_care_update_at: "2025-06-01T00:00:00Z".to_string(),
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: PetState = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.current_stage, 2);
        assert_eq!(restored.animation_state, "celebrating");
        assert_eq!(restored.accessories, vec!["sunglasses"]);
        assert_eq!(restored.total_pomodoros, 20);
        assert_eq!(restored.scene, "space");
        assert_eq!(restored.evolution_path, "scholar");
    }

    #[test]
    fn pet_state_deserialize_from_camel_case_json() {
        let json = r#"{"currentStage":1,"animationState":"idle","accessories":[],"totalPomodoros":5,"mood":"content","energy":80,"hunger":20,"cleanliness":80,"affection":50,"personality":"balanced","evolutionPath":"undetermined","skin":"classic","scene":"meadow","lastInteraction":null,"lastCareUpdateAt":"2025-01-01T00:00:00Z"}"#;
        let pet: PetState = serde_json::from_str(json).unwrap();
        assert_eq!(pet.current_stage, 1);
        assert_eq!(pet.total_pomodoros, 5);
    }

    #[test]
    fn pet_state_deserialize_legacy_payload_uses_defaults() {
        let json = r#"{"currentStage":1,"animationState":"idle","accessories":[],"totalPomodoros":5}"#;
        let pet: PetState = serde_json::from_str(json).unwrap();
        assert_eq!(pet.current_stage, 1);
        assert_eq!(pet.mood, "content");
        assert_eq!(pet.skin, "classic");
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
        assert!(s.notifications_enabled);
        assert!(!s.sounds_enabled);
        assert!((s.sound_volume - 0.7).abs() < f32::EPSILON);
        assert_eq!(s.ui_theme, "sunrise");
        assert_eq!(s.pet_skin, "classic");
        assert_eq!(s.pet_scene, "meadow");
    }

    #[test]
    fn settings_serializes_camel_case() {
        let s = Settings::default();
        let json = serde_json::to_value(&s).unwrap();
        assert!(json.get("timerPreset").is_some());
        assert!(json.get("notificationsEnabled").is_some());
        assert!(json.get("soundsEnabled").is_some());
        assert!(json.get("soundVolume").is_some());
        assert!(json.get("uiTheme").is_some());
        assert!(json.get("petSkin").is_some());
        assert!(json.get("petScene").is_some());
        assert!(json.get("focusGuardrailsEnabled").is_some());
        assert!(json.get("focusAllowlist").is_some());
        assert!(json.get("timer_preset").is_none());
    }

    #[test]
    fn settings_roundtrip() {
        let original = Settings {
            timer_preset: "long".to_string(),
            notifications_enabled: false,
            sounds_enabled: true,
            sound_volume: 0.4,
            ui_theme: "dusk".to_string(),
            pet_skin: "pixel".to_string(),
            pet_scene: "forest".to_string(),
            focus_guardrails_enabled: true,
            focus_guardrails_work_only: true,
            focus_allowlist: vec!["localhost".to_string()],
            focus_blocklist: vec!["youtube.com".to_string()],
        };
        let json_str = serde_json::to_string(&original).unwrap();
        let restored: Settings = serde_json::from_str(&json_str).unwrap();
        assert_eq!(restored.timer_preset, "long");
        assert!(!restored.notifications_enabled);
        assert!(restored.sounds_enabled);
        assert!((restored.sound_volume - 0.4).abs() < f32::EPSILON);
        assert_eq!(restored.ui_theme, "dusk");
        assert_eq!(restored.pet_skin, "pixel");
        assert_eq!(restored.pet_scene, "forest");
        assert!(restored.focus_guardrails_enabled);
        assert_eq!(restored.focus_allowlist, vec!["localhost"]);
        assert_eq!(restored.focus_blocklist, vec!["youtube.com"]);
    }

    #[test]
    fn settings_patch_applies_and_clamps() {
        let mut settings = Settings::default();
        SettingsPatch {
            timer_preset: Some("short".to_string()),
            notifications_enabled: Some(false),
            sounds_enabled: Some(true),
            sound_volume: Some(2.0),
            ui_theme: Some("dusk".to_string()),
            pet_skin: Some("plush".to_string()),
            pet_scene: Some("forest".to_string()),
            focus_guardrails_enabled: Some(true),
            focus_guardrails_work_only: Some(false),
            focus_allowlist: Some(vec!["localhost".to_string()]),
            focus_blocklist: Some(vec!["youtube.com".to_string()]),
        }
        .apply_to(&mut settings);

        assert_eq!(settings.timer_preset, "short");
        assert!(!settings.notifications_enabled);
        assert!(settings.sounds_enabled);
        assert!((settings.sound_volume - 1.0).abs() < f32::EPSILON);
        assert_eq!(settings.ui_theme, "dusk");
        assert_eq!(settings.pet_skin, "plush");
        assert_eq!(settings.pet_scene, "forest");
        assert!(settings.focus_guardrails_enabled);
        assert!(!settings.focus_guardrails_work_only);
        assert_eq!(settings.focus_allowlist, vec!["localhost"]);
        assert_eq!(settings.focus_blocklist, vec!["youtube.com"]);
    }

    #[test]
    fn settings_deserialize_legacy_payload_uses_defaults() {
        let json = r#"{"timerPreset":"standard"}"#;
        let settings: Settings = serde_json::from_str(json).unwrap();
        assert!(settings.notifications_enabled);
        assert!(!settings.sounds_enabled);
        assert_eq!(settings.ui_theme, "sunrise");
    }

    #[test]
    fn timer_runtime_default_is_idle() {
        let runtime = TimerRuntimeState::default();
        assert_eq!(runtime.phase, "idle");
        assert_eq!(runtime.preset, "standard");
        assert_eq!(runtime.seconds_left, 25 * 60);
        assert_eq!(runtime.total_seconds, 25 * 60);
        assert!(!runtime.paused);
    }

    #[test]
    fn timer_runtime_serializes_camel_case() {
        let runtime = TimerRuntimeState::default();
        let json = serde_json::to_value(&runtime).unwrap();
        assert!(json.get("secondsLeft").is_some());
        assert!(json.get("totalSeconds").is_some());
        assert!(json.get("sessionId").is_some());
        assert!(json.get("lastUpdatedAt").is_some());
        assert!(json.get("seconds_left").is_none());
    }

    #[test]
    fn user_progress_default_values() {
        let progress = UserProgress::default();
        assert_eq!(progress.level, 1);
        assert_eq!(progress.xp_total, 0);
        assert_eq!(progress.streak_days, 0);
        assert_eq!(progress.total_sessions, 0);
    }

    #[test]
    fn user_progress_serializes_camel_case() {
        let progress = UserProgress::default();
        let json = serde_json::to_value(&progress).unwrap();
        assert!(json.get("xpTotal").is_some());
        assert!(json.get("streakDays").is_some());
        assert!(json.get("longestStreak").is_some());
        assert!(json.get("totalFocusMinutes").is_some());
    }

    #[test]
    fn daily_summary_serializes_camel_case() {
        let summary = DailySummary::default();
        let json = serde_json::to_value(&summary).unwrap();
        assert!(json.get("sessionsCompleted").is_some());
        assert!(json.get("focusMinutes").is_some());
        assert!(json.get("tasksCompleted").is_some());
        assert!(json.get("goalsCompleted").is_some());
        assert!(json.get("coinsEarned").is_some());
        assert!(json.get("xpEarned").is_some());
        assert!(json.get("guardrailsInterventions").is_some());
        assert!(json.get("highNudges").is_some());
    }

    #[test]
    fn customization_loadout_serializes_camel_case() {
        let loadout = CustomizationLoadout {
            name: "Focus".to_string(),
            ui_theme: "mint".to_string(),
            pet_skin: "pixel".to_string(),
            pet_scene: "forest".to_string(),
            accessories: vec!["party_hat".to_string()],
        };
        let json = serde_json::to_value(&loadout).unwrap();
        assert!(json.get("uiTheme").is_some());
        assert!(json.get("petSkin").is_some());
        assert!(json.get("petScene").is_some());
    }

    #[test]
    fn pet_event_serializes_camel_case() {
        let event = PetEvent {
            id: "evt-1".to_string(),
            kind: "quest".to_string(),
            description: "Found a shiny token".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
            resolved: false,
        };
        let json = serde_json::to_value(&event).unwrap();
        assert!(json.get("createdAt").is_some());
        assert!(json.get("created_at").is_none());
    }

    #[test]
    fn pet_quest_serializes_camel_case() {
        let quest = PetQuest {
            id: "quest-1".to_string(),
            title: "Focus Sprint".to_string(),
            description: "Complete focused sessions".to_string(),
            target_sessions: 2,
            completed_sessions: 1,
            reward_coins: 16,
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&quest).unwrap();
        assert!(json.get("targetSessions").is_some());
        assert!(json.get("completedSessions").is_some());
        assert!(json.get("rewardCoins").is_some());
    }

    #[test]
    fn focus_guardrails_status_serializes_camel_case() {
        let status = FocusGuardrailsStatus {
            enabled: true,
            active: false,
            phase: "work".to_string(),
            matched_blocklist: vec![],
            matched_allowlist: vec!["localhost".to_string()],
            blocked_hosts_count: 0,
            nudge_level: "none".to_string(),
            recommended_action: "none".to_string(),
            message: "ok".to_string(),
        };
        let json = serde_json::to_value(&status).unwrap();
        assert!(json.get("matchedBlocklist").is_some());
        assert!(json.get("matchedAllowlist").is_some());
    }

    #[test]
    fn focus_guardrail_event_serializes_camel_case() {
        let event = FocusGuardrailEvent {
            id: "guardrail-1".to_string(),
            phase: "work".to_string(),
            hosts: vec!["youtube.com".to_string()],
            matched_blocklist: vec!["youtube.com".to_string()],
            nudge_level: "medium".to_string(),
            recommended_action: "show_nudge".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&event).unwrap();
        assert!(json.get("matchedBlocklist").is_some());
        assert!(json.get("recommendedAction").is_some());
        assert!(json.get("createdAt").is_some());
    }
}
