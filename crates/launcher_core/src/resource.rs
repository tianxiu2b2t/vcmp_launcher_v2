use std::io::{BufReader, Cursor};
use anyhow::anyhow;
use futures_util::StreamExt;
use sevenz_rust2::{ArchiveReader, Password};

use crate::{config::get_config, constant::{CLIENT, VERSIONS}, utils::{ProgressBar, ProgressbarBase}};

async fn inner_download_resource(version: &str, progressbar: &mut dyn ProgressbarBase) -> anyhow::Result<Vec<u8>> {
    let update_url = get_config().internet.get_update_url().join("download/")?;
    // post form json
    let req_data = serde_json::to_string(&serde_json::json!({
        "version": version,
        "password": get_config().internet.password.unwrap_or_default(),
    })).unwrap();
    let req = CLIENT.post(update_url).multipart(reqwest::multipart::Form::new().text("json", req_data)).send().await?;
    let req = req.error_for_status()?;
    let total = req.content_length().unwrap_or(0);
    progressbar.set_total(total as usize);
    let mut data = vec![];
    let mut stream = req.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item?;
        data.extend_from_slice(&item);
        progressbar.update(item.len());
    }
    Ok(data)
}

pub async fn download_resource(version: &str, progressbar: Option<&mut dyn ProgressbarBase>) -> anyhow::Result<Vec<u8>> {
    let mut default_progressbar: Box<dyn ProgressbarBase> = Box::new(ProgressBar::default());
    inner_download_resource(version, progressbar.unwrap_or(&mut *default_progressbar)).await
}

pub fn unpack_resource(
    version: &str,
    data: &[u8],
) -> anyhow::Result<String> {
    if data.is_empty() {
        return Err(anyhow!("Resource is empty"))
    }
    // 7z! sevenz_rust2
    let outdir = VERSIONS.join(version);
    // mkdir
    std::fs::create_dir_all(&outdir)?;
    let mut reader = BufReader::new(Cursor::new(data));
    let version = ArchiveReader::new(&mut reader, Password::empty()).map_err(|f| anyhow::anyhow!("7z reader failed"))?.read_file("version.txt").map_err(|e| anyhow!(("Failed to read version.txt")))?;
    sevenz_rust2::decompress(
        &mut reader,
        outdir,
    )?;
    Ok(String::from_utf8(version)?)
}

