use std::net::IpAddr;

use serde::{Deserialize, Serialize};
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
}

pub async fn get_server_info(server: &Server) -> Result<ServerInfo> {
    // udp socket
    let ip = match server.ip {
        IpAddr::V4(ip) => ip,
        IpAddr::V6(_) => return Err(anyhow!("IPv6 not supported")),
    };
    // let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    // socket.connect((server.ip, server.port))?;
    // socket.set_read_timeout(Some(std::time::Duration::from_secs(3)))?;

    // // send request
    // // b"VCMP{v4host:b}{port:short}"
    // // i or c
    // let mut origin_buf = b"VCMP".to_vec();
    // origin_buf.extend(ip.to_bits().to_be_bytes());
    // origin_buf.extend(server.port.to_be_bytes());
    // let mut i_buf = origin_buf.clone();
    // let mut c_buf = origin_buf.clone();
    // i_buf.extend(b"i");
    // c_buf.extend(b"c");

    // socket.send(&i_buf)?;
    // socket.send(&c_buf)?;

    // // read response
    // let mut recv_i = vec![];
    // let mut recv_c = vec![];
    // for _ in 0..2 {
    //     let mut buf = [0u8; 1024];
    //     match socket.recv(&mut buf) {
    //         Ok(_) => {
    //             if buf[0] == b'i' {
    //                 recv_i.extend_from_slice(&buf[1..]);
    //             } else if buf[0] == b'c' {
    //                 recv_c.extend_from_slice(&buf[1..]);
    //             }
    //         }
    //         Err(e) => {
    //             return Err(e.into());
    //         }
    //     }
    // }

    let conn = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    conn.connect((server.ip, server.port)).await?;

    let mut originPacket = b"VCMP".to_vec();
    originPacket.extend(ip.to_bits().to_be_bytes());
    originPacket.extend(server.port.to_be_bytes());

    let mut iPacket = originPacket.clone();
    let mut cPacket = originPacket.clone();
    iPacket.extend(b"i");
    cPacket.extend(b"c");

    conn.send(&iPacket).await?;
    conn.send(&cPacket).await?;
    // let mut recv_i = vec![];
    // let mut recv_c = vec![];
    for _ in 0..2 {
        let mut buf = [0u8; 1024];
        match conn.recv(&mut buf).await {
            Ok(size) => {
                let b = &buf[..size];
                println!("{server:?} recv buf: {}", format_bytes(&b));
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
