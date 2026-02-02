mod commands;
mod models;

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

/// Guards multi-step read-modify-write operations on the store.
pub struct StoreLock(pub Mutex<()>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(StoreLock(Mutex::new(())))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::pet::get_pet_state,
            commands::pet::set_pet_animation,
            commands::pomodoro::start_pomodoro,
            commands::pomodoro::complete_pomodoro,
            commands::coins::get_coin_balance,
            commands::coins::spend_coins,
            commands::tasks::get_tasks,
            commands::tasks::add_task,
            commands::tasks::toggle_task,
            commands::tasks::delete_task,
            commands::goals::get_daily_goals,
            commands::goals::update_goal_progress,
            commands::shop::get_shop_items,
            commands::shop::purchase_item,
        ])
        .setup(|app| {
            // Build system tray
            let show_pet = MenuItem::with_id(app, "show_pet", "Show Pet", true, None::<&str>)?;
            let show_panel =
                MenuItem::with_id(app, "show_panel", "Show Panel", true, None::<&str>)?;
            let start_pomo =
                MenuItem::with_id(app, "start_pomodoro", "Start Pomodoro", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu =
                Menu::with_items(app, &[&show_pet, &show_panel, &start_pomo, &quit])?;

            let _tray = TrayIconBuilder::new()
                .tooltip("Desktop Pet")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show_pet" => {
                        if let Some(w) = app.get_webview_window("pet") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "show_panel" => {
                        if let Some(w) = app.get_webview_window("panel") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "start_pomodoro" => {
                        let _ = app.emit("tray-start-pomodoro", ());
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("panel") {
                            let _ = w.unminimize();
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
