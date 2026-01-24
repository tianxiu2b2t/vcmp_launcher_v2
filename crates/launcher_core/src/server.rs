use std::{net::IpAddr, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use crate::{constant::serde_false, utils::format_bytes};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    ip: IpAddr,
    port: u16,
    #[serde(default = "serde_false", rename = "is_official")]
    official: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerInfo {
    pub server: Server,
    pub servername: String,
    pub gamemode: String,
    pub map: String,
    pub players: u8,
    pub maxplayers: u8,
    pub password: bool,
    pub players_list: Vec<String>,
    pub version: String,
}

async fn inner_get_server_info(server: &Server) -> Result<ServerInfo> {
    // udp socket
    let ip = match server.ip {
        IpAddr::V4(ip) => ip,
        IpAddr::V6(_) => return Err(anyhow!("IPv6 not supported")),
    };

    let conn = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    conn.connect((server.ip, server.port)).await?;

    let mut origin_packet = b"VCMP".to_vec();
    origin_packet.extend(ip.to_bits().to_be_bytes());
    origin_packet.extend(server.port.to_be_bytes());

    let mut i_packet = origin_packet.clone();
    let mut c_packet = origin_packet.clone();
    i_packet.extend(b"i");
    c_packet.extend(b"c");

    conn.send(&i_packet).await?;
    conn.send(&c_packet).await?;
    let mut recv_i = vec![];
    let mut recv_c = vec![];
    for _ in 0..2 {
        let mut buf = [0u8; 1024];
        match conn.recv(&mut buf).await {
            Ok(size) => {
                let b = &buf[origin_packet.len()..size];
                if b[0] == b'i' {
                    recv_i.extend(b);
                } else if b[0] == b'c' {
                    recv_c.extend(b);
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    // println!("recv_i: {:?}", format_bytes(&recv_i));
    // println!("recv_c: {:?}", format_bytes(&recv_c));

    Err(anyhow::anyhow!("None"))
}

pub async fn get_server_info(server: &Server) -> Result<ServerInfo> {
    timeout(Duration::from_secs(5), inner_get_server_info(server)).await?
}