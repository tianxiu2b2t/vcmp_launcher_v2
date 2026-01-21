use serde::{Deserialize, Serialize};

use crate::server::Server;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetServerResponse {
    pub success: bool,
    pub servers: Vec<Server>
}