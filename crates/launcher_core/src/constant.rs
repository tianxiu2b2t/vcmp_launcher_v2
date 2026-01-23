use std::{path::{Path, PathBuf}, sync::LazyLock};

pub fn serde_false() -> bool {
    false
}

macro_rules! static_urls {
    ($name:ident: [$($url:literal),+ $(,)?]) => {
        pub static $name: std::sync::LazyLock<Vec<url::Url>> = std::sync::LazyLock::new(|| {
            vec![$(
                url::Url::parse($url).unwrap(),
            )+]
        });
    };
}

static_urls!(MASTER_URLS: [
    "http://master.vc-mp.org",
    "http://master.thijn.ovh",
    "http://master.adtec.ovh"
]);

static_urls!(MIRROR_MASTER_URLS: [
    "https://vcmp.txit.top"
]);

static_urls!(UPDATE_URLS: [
    "http://u04.vc-mp.org",
    "http://u04.thijn.ovh",
    "http://u04.adtec.ovh"
]);

static_urls!(MIRROR_UPDATE_URLS: [
    "https://vcmp.txit.top"
]);

pub static APPDATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(Path::new("./appdata")));