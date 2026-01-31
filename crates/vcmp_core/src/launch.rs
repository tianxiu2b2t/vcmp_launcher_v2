use std::{
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

#[allow(unused)]
use crate::{GameLauncherError, GameLauncherResult};

#[cfg(not(target_os = "windows"))]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod windows;

#[allow(unused)]
fn inner_launch_game(
    gta_exe: &Path,
    dll_file: &Path,
    command_line: String,
) -> GameLauncherResult<u32> {
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
    windows::launcher_common_game(gta_exe, dll_file, command_line)
}

#[derive(Debug, Clone)]
pub struct LaunchConfig {
    gta_exe: PathBuf,
    dll_file: PathBuf,
    username: String,
    password: Option<String>,
    ip: Ipv4Addr,
    port: u16,
}
impl LaunchConfig {
    pub fn new(
        gta_exe: PathBuf,
        dll_file: PathBuf,
        username: impl Into<String>,
        ip: Ipv4Addr,
        port: u16,
    ) -> Self {
        Self {
            gta_exe,
            dll_file,
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

pub fn launch_game(config: LaunchConfig) -> GameLauncherResult<u32> {
    let mut command_line = format!(
        "-c -h {} -c -p {} -n {}",
        config.ip, config.port, config.username
    );
    // if password
    if let Some(password) = config.password {
        command_line.push_str(&format!(" -z {}", password));
    }

    inner_launch_game(&config.gta_exe, &config.dll_file, command_line)
}
