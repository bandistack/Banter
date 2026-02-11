mod store;
mod oauth;
mod twitch;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            oauth::exchange_token,
            oauth::refresh_token,
            twitch::get_current_user,
            twitch::start_twitch_chat,
            twitch::send_twitch_message,
            twitch::logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
