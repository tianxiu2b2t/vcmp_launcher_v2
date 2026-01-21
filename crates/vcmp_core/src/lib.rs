use std::fmt::Display;

pub mod launch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameLauncherError {
    CreateProcessWFailed(String),
    VirtualAllocExFailed,
    WriteProcessMemoryFailed,
    GetModuleHandleFailed,
    GetProcAddressFailed,
    CreateRemoteThreadFailed,
    WaitForSingleObjectFailed(u32),
    GetExitCodeThreadFailed,
    LauncherFailed(u32),
    NotSupportedPlatform,
    // 不支持架构
    NotSupportedArch(Option<String>),
}

pub fn get_last_error() -> u32 {
    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::Foundation::GetLastError;
        GetLastError().0
    }
    #[cfg(not(target_os = "windows"))]
    0
}

impl Display for GameLauncherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameLauncherError::CreateProcessWFailed(err) => {
                write!(f, "CreateProcessWFailed: {}", err)
            }
            GameLauncherError::VirtualAllocExFailed => {
                write!(f, "VirtualAllocExFailed {:?}", get_last_error())
            }
            GameLauncherError::WriteProcessMemoryFailed => {
                write!(f, "WriteProcessMemoryFailed {:?}", get_last_error())
            }
            GameLauncherError::GetModuleHandleFailed => {
                write!(f, "GetModuleHandleFailed {:?}", get_last_error())
            }
            GameLauncherError::GetProcAddressFailed => {
                write!(f, "GetProcAddressFailed {:?}", get_last_error())
            }
            GameLauncherError::CreateRemoteThreadFailed => {
                write!(f, "CreateRemoteThreadFailed {:?}", get_last_error())
            }
            GameLauncherError::WaitForSingleObjectFailed(event) => {
                write!(f, "WaitForSingleObjectFailed {:?}", event)
            }
            GameLauncherError::GetExitCodeThreadFailed => {
                write!(f, "GetExitCodeThreadFailed {:?}", get_last_error())
            }
            GameLauncherError::LauncherFailed(code) => write!(f, "LauncherFailed: {}", code),
            GameLauncherError::NotSupportedPlatform => {
                write!(f, "NotSupportedPlatform")
            }
            GameLauncherError::NotSupportedArch(e) => write!(f, "NotSupportedArch: {:?}", e),
        }
    }
}

pub type GameLauncherResult<T> = Result<T, GameLauncherError>;
