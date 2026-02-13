pub mod store;
pub mod oauth;
pub mod twitch;
pub mod models;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            oauth::oauth::chtk,
            store::store::logout,
            twitch::commands::get_current_user,
            twitch::commands::twitch_connect,
            twitch::commands::twitch_disconnect,
            twitch::commands::twitch_send,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}