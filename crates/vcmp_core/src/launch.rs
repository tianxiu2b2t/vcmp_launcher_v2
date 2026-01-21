use std::{net::Ipv4Addr, path::{Path, PathBuf}};

#[allow(unused)]
use crate::{GameLauncherError, GameLauncherResult};

#[cfg(not(target_os = "windows"))]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[allow(unused)]
fn inner_launch_game(gta_dir: &Path, dll_dir: &Path, command_line: String) -> GameLauncherResult<u32> {
    #[cfg(not(target_os = "windows"))]
    {
        return Err(GameLauncherError::NotSupportedPlatform);
    }

    // is i686 or i586
    #[cfg(not(target_arch = "x86"))]
    {
        return Err(GameLauncherError::NotSupportedArch(None));
    }

    #[allow(unused)]
    windows::launcher_common_game(gta_dir, dll_dir, command_line)
}

#[derive(Debug, Clone)]
pub struct LaunchConfig {
    gta_dir: PathBuf,
    dll_dir:  PathBuf,
    username: String,
    password: Option<String>,
    ip: Ipv4Addr,
    port: u16,
}
impl LaunchConfig {
    pub fn new(
        gta_dir: PathBuf,
        dll_dir: PathBuf,
        username: impl Into<String>,
        ip: Ipv4Addr,
        port: u16,
    ) -> Self {
        Self {
            gta_dir,
            dll_dir,
            username: username.into(),
            password: None,
            ip,
            port,
        }
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
}

pub fn launch_game(
    config: LaunchConfig,
) -> GameLauncherResult<u32> {
    //         "-c -h 120.236.253.141 -c -p 8189 -n 2b2ttianxiu -z 123456".to_string(),
    let mut command_line = format!(
        "-c -h {} -c -p {} -n {}",
        config.ip,
        config.port,
        config.username
    );
    // if password
    if let Some(password) = config.password {
        command_line.push_str(&format!(" -z {}", password));
    }

    inner_launch_game(&config.gta_dir, &config.dll_dir, command_line)
}