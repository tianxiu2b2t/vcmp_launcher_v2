pub mod bridge;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> tauri::Result<()> {
    launcher_core::logger::init(
        launcher_core::logger::LoggerConfig::default()
    );
    launcher_core::config::init_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![bridge::get_config, bridge::set_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
