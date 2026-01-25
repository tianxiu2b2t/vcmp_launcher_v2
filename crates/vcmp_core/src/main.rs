use std::{net::Ipv4Addr, path::PathBuf};

use clap::Parser;
use vcmp_core::launch::LaunchConfig;

/// VCMP游戏启动器
#[derive(Parser)]
#[command(name = "VCMP Launcher")]
#[command(version = "1.0")]
#[command(about = "Launches Vice City Multiplayer", long_about = None)]
struct Args {
    /// GTA Vice City安装目录
    #[arg(short, long, value_name = "PATH")]
    gta_dir: PathBuf,

    /// VCMP DLL目录
    #[arg(short, long, value_name = "PATH")]
    dll_dir: PathBuf,

    /// 玩家用户名
    #[arg(short, long, value_name = "NAME")]
    username: String,

    /// 服务器IP地址
    #[arg(short, long, value_name = "IP")]
    ip: String,

    /// 服务器端口
    #[arg(short, long, value_name = "PORT")]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let ip_addr: Ipv4Addr = match args.ip.parse() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("valid ip: {}", e);
            std::process::exit(1);
        }
    };

    match vcmp_core::launch::launch_game(LaunchConfig::new(
        args.gta_dir,
        args.dll_dir,
        args.username,
        ip_addr,
        args.port,
    )) {
        Ok(res) => {
            println!("pid: {res}")
        }
        Err(e) => {
            eprintln!("Failed to launch game: {:?}", e);
        }
    }
}
