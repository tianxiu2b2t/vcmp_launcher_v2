//! 构建脚本：把可选的 `build/vcmp_core.exe` 转成环境变量交给 `constant.rs` 的 `env!`。
//!
//! 背景：注入器 `vcmp_core` 只能编译出 **Windows i686** 的 exe，由 `cli.py` 产出并放到 `build/`。
//! 以前 `constant.rs` 直接写死 `include_bytes!("../../build/vcmp_core.exe")`，
//! 只要没先跑 `cli.py`，源码就**无法编译**（Linux/macOS 上尤其明显）。
//!
//! 现在的规则：
//! - `build/vcmp_core.exe` 存在 → 原样嵌入（保持 Windows 行为完全不变）；
//! - 不存在 → 嵌入空字节，并打印警告（源码仍可编译，`launcher_core::launch::init()` 会跳过解包）；
//! - 不存在 **且** 宿主是 Windows 且是 release → 直接报错退出，避免打出「没有注入器」的发布包。
//!
//! `cargo:rerun-if-changed` 只在路径存在时才会由 Cargo 生效，所以这里始终监视 `build/` 目录，
//! 这样 `cli.py` 之后生成 exe 也能让构建脚本重跑、重新嵌入。

use std::{env, path::Path};

/// 内嵌 exe 的来源路径，必须和 `src/constant.rs` 里的 `env!("VCMP_CORE_BYTES")` 保持一致。
const VCMP_CORE_BYTES: &str = "../../build/vcmp_core.exe";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // 监视目录（而非文件）——文件不存在时 rerun-if-changed 不会生效
    println!("cargo:rerun-if-changed=build");
    println!("cargo:rerun-if-changed={VCMP_CORE_BYTES}");

    let path = Path::new(VCMP_CORE_BYTES);
    let exists = path.is_file();
    let is_windows_host = env::var("HOST").is_ok_and(|h| h.contains("windows"));
    let is_release = env::var("PROFILE").is_ok_and(|p| p == "release");
    // 必须给绝对路径：`include_bytes!` 里写死的相对路径是相对**当前源文件**解析的
    // （constant.rs 在 src/ 下），环境变量里的相对路径则由 rustc 相对 `--crate-root`（crate 根）解析，
    // 两者差一层目录，所以这里统一拼绝对路径，避免歧义。
    let root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set by cargo");
    let abs_path = Path::new(&root).join(VCMP_CORE_BYTES);

    if exists {
        println!("cargo:rustc-env=VCMP_CORE_BYTES={}", abs_path.display());
        return;
    }

    // 兜底：`include_bytes!` 不接受空字符串，所以这里在 OUT_DIR 里放一个空文件当占位，
    // 让源码在没有注入器时也能编译（例如 Linux 上只做 `cargo check`）。
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is always set by cargo");
    let placeholder = Path::new(&out_dir).join("vcmp_core.exe");
    std::fs::write(&placeholder, []).expect("failed to write the placeholder vcmp_core.exe");

    println!("cargo:warning={VCMP_CORE_BYTES} not found; embedding an empty vcmp_core.exe \
         (run `python cli.py` / build the `vcmp_core` crate for i686-pc-windows-msvc first).");
    println!("cargo:rustc-env=VCMP_CORE_BYTES={}", placeholder.display());

    if is_windows_host && is_release {
        panic!(
            "{VCMP_CORE_BYTES} is required for a release build on Windows; \
             build it first with `python cli.py build`"
        );
    }
}
