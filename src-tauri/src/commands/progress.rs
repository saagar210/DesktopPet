use tauri::AppHandle;

use crate::models::{DailySummary, UserProgress};

#[tauri::command]
pub fn get_user_progress(app: AppHandle) -> Result<UserProgress, String> {
    crate::progression::get_user_progress(&app)
}

#[tauri::command]
pub fn get_daily_summaries(app: AppHandle, days: Option<u32>) -> Result<Vec<DailySummary>, String> {
    crate::progression::get_daily_summaries(&app, days.unwrap_or(14).max(1).min(90))
}
