// use launcher_core::config::Config;

use launcher_core::server::{Server, ServerInfo};
use tauri::AppHandle;

use crate::state::TauriProgressbar;

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
pub async fn ping_server(
    server: launcher_core::server::Server,
    millis: u64,
) -> tauri::Result<ServerInfo> {
    Ok(launcher_core::server::get_server_info(&server, millis).await?)
}

#[tauri::command]
pub fn open_folder_dialog() -> Option<String> {
    None
}


#[tauri::command]
pub async fn download_resource(app_handle: AppHandle, version: &str) -> tauri::Result<String> {
    Ok(inner_download_resource(app_handle, version).await.unwrap())
}

async fn inner_download_resource(
    app_handle: AppHandle,
    version: &str,
) -> anyhow::Result<String> {
    let mut progressbar = TauriProgressbar::new(version.to_string(), app_handle);
    progressbar.set_status("Downloading resource");
    let data = launcher_core::resource::download_resource(version, Some(&mut progressbar)).await?;
    progressbar.set_status("Unpacking resource");
    let version = launcher_core::resource::unpack_resource(version, &data)?;
    progressbar.set_status("Done");
    Ok(version)
}

#[tauri::command]
pub async fn launch(server: Server) -> tauri::Result<()> {
    Ok(())
}