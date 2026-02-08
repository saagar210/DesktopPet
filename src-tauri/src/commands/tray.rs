use tauri::AppHandle;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrayBadgeResult {
    pub used_title: bool,
    pub used_tooltip: bool,
}

#[tauri::command]
pub fn set_tray_badge(app: AppHandle, count: Option<u32>) -> Result<TrayBadgeResult, String> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| "Tray icon not found".to_string())?;

    let result = match count.unwrap_or(0) {
        0 => {
            let used_tooltip = tray.set_tooltip(Some("Desktop Pet")).is_ok();
            let used_title = tray.set_title(None::<&str>).is_ok();
            TrayBadgeResult {
                used_title,
                used_tooltip,
            }
        }
        value => {
            let clamped = value.min(99);
            let tooltip = format!("Desktop Pet ({clamped})");
            let used_tooltip = tray.set_tooltip(Some(tooltip)).is_ok();
            let used_title = tray.set_title(Some(clamped.to_string())).is_ok();
            TrayBadgeResult {
                used_title,
                used_tooltip,
            }
        }
    };

    Ok(result)
}
