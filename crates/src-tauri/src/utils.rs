use std::{sync::{LazyLock, atomic::AtomicU32}, time::{SystemTime, UNIX_EPOCH}};

pub static OBJECT_ID: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(rand::random()));

pub fn object_id() -> String {
    // 4字节时间戳
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let timestamp = (since_the_epoch.as_secs() as u32).to_be_bytes();
    
    // 5字节随机数（机器ID+进程ID）
    let random_bytes1: [u8; 5] = rand::random();
    
    let id = OBJECT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst).to_be_bytes();

    // 3字节递增计数器
    let random_bytes2: [u8; 3] = [id[1], id[2], id[3]];
    
    // 组合所有字节
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&timestamp);
    bytes.extend_from_slice(&random_bytes1);
    bytes.extend_from_slice(&random_bytes2);
    
    // 转换为十六进制字符串
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}