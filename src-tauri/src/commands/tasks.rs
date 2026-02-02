use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::models::Task;

fn load_tasks(app: &AppHandle) -> Result<Vec<Task>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    Ok(store
        .get("tasks")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default())
}

fn save_tasks(app: &AppHandle, tasks: &[Task]) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    store.set("tasks", json!(tasks));
    Ok(())
}

#[tauri::command]
pub fn get_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    load_tasks(&app)
}

#[tauri::command]
pub fn add_task(app: AppHandle, title: String) -> Result<Task, String> {
    let mut tasks = load_tasks(&app)?;
    let task = Task {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        completed: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    tasks.push(task.clone());
    save_tasks(&app, &tasks)?;
    Ok(task)
}

#[tauri::command]
pub fn toggle_task(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    task_id: String,
) -> Result<Vec<Task>, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let mut tasks = load_tasks(&app)?;
    let was_incomplete = tasks
        .iter()
        .find(|t| t.id == task_id)
        .map(|t| !t.completed)
        .unwrap_or(false);

    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.completed = !task.completed;
    }
    save_tasks(&app, &tasks)?;

    // If task was just completed, increment goal
    if was_incomplete {
        let _ = crate::commands::goals::increment_goal_progress(&app, "tasks");
    }

    Ok(tasks)
}

#[tauri::command]
pub fn delete_task(app: AppHandle, task_id: String) -> Result<Vec<Task>, String> {
    let mut tasks = load_tasks(&app)?;
    tasks.retain(|t| t.id != task_id);
    save_tasks(&app, &tasks)?;
    Ok(tasks)
}
