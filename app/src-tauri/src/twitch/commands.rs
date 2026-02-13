use crate::twitch::{actor, user};
use tauri::AppHandle;

#[tauri::command]
pub fn get_current_user() -> Result<String, String> {
    user::current_user()
}

#[tauri::command]
pub async fn twitch_connect(channel: String, app: AppHandle) -> Result<(), String> {
    actor::connect(channel, app).await
}

#[tauri::command]
pub async fn twitch_disconnect() -> Result<(), String> {
    actor::disconnect();
    Ok(())
}

#[tauri::command]
pub async fn twitch_send(text: String) -> Result<(), String> {
    actor::send(&text).await
}