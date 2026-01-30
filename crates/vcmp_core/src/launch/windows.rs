use std::ffi::OsStr;
use std::ffi::c_void;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;
use windows::core::{PCSTR, PCWSTR, PWSTR};

use crate::{GameLauncherError, GameLauncherResult};

fn get_wchar_t(content: &str) -> Vec<u16> {
    OsStr::new(content)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>()
}

#[derive(Debug)]
pub struct CommonGame {
    pub gta_exe: PathBuf,
    pub dll_file: PathBuf,
    pub redirect_dll_path: PathBuf,
    pub pi: PROCESS_INFORMATION,
    pub inject_thread: Option<HANDLE>,
    pub launched: bool,
}

impl CommonGame {
    pub fn new(gta_exe: PathBuf, dll_file: PathBuf, redirect_dll_path: PathBuf) -> Self {
        Self {
            gta_exe,
            dll_file,
            pi: PROCESS_INFORMATION::default(),
            launched: false,
            inject_thread: None,
            redirect_dll_path,
        }
    }

    pub fn launch(&mut self, command_line: String) -> GameLauncherResult<u32> {
        let gta_dir = self.gta_exe.parent().unwrap();
        // 2. 创建自定义环境变量字符串
        let mut env_vars = vec![
            // ("ALLUSERSPROFILE", "C:\\ProgramData"),
            // ("APPDATA", "D:\\Files\\TestVC"),
            // ("COMPUTERNAME", "2B2TTIANXIU"),
            // ("ComSpec", "C:\\WINDOWS\\system32\\cmd.exe"),
            // ("CURL_HOME", "D:\\curl-8.12.1_1-win64-mingw\\bin"),
            // ("DevEco Studio", "D:\\DevEco Studio\\bin;"),
            // ("DriverData", "C:\\Windows\\System32\\Drivers\\DriverData"),
            // ("EFC_10088_1262719628", "1"),
            // ("EFC_10088_1592913036", "1"),
            // ("EFC_10088_2283032206", "1"),
            // ("EFC_10088_2775293581", "1"),
            // ("EFC_10088_3789132940", "1"),
            // ("FPS_BROWSER_APP_PROFILE_STRING", "Internet Explorer"),
            // ("FPS_BROWSER_USER_PROFILE_STRING", "Default"),
            // (
            //     "GIT_ASKPASS",
            //     "d:\\Microsoft VS Code\\resources\\app\\extensions\\git\\dist\\askpass.sh",
            // ),
            // ("HOMEDRIVE", "C:"),
            // ("HOMEPATH", "\\Users\\2b2ttianxiu"),
            // (
            //     "IntelliJ IDEA Community Edition",
            //     "D:\\idea\\IntelliJ IDEA Community Edition 2024.2\\bin;",
            // ),
            // ("LANG", "en_US.UTF-8"),
            // (
            //     "LD_LIBRARY_PATH",
            //     "C:\\Users\\2b2ttianxiu\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib",
            // ),
            // ("LOCALAPPDATA", "C:\\Users\\2b2ttianxiu\\AppData\\Local"),
            // ("LOGONSERVER", "\\\\2B2TTIANXIU"),
            // ("MSVC_CL", "D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\"),
            // ("NODE_HOME", "D:\\nodejs"),
            // ("NUMBER_OF_PROCESSORS", "4"),
            // ("OS", "Windows_NT"),
            // (
            //     "OUT_DIR",
            //     "D:\\workspaces\\vcmp_launcher_v2\\target\\i686-pc-windows-msvc\\debug\\build\\vcmp_core-f45103b127fced34\\out",
            // ),
            // (
            //     "Path",
            //     "D:\\workspaces\\vcmp_launcher_v2\\target\\i686-pc-windows-msvc\\debug;D:\\workspaces\\vcmp_launcher_v2\\target\\i686-pc-windows-msvc\\debug\\deps;C:\\Users\\2b2ttianxiu\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib;C:\\Program Files\\PowerShell\\7;c:\\Users\\2b2ttianxiu\\AppData\\Roaming\\Code\\User\\globalStorage\\github.copilot-chat\\debugCommand;c:\\Users\\2b2ttianxiu\\AppData\\Roaming\\Code\\User\\globalStorage\\github.copilot-chat\\copilotCli;C:\\Program Files\\Zulu\\zulu-25\\bin\\;D:\\curl-8.12.1_1-win64-mingw\\bin;C:\\Program Files (x86)\\Common Files\\Oracle\\Java\\javapath;C:\\Program Files\\Common Files\\Oracle\\Java\\javapath;C:\\WINDOWS\\system32;C:\\WINDOWS;C:\\WINDOWS\\System32\\Wbem;C:\\WINDOWS\\System32\\WindowsPowerShell\\v1.0\\;C:\\WINDOWS\\System32\\OpenSSH\\;C:\\Program Files (x86)\\NVIDIA Corporation\\PhysX\\Common;C:\\Program Files\\Bandizip\\;C:\\Program Files\\Git\\cmd;C:\\Program Files\\dotnet\\;D:\\OpenSSL-Win64\\bin;D:\\Windows Kits\\10\\Windows Performance Toolkit\\;D:\\go\\bin;C:\\Program Files\\CMake\\bin;D:\\llvm-mingw-20250402-msvcrt-x86_64\\bin;%NODE_PATH%;D:\\nodejs\\node_global;D:\\nodejs\\node_cache;D:\\nodejs;C:\\Users\\2b2ttianxiu\\AppData\\Roaming\\npm;D:\\ffmpeg;D:\\vcpkg;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx64\\x64;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx64\\x86;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx86\\x64;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx86\\x86;C:\\Program Files\\NVIDIA Corporation\\NVIDIA App\\NvDLISR;D:\\adb;C:\\Program Files (x86)\\PowerShell\\7\\;C:\\Program Files\\PowerShell\\7\\;D:\\easyshare\\x86\\;D:\\easyshare\\x64\\;C:\\Users\\2b2ttianxiu\\.cargo\\bin;C:\\Users\\2b2ttianxiu\\AppData\\Local\\Programs\\Python\\Python312\\Scripts\\;C:\\Users\\2b2ttianxiu\\AppData\\Local\\Programs\\Python\\Python312\\;C:\\Users\\2b2ttianxiu\\AppData\\Local\\Programs\\Python\\Launcher\\;C:\\Users\\2b2ttianxiu\\AppData\\Local\\Microsoft\\WindowsApps;C:\\Program Files\\Tesseract-OCR;D:\\idea\\IntelliJ IDEA Community Edition 2024.2\\bin;D:\\sqlite3;D:\\Microsoft VS Code\\bin;C:\\Users\\2b2ttianxiu\\.dotnet\\tools;Z:\\ffmpeg\\bin;C:\\Users\\2b2ttianxiu\\go\\bin;D:\\Mongodb\\sh\\;D:\\nodejs\\node_global;C:\\Users\\2b2ttianxiu\\AppData\\Roaming\\npm;D:\\ffmpeg;D:\\vcpkg;C:\\Users\\2b2ttianxiu\\AppData\\Local\\JetBrains\\Toolbox\\scripts;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx64\\x64;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx86\\x64;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx64\\x86;D:\\VCS\\VC\\Tools\\MSVC\\14.40.33807\\bin\\\\Hostx86\\x86;C:\\Users\\2b2ttianxiu\\AppData\\Local\\Microsoft\\WindowsApps;D:\\adb;C:\\Users\\2b2ttianxiu\\AppData\\Local\\Programs\\Zed Nightly\\bin;D:\\RustRover 2025.2.2\\bin;D:\\DevEco Studio\\bin;;C:\\Users\\2b2ttianxiu\\AppData\\Local\\GitHubDesktop\\bin;C:\\Users\\2b2ttianxiu\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\bin",
            // ),
            // (
            //     "PATHEXT",
            //     ".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC;.PY;.PYW;.CPL",
            // ),
            // ("POWERSHELL_DISTRIBUTION_CHANNEL", "MSI:Windows 10 Pro"),
            // ("PROCESSOR_ARCHITECTURE", "x86"),
            // ("PROCESSOR_ARCHITEW6432", "AMD64"),
            // (
            //     "PROCESSOR_IDENTIFIER",
            //     "Intel64 Family 6 Model 158 Stepping 9, GenuineIntel",
            // ),
            // ("PROCESSOR_LEVEL", "6"),
            // ("PROCESSOR_REVISION", "9e09"),
            // ("ProgramData", "C:\\ProgramData"),
            // ("ProgramFiles", "C:\\Program Files (x86)"),
            // ("ProgramFiles(x86)", "C:\\Program Files (x86)"),
            // ("ProgramW6432", "C:\\Program Files"),
            // (
            //     "PSModulePath",
            //     "D:\\Documents\\PowerShell\\Modules;C:\\Program Files\\PowerShell\\Modules;c:\\program files\\powershell\\7\\Modules;C:\\Program Files\\WindowsPowerShell\\Modules;C:\\WINDOWS\\system32\\WindowsPowerShell\\v1.0\\Modules",
            // ),
            // ("SESSIONNAME", "Console"),
            // ("SystemDrive", "C:"),
            // ("SystemRoot", "C:\\WINDOWS"),
            // ("TEMP", "C:\\Users\\2B2TTI~1\\AppData\\Local\\Temp"),
            // ("TMP", "C:\\Users\\2B2TTI~1\\AppData\\Local\\Temp"),
            // ("USERDOMAIN", "2B2TTIANXIU"),
            // ("USERDOMAIN_ROAMINGPROFILE", "2B2TTIANXIU"),
            // ("USERNAME", "2b2ttianxiu"),
            // ("USERPROFILE", "C:\\Users\\2b2ttianxiu"),
            // ("windir", "C:\\WINDOWS"),
            // ("ZES_ENABLE_SYSMAN", "1"),
            ("VCMP_REDIRECT_DLL_PATH", self.redirect_dll_path.to_str().unwrap()),
        ];
        println!("{env_vars:?}");

        let mut env_block = String::new();
        for (key, val) in env_vars {
            env_block.push_str(key);
            env_block.push('=');
            env_block.push_str(val);
            env_block.push('\0');
            env_block.push('\0');
        }
        env_block.push('\0'); // 双空字符终止
        env_block.push('\0');

        // 转换为宽字符
        let mut env_wide: &mut [u8] = unsafe { env_block.as_bytes_mut() };

        unsafe {
            CreateProcessW(
                PCWSTR(get_wchar_t(self.gta_exe.to_str().unwrap()).as_ptr()),
                Some(PWSTR(get_wchar_t(&command_line).as_mut_ptr())),
                None,
                None,
                false,
                CREATE_SUSPENDED | CREATE_UNICODE_ENVIRONMENT,
                None,// Some(env_wide.as_mut_ptr() as *const c_void),
                PCWSTR(get_wchar_t(gta_dir.to_str().unwrap()).as_ptr()),
                &STARTUPINFOW::default(),
                &mut self.pi,
            )
            .map_err(|e| GameLauncherError::CreateProcessWFailed(e.message()))?;
        }

        let dll_wide = get_wchar_t(self.dll_file.to_str().unwrap());
        let byte_len = dll_wide.len() * 2 + 1; // 字节数

        let remote_buf = unsafe {
            VirtualAllocEx(
                self.pi.hProcess,
                None,
                byte_len,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };
        if remote_buf.is_null() {
            return Err(GameLauncherError::VirtualAllocExFailed);
        }

        let mut written = 0usize;
        unsafe {
            WriteProcessMemory(
                self.pi.hProcess,
                remote_buf,
                dll_wide.as_ptr() as *const c_void,
                byte_len,
                Some(&mut written),
            )
        }
        .map_err(|_| GameLauncherError::WriteProcessMemoryFailed)?;

        if written != byte_len {
            return Err(GameLauncherError::WriteProcessMemoryFailed);
        }

        let load_lib = unsafe {
            let kernel = GetModuleHandleA(PCSTR(c"kernel32.dll".as_ptr() as *const u8))
                .map_err(|_| GameLauncherError::GetModuleHandleFailed)?;
            GetProcAddress(kernel, PCSTR(c"LoadLibraryW".as_ptr() as *const u8))
                .ok_or(GameLauncherError::GetProcAddressFailed)?
        };

        let inject_thread = unsafe {
            CreateRemoteThread(
                self.pi.hProcess,
                None,
                0,
                Some(std::mem::transmute::<
                    unsafe extern "system" fn() -> isize,
                    unsafe extern "system" fn(*mut std::ffi::c_void) -> u32,
                >(load_lib)),
                Some(remote_buf),
                0,
                None,
            )
            .map_err(|_| GameLauncherError::CreateRemoteThreadFailed)?
        };

        unsafe {
            if WaitForSingleObject(inject_thread, 10000) != WAIT_OBJECT_0 {
                return Err(GameLauncherError::WaitForSingleObjectFailed(
                    WaitForSingleObject(inject_thread, 0).0,
                ));
            }
        }

        self.launched = true;

        Ok(self.pi.dwProcessId)
    }

    pub fn clean(&mut self) {
        unsafe {
            if !self.pi.hProcess.is_invalid() {
                if !self.launched {
                    TerminateProcess(self.pi.hProcess, 0).ok();
                } else {
                    ResumeThread(self.pi.hThread);
                }
            }
            if let Some(inject_thread) = self.inject_thread {
                CloseHandle(inject_thread).ok();
                self.inject_thread = None
            }
            CloseHandle(self.pi.hThread).ok();
            CloseHandle(self.pi.hProcess).ok();
        }
    }
}

impl Drop for CommonGame {
    fn drop(&mut self) {
        self.clean();
    }
}

pub fn launcher_common_game(
    gta_exe: &Path,
    dll_file: &Path,
    command_line: String,
    redirect_dll_path: &Path,
) -> GameLauncherResult<u32> {
    let mut game = CommonGame::new(gta_exe.to_path_buf(), dll_file.to_path_buf(), redirect_dll_path.to_path_buf());
    game.launch(command_line)
}
