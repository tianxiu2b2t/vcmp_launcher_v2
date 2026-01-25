use std::{io::BufReader, net::IpAddr, time::Duration};

use crate::{
    constant::serde_false,
    utils::{CRead, decode_gbk, decode_gbk_trim_zero},
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

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
    pub players: u16,
    pub maxplayers: u16,
    pub password: bool,
    pub players_list: Vec<String>,
    pub version: String,
    pub elapsed: Duration,
}

impl TryFrom<(Server, Vec<u8>, Vec<u8>, Duration)> for ServerInfo {
    type Error = anyhow::Error;
    fn try_from(value: (Server, Vec<u8>, Vec<u8>, Duration)) -> anyhow::Result<Self> {
        let server = value.0;
        let mut i = BufReader::new(value.1.as_slice());
        let mut c = BufReader::new(value.2.as_slice());
        let e = value.3;
        Ok(Self {
            server,
            version: decode_gbk_trim_zero(i.read_buf(12)?.as_slice()),
            password: i.read_buf(1)?[0] == 1,
            players: u16::from_le_bytes(i.read_buf(2)?.as_slice().try_into().unwrap()),
            maxplayers: u16::from_le_bytes(i.read_buf(2)?.as_slice().try_into().unwrap()),
            servername: decode_gbk(
                {
                    let size = u32::from_le_bytes(i.read_buf(4)?.as_slice().try_into().unwrap());
                    i.read_buf(size as usize)?
                }
                .as_slice(),
            ),
            gamemode: decode_gbk(
                {
                    let size = u32::from_le_bytes(i.read_buf(4)?.as_slice().try_into().unwrap());
                    i.read_buf(size as usize)?
                }
                .as_slice(),
            ),
            map: decode_gbk(
                {
                    let size = u32::from_le_bytes(i.read_buf(4)?.as_slice().try_into().unwrap());
                    i.read_buf(size as usize)?
                }
                .as_slice(),
            ),
            players_list: {
                let online = u16::from_le_bytes(c.read_buf(2)?.as_slice().try_into().unwrap());
                let mut players = Vec::with_capacity(online as usize);
                for _ in 0..online {
                    let size = c.read_buf(1)?[0];
                    players.push(decode_gbk(c.read_buf(size as usize)?.as_slice()));
                }
                players
            },
            elapsed: e,
        })
    }
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
    let mut recv_i: Vec<u8> = vec![];
    let mut recv_c: Vec<u8> = vec![];
    let start = std::time::Instant::now();
    let mut elapsed: Option<Duration> = None;
    while recv_i.is_empty() || recv_c.is_empty() {
        let mut buf = [0u8; 1024];
        match conn.recv(&mut buf).await {
            Ok(size) => {
                let data = &buf[origin_packet.len()..size];
                let p = data[0];
                let data = &data[1..];
                if elapsed.is_none() {
                    elapsed = Some(start.elapsed());
                }
                if p == b'i' {
                    recv_i.extend(data);
                } else if p == b'c' {
                    recv_c.extend(data);
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    // println!("recv_i: {:?}", format_bytes(&recv_i));
    // println!("recv_c: {:?}", format_bytes(&recv_c));

    Ok(
        ServerInfo::try_from((server.clone(), recv_i, recv_c, elapsed.unwrap()))
            .map_err(|e| anyhow!("{server:?}: {e}"))
            .unwrap(),
    )
}

pub async fn get_server_info(server: &Server, millis: u64) -> Result<ServerInfo> {
    timeout(Duration::from_millis(millis), inner_get_server_info(server))
        .await
        .map_err(|e| anyhow!(e))?
}
