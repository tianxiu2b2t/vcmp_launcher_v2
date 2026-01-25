use tracing::event;

use crate::{config::get_config, constant::{VCMP_CORE, VCMP_CORE_PATH, VERSIONS}, server::Server};

pub fn unpack() {
    match std::fs::write(VCMP_CORE_PATH.to_path_buf(), VCMP_CORE) {
        Ok(_) => event!(tracing::Level::INFO, "Unpacked vcmp-core"),
        Err(e) => event!(tracing::Level::ERROR, "Failed to unpack vcmp-core: {}", e),
    };
}

pub fn launch(server: Server, version: impl Into<String>, password: Option<String>) {
    let config = get_config();
    let username = config.game.username.as_ref();
    let gta_exe = config.game.get_game_exe().as_ref();
}