use std::io::Read;

use tracing::event;

use crate::{config::get_config, constant::{VCMP_CORE, VCMP_CORE_PATH, VERSIONS}, models::Server};

pub fn unpack() {
    match std::fs::write(VCMP_CORE_PATH.to_path_buf(), VCMP_CORE) {
        Ok(_) => event!(tracing::Level::INFO, "Unpacked vcmp-core"),
        Err(e) => event!(tracing::Level::ERROR, "Failed to unpack vcmp-core: {}", e),
    };
    match std::fs::write(
        crate::constant::LIBRARY_REDIRECTOR_PATH.to_path_buf(),
        crate::constant::LIBRARY_REDIRECTOR,
    ) {
        Ok(_) => event!(tracing::Level::INFO, "Unpacked library-redirector"),
        Err(e) => event!(tracing::Level::ERROR, "Failed to unpack library-redirector: {}", e),
    };
}

pub fn init() {
    unpack();
}

pub fn launch(server: Server, version: impl Into<String>, password: Option<String>) -> anyhow::Result<u32>{
    let config = get_config();
    let username = match config.game.username.as_ref() {
        Some(name) => name,
        None => Err(anyhow::anyhow!("Username not set in config"))?,
    };
    let ref_game = config.game.get_game_exe();
    let gta_exe = match ref_game.as_ref() {
        Some(path) => path,
        None => Err(anyhow::anyhow!("GTA exe path not set in config"))?,
    };
    let version_path = VERSIONS.join(version.into());
    let dll_file = version_path.join("vcmp-game.dll");

    // std
    let process = std::process::Command::new(VCMP_CORE_PATH.to_path_buf())
        .arg("--gta-exe")
        .arg(gta_exe)
        .arg("--dll-file")
        .arg(dll_file)
        .arg("--username")
        .arg(username)
        .arg("--ip")
        .arg(server.ip.to_string())
        .arg("--port")
        .arg(server.port.to_string())
        .arg("--redirect-dll-path")
        .arg(version_path.to_path_buf().to_str().unwrap())
        .args(if let Some(pw) = password {
            vec!["--password".to_string(), pw]
        } else {
            vec![]
        })
        .stdout(std::process::Stdio::piped())
        .spawn();

    // match stdout pid:
    match process {
        Ok(child) => {
            // stdout: pid: 
            let id = child.stdout.and_then(|stdout| {
                let data = stdout.bytes().collect::<Result<Vec<_>, _>>().ok()?;
                let output = String::from_utf8_lossy(&data);
                let pid_str = output.trim().strip_prefix("pid: ")?;
                pid_str.parse::<u32>().ok()
            })
            .unwrap_or(0);
            Ok(id)
        },
        Err(e) => Err(anyhow::anyhow!("Failed to launch vcmp-core: {}", e)),
    }

        
        

}