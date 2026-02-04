use serde_json::json;
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::events::EVENT_COINS_CHANGED;
use crate::models::CoinBalance;

#[tauri::command]
pub fn get_coin_balance(app: AppHandle) -> Result<CoinBalance, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let coins: CoinBalance = store
        .get("coins")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(coins)
}

#[tauri::command]
pub fn spend_coins(
    app: AppHandle,
    store_lock: tauri::State<'_, crate::StoreLock>,
    amount: u32,
) -> Result<CoinBalance, String> {
    let _guard = store_lock.0.lock().map_err(|e| e.to_string())?;
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let mut coins: CoinBalance = store
        .get("coins")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    if coins.available() < amount {
        return Err("Insufficient coins".to_string());
    }

    coins.spent += amount;
    store.set("coins", json!(coins));
    let _ = app.emit(EVENT_COINS_CHANGED, &coins);
    Ok(coins)
}
