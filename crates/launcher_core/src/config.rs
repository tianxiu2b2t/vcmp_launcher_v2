use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use tracing::{Level, event};

use crate::constant::{CONFIG_PATH, MIRROR_MASTER_URLS, MIRROR_UPDATE_URLS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    master_url: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_url: Option<url::Url>,
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
            master_url: Some(MIRROR_MASTER_URLS[0].clone()),
            update_url: Some(MIRROR_UPDATE_URLS[0].clone()),
            password: None,
        }
    }
}

impl InternetConfig {
    pub fn get_master_url(&self) -> &url::Url {
        self.master_url.as_ref().unwrap_or(&MIRROR_MASTER_URLS[0])
    }

    pub fn get_update_url(&self) -> &url::Url {
        self.update_url.as_ref().unwrap_or(&MIRROR_UPDATE_URLS[0])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub internet: InternetConfig,
    pub game: GameConfig,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn load_config_from_file() -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(CONFIG_PATH.to_path_buf())?;
    let config = toml::from_str::<Config>(&content)?;
    Ok(config)
}

pub fn init_config() {
    let config = match load_config_from_file() {
        Ok(config) => config,
        Err(e) => {
            event!(Level::WARN, "failed to load config: {}", e);
            Config::default()
        }
    };
    CONFIG.set(config).unwrap();
}

pub fn get_config() -> &'static Config {
    CONFIG.get().unwrap()
}

pub fn save_config(config: &Config) -> anyhow::Result<()> {
    // first save to memory
    CONFIG.set(config.clone()).unwrap();
    // then save to file
    let content = toml::to_string_pretty(config)?;
    std::fs::write(CONFIG_PATH.to_path_buf(), content)?;
    Ok(())
}