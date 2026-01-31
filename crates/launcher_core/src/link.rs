use std::path::{self, Path};

use tracing::{Level, event};

use crate::constant::{SYMLINK_DIRS, SYMLINK_FILES, VERSIONS};

fn create_symlink_for_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> anyhow::Result<()> {
    let src = path::absolute(src.as_ref())?;
    let dst = path::absolute(dst.as_ref())?;
    // remove link first
    if dst.exists() {
        std::fs::remove_file(&dst)?;
    }
    // if src is not exists, create file
    if !src.exists() {
        std::fs::write(&src, b"")?;
    }
    // #[cfg(target_family = "windows")]
    // std::os::windows::fs::symlink_file(src, dst)?;
    // #[cfg(not(target_family = "windows"))]
    // std::os::unix::fs::symlink(src, dst)?;3
    std::fs::hard_link(src, dst)?;
    Ok(())
}

fn create_symlink_for_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> anyhow::Result<()> {
    #[allow(unused)]
    let src = path::absolute(src.as_ref())?;
    let dst = path::absolute(dst.as_ref())?;
    // remove link first
    if dst.exists() {
        // for to remove
        std::fs::remove_dir_all(&dst)?;
    }
    // if src is not exists, create dir
    if !src.exists() {
        std::fs::create_dir_all(&src)?;
    }
    // std::fs::hard_link(src, dst)?;
    #[cfg(target_family = "windows")]
    std::os::windows::fs::junction_point(src, dst)?;
    // #[cfg(not(target_family = "windows"))]
    // std::os::unix::fs::symlink(src, dst)?;
    // use juf
    Ok(())
}

pub fn create_symlink(version: &str) -> anyhow::Result<()> {
    let version_path = VERSIONS.join(version);
    for dir in SYMLINK_DIRS.iter() {
        let link = path::absolute(version_path.join(dir))?;
        let target = path::absolute(crate::constant::GAME_DATA.join(dir))?;
        match create_symlink_for_dir(target, link) {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to create symlink for {}: {}", dir, e);
            }
        }
    }

    for file in SYMLINK_FILES.iter() {
        let link = path::absolute(version_path.join(file))?;
        let target = path::absolute(crate::constant::GAME_DATA.join(file))?;
        match create_symlink_for_file(target, link) {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to create symlink for {}: {}", file, e);
            }
        }
    }

    // create symlink
    // so VERSIONS/version/serverlogs -> GAME_DATA/serverlogs
    // and VERSIONS/version/serverlogs -> GAME_DATA/serverlogs

    Ok(())
}