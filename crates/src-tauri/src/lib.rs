use std::sync::OnceLock;

use tauri::AppHandle;

pub mod bridge;
pub mod state;
pub mod utils;

pub static APP_HANDLE: OnceLock<&'static AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> tauri::Result<()> {
    launcher_core::logger::init(launcher_core::logger::LoggerConfig::default());
    launcher_core::config::init_config();
    launcher_core::launch::init();
    launcher_core::database::init_database().await;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            bridge::get_config,
            bridge::set_config,
            bridge::fetch_internet_servers,
            bridge::ping_server,
            bridge::download_resource,
            bridge::random_object_id,
            bridge::launch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
