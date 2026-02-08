use tauri::AppHandle;

#[tauri::command]
pub fn set_tray_badge(app: AppHandle, count: Option<u32>) -> Result<(), String> {
    let tray = app
        .tray_by_id("main")
        .ok_or_else(|| "Tray icon not found".to_string())?;

    match count.unwrap_or(0) {
        0 => {
            let _ = tray.set_tooltip(Some("Desktop Pet"));
            let _ = tray.set_title(None::<&str>);
        }
        value => {
            let clamped = value.min(99);
            let tooltip = format!("Desktop Pet ({clamped})");
            let _ = tray.set_tooltip(Some(tooltip));
            let _ = tray.set_title(Some(clamped.to_string()));
        }
    }

    Ok(())
}
