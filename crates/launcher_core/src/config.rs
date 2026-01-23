use serde::{Deserialize, Serialize};

use crate::constant::{MIRROR_MASTER_URLS, MIRROR_UPDATE_URLS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    pub master_url: url::Url,
    pub update_url: url::Url,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameConfig {
    pub game_dir: Option<std::path::PathBuf>,
    pub username: Option<String>,
}

impl Default for InternetConfig {
    fn default() -> Self {
        Self {
            master_url: MIRROR_MASTER_URLS[0].clone(),
            update_url: MIRROR_UPDATE_URLS[0].clone(),
            password: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub internet: InternetConfig,
    pub game: GameConfig,
}

