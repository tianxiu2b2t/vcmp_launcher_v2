use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetConfig {
    pub master_url: url::Url,
    pub update_url: url::Url,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub game_dir: std::path::PathBuf,
    pub username: String,
}
