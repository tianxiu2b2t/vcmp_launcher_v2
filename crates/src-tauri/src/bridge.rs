use launcher_core::config::Config;

#[tauri::command]
pub fn get_config() -> launcher_core::config::Config {
    Config::default()
}

#[tauri::command]
pub fn set_config(config: launcher_core::config::Config) -> tauri::Result<()> {
    // TODO: Save config
    Ok(())
}