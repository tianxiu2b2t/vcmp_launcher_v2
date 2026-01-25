use std::collections::HashMap;

use crate::{config::get_config, constant::CLIENT};

pub async fn download_resource(version: &str) -> anyhow::Result<Vec<u8>> {
    let update_url = get_config().internet.get_update_url().join("download")?;
    // post form json
    let mut form = HashMap::new();
    form.insert("json", serde_json::json!({
        "version": version,
        "password": get_config().internet.password,
    }));
    let req = CLIENT.post(update_url).form(&form).send().await?;
    let data = req.bytes().await?;
    Ok(data.to_vec())
}