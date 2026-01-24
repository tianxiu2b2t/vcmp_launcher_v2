// use launcher_core::config::Config;

use launcher_core::server::ServerInfo;

#[tauri::command]
pub fn get_config() -> launcher_core::config::Config {
    launcher_core::config::get_config()
}

#[tauri::command]
pub fn set_config(config: launcher_core::config::Config) -> tauri::Result<()> {
    launcher_core::config::save_config(&config)?;
    Ok(())
}

#[tauri::command]
pub async fn fetch_internet_servers() -> tauri::Result<Vec<launcher_core::server::Server>> {
    Ok(launcher_core::internet::fetch_internet_servers().await)
}

#[tauri::command]
pub async fn ping_server(server: launcher_core::server::Server, millis: u64) -> tauri::Result<ServerInfo> {
    Ok(launcher_core::server::get_server_info(&server, millis).await?)
}

#[tauri::command]
pub fn open_folder_dialog() -> Option<String> {
    None
}