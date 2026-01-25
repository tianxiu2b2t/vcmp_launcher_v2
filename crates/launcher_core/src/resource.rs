use std::{collections::HashMap, io::{BufReader, Cursor}};
use futures_util::StreamExt;
use sevenz_rust2::{ArchiveReader, Password};

use crate::{config::get_config, constant::{CLIENT, VERSIONS}, utils::{ProgressBar, ProgressbarBase}};

async fn inner_download_resource(version: &str, progressbar: &mut dyn ProgressbarBase) -> anyhow::Result<Vec<u8>> {
    let update_url = get_config().internet.get_update_url().join("download")?;
    // post form json
    let mut form = HashMap::new();
    form.insert("json", serde_json::json!({
        "version": version,
        "password": get_config().internet.password,
    }));
    let req = CLIENT.post(update_url).form(&form).send().await?;
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
    // 7z! sevenz_rust2
    let outdir = VERSIONS.join(version);
    // mkdir
    std::fs::create_dir_all(&outdir)?;
    let mut reader = BufReader::new(Cursor::new(data));
    let version = ArchiveReader::new(&mut reader, Password::empty())?.read_file("VERSION.txt")?;
    sevenz_rust2::decompress(
        &mut reader,
        outdir,
    )?;
    Ok(String::from_utf8(version)?)
}

