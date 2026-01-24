use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

use serde::{Deserialize, Serialize};
use tracing::{Level, event};

use crate::constant::{CONFIG_PATH, MIRROR_MASTER_URLS, MIRROR_UPDATE_URLS};
use crate::utils::is_empty;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    #[serde(skip_serializing_if = "is_empty")]
    master_url: Option<String>,
    #[serde(skip_serializing_if = "is_empty")]
    update_url: Option<String>,
    #[serde(skip_serializing_if = "is_empty")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameConfig {
    #[serde(skip_serializing_if = "is_empty")]
    game_dir: Option<String>,
    #[serde(skip_serializing_if = "is_empty")]
    pub username: Option<String>,
}

impl Default for InternetConfig {
    fn default() -> Self {
        Self {
            master_url: Some(MIRROR_MASTER_URLS[0].clone().to_string()),
            update_url: Some(MIRROR_UPDATE_URLS[0].clone().to_string()),
            password: None,
        }
    }
}

impl InternetConfig {
    pub fn get_master_url(&self) -> url::Url {
        self.master_url.as_ref().and_then(|url_str| {
            url::Url::parse(url_str).map_err(|e| event!(Level::ERROR, "Failed to parse master url: '{url_str}', use default, {e}")).ok()
        }).unwrap_or_else(|| MIRROR_MASTER_URLS[0].clone())
    }

    pub fn get_update_url(&self) -> url::Url {
        self.update_url.as_ref().and_then(|url_str| {
            url::Url::parse(url_str).map_err(|e| event!(Level::ERROR, "Failed to parse master url: '{url_str}', use default, {e}")).ok()
        }).unwrap_or_else(|| MIRROR_MASTER_URLS[0].clone())
    }
}

impl GameConfig {
    pub fn get_game_dir(&self) -> Option<PathBuf> {
        self.game_dir.as_ref().map_or_else(
            || None,
            |v| {
                let path = PathBuf::from(v);
                if !path.exists() {
                    event!(Level::WARN, "Game directory '{v}' does not exist");
                    return None;   
                }
                Some(path)
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub internet: InternetConfig,
    pub game: GameConfig,
}

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

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
    CONFIG.set(RwLock::new(config)).unwrap();
}

pub fn get_config() -> Config {
    CONFIG.get().unwrap().read().unwrap().clone()
}

pub fn save_config(config: &Config) -> anyhow::Result<()> {
    // first save to memory
    CONFIG.get().unwrap().write().unwrap().clone_from(config);
    // then save to file
    let content = toml::to_string_pretty(config)?;
    std::fs::write(CONFIG_PATH.to_path_buf(), content)?;
    Ok(())
}