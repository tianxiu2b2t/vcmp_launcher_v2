use std::{path::{Path, PathBuf}, sync::LazyLock, time::Duration};

use reqwest::Client;

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

pub static ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(debug_assertions)]
    {
        let root = PathBuf::from(Path::new("../../runner"));
        // mkdir if not exists
        if !root.exists() {
            std::fs::create_dir_all(&root).unwrap();
        }
        root
    }

    #[cfg(not(debug_assertions))]
    PathBuf::from(Path::new("./"))
});
pub static APPDATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = ROOT.join("./appdata");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path
});
pub static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| ROOT.join("./config.toml"));
pub static VCMP_CORE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = ROOT.join("libs");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path.join("vcmp_core.exe")
});
pub static LIBRARY_REDIRECTOR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = ROOT.join("libs");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path.join("library_redirector.dll")
});

pub static VERSIONS: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = APPDATA_DIR.join("versions");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path
});

pub static GAME_DATA: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = APPDATA_DIR.join("game_data");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    path
});
pub static DATABASE_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = APPDATA_DIR.join("appdata.db");
    if !path.exists() {
        std::fs::File::create(&path).unwrap();
    }
    path
});

pub static VCMP_CORE: &[u8] = include_bytes!("../../../build/vcmp_core.exe");

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::ClientBuilder::new().connect_timeout(Duration::from_secs(5)).build().unwrap()
});

    // serverlogs(dir)
    // store(dir)
    // .appdata (file)
    // debuglog.txt (file)
    // vcmp_config.txt (file)

pub static SYMLINK_DIRS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "serverlogs",
        "store",
    ]
});

pub static SYMLINK_FILES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "vcmp_config.txt",
        "debuglog.txt",
        ".appdata",
    ]
});