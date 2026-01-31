use std::{io::{BufRead, BufReader}, path};

use tracing::event;

use crate::{config::get_config, constant::{GAME_DATA, VCMP_CORE, VCMP_CORE_PATH, VERSIONS}, link::create_symlink, models::Server, utils::is_empty};

pub fn unpack() {
    match std::fs::write(VCMP_CORE_PATH.to_path_buf(), VCMP_CORE) {
        Ok(_) => event!(tracing::Level::INFO, "Unpacked vcmp-core"),
        Err(e) => event!(tracing::Level::ERROR, "Failed to unpack vcmp-core: {}", e),
    };
}

pub fn init() {
    unpack();
}

pub fn launch(server: Server, version: impl Into<String>, password: Option<String>) -> anyhow::Result<u32> {
    let version = version.into();
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
    let version_path = VERSIONS.join(&version);
    let dll_file = version_path.join("vcmp-game.dll");
    // create symlink to game data
    match create_symlink(&version) {
        Ok(_) => event!(tracing::Level::INFO, "Created symlink for version {}", version),
        Err(e) => {
            event!(tracing::Level::ERROR, "Failed to create symlink for version {}: {}", version, e);
            return Err(e);
        },
    };
    // copy some version to game dir
    // overwrite .appdata
    let appdata_file = GAME_DATA.join(".appdata");
    // write ../../
    std::fs::write(appdata_file, "\x00../../")?;

    // std
    let args = vec![
        "--gta-exe".to_string(),
        gta_exe.to_string_lossy().to_string(),
        "--dll-file".to_string(),
        path::absolute(dll_file)?.to_str().unwrap().to_string(),
        "--username".to_string(),
        username.to_string(),
        "--ip".to_string(),
        server.ip.to_string(),
        "--port".to_string(),
        server.port.to_string(),
        "--redirect-dll-path".to_string(),
        path::absolute(version_path)?.to_str().unwrap().to_string(),
    ];

    let password_args = if is_empty(&password) {
        vec![]
    } else {
        vec!["--password".to_string(), password.unwrap()]
    };

    let merged_args = args.into_iter().chain(password_args).collect::<Vec<String>>();
    event!(tracing::Level::INFO, "Launching vcmp-core with args: {:?}", merged_args);

    let process = std::process::Command::new(VCMP_CORE_PATH.to_path_buf())
        .args(merged_args)
        .stdout(std::process::Stdio::piped())
        .spawn();

    // match stdout pid:
    match process {
        Ok(child) => {
            let id = child.stdout.and_then(|stdout| {
                println!("Reading stdout for pid...");
                let mut reader = BufReader::new(stdout);
                let mut buffer = String::new();
                while reader.read_line(&mut buffer).unwrap_or(0) > 0 {
                    println!("stdout: {}", buffer.trim());
                    if buffer.contains("pid: ") {
                        let pid_str = buffer.trim().strip_prefix("pid: ")?;
                        return pid_str.parse::<u32>().ok();
                    }
                    buffer.clear();
                }
                None
            });
            Ok(id.unwrap_or(0))
        },
        Err(e) => Err(anyhow::anyhow!("Failed to launch vcmp-core: {}", e)),
    }

        
        

}