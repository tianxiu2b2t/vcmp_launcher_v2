use std::{net::IpAddr, time::Duration};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::constant::serde_false;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetServerResponse {
    pub success: bool,
    pub servers: Vec<Server>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
    #[serde(default = "serde_false", rename = "is_official")]
    pub official: bool,
}

// make it equal and hash
impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip && self.port == other.port
    }
}

impl Eq for Server {}

impl std::hash::Hash for Server {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ip.hash(state);
        self.port.hash(state);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInfo {
    pub server: Server,
    pub servername: String,
    pub gamemode: String,
    pub map: String,
    pub players: u16,
    pub maxplayers: u16,
    pub password: bool,
    pub players_list: Vec<String>,
    pub version: String,
    pub elapsed: Duration,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawServerFromDatabase {
    pub address: String,
    pub port: u16,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerFromDatabase {
    pub server: Server,
    pub password: Option<String>,
}

// try into
impl TryFrom<RawServerFromDatabase> for ServerFromDatabase {
    type Error = String;

    fn try_from(value: RawServerFromDatabase) -> Result<Self, Self::Error> {
        let address = value.address.parse().map_err(|_| "Invalid address")?;
        Ok(ServerFromDatabase {
            server: Server {
                ip: address,
                port: value.port,
                official: false,
            },
            password: value.password,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawHistoryServerFromDatabase {
    pub address: String,
    pub port: u16,
    pub version: String,
    pub timestamp: chrono::DateTime::<Utc>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistoryServerFromDatabase {
    pub server: Server,
    pub version: String,
    pub timestamp: chrono::DateTime::<Utc>
}

impl TryFrom<RawHistoryServerFromDatabase> for HistoryServerFromDatabase {
    type Error = String;

    fn try_from(value: RawHistoryServerFromDatabase) -> Result<Self, Self::Error> {
        let address = value.address.parse().map_err(|_| "Invalid address")?;
        Ok(HistoryServerFromDatabase {
            server: Server {
                ip: address,
                port: value.port,
                official: false,
            },
            version: value.version,
            timestamp: value.timestamp,
        })
    }
}