use std::time::Duration;

use anyhow::{anyhow, Result};
use futures_util::FutureExt;
use tokio::time::timeout;

use crate::{
    constant::CLIENT,
    models::{InternetServerResponse, Server},
};

/// 单次请求（含读 body）的超时时间；连接超时另外在 `CLIENT` 上配了 5s。
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// 拉取互联网服务器列表。
///
/// 旧实现是 `inner_fetch_internet(...).await.unwrap()`：镜像站连不上（TLS 被重置、返回 HTML 等）时
/// 会在 tokio worker 线程上 panic。而这条链路是前端 `onMounted` 触发的，
/// 一次网络抖动就够刷出一堆 panic（用户实测日志见 CHANGELOG）。
/// 现在把错误原样往上抛给 Tauri command，由前端展示，不再 panic。
pub async fn fetch_internet_servers() -> Result<Vec<Server>> {
    let config = crate::config::get_config();
    // TODO: Cache this
    let mut urls = vec![config.internet.get_master_url()];
    // 配置里的镜像站失败时，回退到内置的备用地址（官方 master 列表）
    for fallback in crate::constant::MASTER_URLS.iter() {
        if !urls.iter().any(|known| known == fallback) {
            urls.push(fallback.clone());
        }
    }

    // 依次尝试：只有前一个失败才发下一个请求
    let attempts = urls.into_iter().map(|base| {
        async move {
            let url = base.join("servers/")?;
            match timeout(FETCH_TIMEOUT, inner_fetch_internet(url.clone())).await {
                Ok(result) => result.map_err(|e| anyhow!("{url}: {e:#}")),
                Err(_) => Err(anyhow!(
                    "{url}: 请求超时（{}s）",
                    FETCH_TIMEOUT.as_secs()
                )),
            }
        }
        .boxed()
    });

    futures_util::future::select_ok(attempts)
        .await
        .map(|(servers, _)| servers)
        .map_err(|e| anyhow!("所有服务器列表地址都请求失败：{e:#}"))
}

async fn inner_fetch_internet(url: url::Url) -> Result<Vec<Server>> {
    let response = CLIENT.get(url).send().await?;
    // 镜像站异常时经常返回 HTML/空响应，这里给一个比 serde 报错更可读的信息
    let status = response.status();
    let body = response.text().await?;
    let servers = serde_json::from_str::<InternetServerResponse>(&body).map_err(|e| {
        let head: String = body.chars().take(120).collect();
        anyhow!("响应不是合法的服务器列表 JSON（HTTP {status}）：{e}；响应开头：{head:?}")
    })?;
    Ok(servers.servers)
}
