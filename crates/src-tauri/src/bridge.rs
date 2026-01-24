// use launcher_core::config::Config;

#[tauri::command]
pub fn get_config() -> launcher_core::config::Config {
    launcher_core::config::get_config()
}

#[tauri::command]
pub fn set_config(config: launcher_core::config::Config) -> tauri::Result<()> {
    launcher_core::config::save_config(&config)?;
    Ok(())
}