use anyhow::Result;

use crate::{models::InternetServerResponse, server::Server, constant::CLIENT};

pub async fn fetch_internet_servers() -> Vec<Server> {
    let config = crate::config::get_config();
    // TODO: Cache this
    inner_fetch_internet(config.internet.get_master_url().join("servers/").unwrap())
        .await
        .unwrap()
}

async fn inner_fetch_internet(url: url::Url) -> Result<Vec<Server>> {
    let response = CLIENT.get(url).send().await?;
    let servers = response.json::<InternetServerResponse>().await?;
    Ok(servers.servers)
}
