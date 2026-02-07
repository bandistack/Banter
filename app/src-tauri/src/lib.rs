mod api;
mod store;
mod oauth;
mod command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::exchange_token,
            command::refresh_token
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
