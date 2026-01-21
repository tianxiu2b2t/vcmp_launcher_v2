use std::{sync::LazyLock, time::Duration};

use anyhow::Result;
use reqwest::Client;

use crate::{models::InternetServerResponse, server::Server};

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::ClientBuilder::new().connect_timeout(Duration::from_secs(5)).build().unwrap()
});

pub async fn fetch_internet_servers() -> Vec<Server> {
    // TODO: Cache this
    inner_fetch_internet("https://vcmp.txit.top/servers").await.unwrap()
}

async fn inner_fetch_internet(
    url: &str,
) -> Result<Vec<Server>> {
    let response = CLIENT.get(url).send().await?;
    let servers = response.json::<InternetServerResponse>().await?;
    Ok(servers.servers)
}