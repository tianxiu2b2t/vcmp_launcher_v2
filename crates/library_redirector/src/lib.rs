// DLLMain

#[allow(unused)]
use std::path::PathBuf;

#[cfg(windows)]
#[no_mangle]
pub unsafe extern "system" fn DllMain(
    hinst_dll: usize,
    fdw_reason: u32,
    lpv_reserved: *const u8,
) -> bool {
    // env
    let redirect_dir = PathBuf::from(std::env::var("REDIRECT_DIR").unwrap_or_default());
    // 获取当前文件名字（也就是这个dll的名字）
    let name = std::env::current_exe().unwrap();
    let dll_name = name.file_name().unwrap().to_str().unwrap();
    // 获取重定向文件路径
    let redirect_path = redirect_dir.join(dll_name);
    // 如果重定向文件存在，则重定向
    if redirect_path.exists() {
        unsafe {
            let lib = libloading::Library::new(redirect_path).unwrap();
            let func: libloading::Symbol<unsafe extern "system" fn(
                usize,
                u32,
                *const u8,
            ) -> bool> = lib.get(b"DllMain").unwrap();
            return func(hinst_dll, fdw_reason, lpv_reserved);
        }
    }
    false
}