use std::{sync::LazyLock, time::Duration};

use anyhow::Result;
use reqwest::blocking::Client;

use crate::{models::InternetServerResponse, server::Server};

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::blocking::ClientBuilder::new().connect_timeout(Duration::from_secs(5)).build().unwrap()
});

pub fn fetch_internet_servers() -> Vec<Server> {
    // TODO: Cache this
    inner_fetch_internet("https://vcmp.txit.top/servers").unwrap()
}

fn inner_fetch_internet(
    url: &str,
) -> Result<Vec<Server>> {
    let response = CLIENT.get(url).send()?;
    let servers = response.json::<InternetServerResponse>()?;
    Ok(servers.servers)
}