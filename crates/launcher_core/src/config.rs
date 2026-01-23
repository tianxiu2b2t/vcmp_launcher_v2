use std::{cell::OnceCell, sync::OnceLock};

use serde::{Deserialize, Serialize};
use tracing::{Level, event};

use crate::constant::{MIRROR_MASTER_URLS, MIRROR_UPDATE_URLS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    pub master_url: url::Url,
    pub update_url: url::Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_dir: Option<std::path::PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("config.toml")?;
    let config = toml::from_str::<Config>(&content)?;
    CONFIG.set(config).unwrap();
    Ok(())
}

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn save_config(config: &Config) -> anyhow::Result<()> {
    // first save to memory
    CONFIG.set(config.clone()).unwrap();
    // then save to file
    let content = toml::to_string_pretty(config)?;
    std::fs::write("config.toml", content)?;
    Ok(())
}