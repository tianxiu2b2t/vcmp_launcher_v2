use std::collections::HashMap;
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
    pub pi: PROCESS_INFORMATION,
    pub inject_thread: Option<HANDLE>,
    pub launched: bool,
}

impl CommonGame {
    pub fn new(gta_exe: PathBuf, dll_file: PathBuf) -> Self {
        Self {
            gta_exe,
            dll_file,
            pi: PROCESS_INFORMATION::default(),
            launched: false,
            inject_thread: None,
        }
    }

    pub fn launch(&mut self, command_line: String) -> GameLauncherResult<u32> {
        let gta_dir = self.gta_exe.parent().unwrap();

        unsafe {
            CreateProcessW(
                PCWSTR(get_wchar_t(self.gta_exe.to_str().unwrap()).as_ptr()),
                Some(PWSTR(get_wchar_t(&command_line).as_mut_ptr())),
                None,
                None,
                false,
                CREATE_SUSPENDED,
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
) -> GameLauncherResult<u32> {
    let mut game = CommonGame::new(gta_exe.to_path_buf(), dll_file.to_path_buf());
    game.launch(command_line)
}
