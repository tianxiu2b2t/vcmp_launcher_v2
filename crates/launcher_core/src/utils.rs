use std::{
    any::Any,
    borrow::Cow,
    io::{BufReader, Read},
};

use serde::{Deserialize, Serialize};

// printable chars
pub fn format_bytes(bytes: &[u8]) -> String {
    let mut result = String::new();
    for byte in bytes {
        if *byte >= 32 && *byte <= 126 {
            result.push(*byte as char);
        } else {
            result.push_str(format!("\\{byte:02x}").as_str());
        }
    }
    result
}

pub fn is_empty<T: Any>(value: &T) -> bool {
    // 尝试转换为String
    if let Some(string) = (value as &dyn Any).downcast_ref::<String>() {
        // delete whitespace
        let string = string.trim();
        return string.is_empty();
    }

    // 尝试转换为Option<String>
    if let Some(option) = (value as &dyn Any).downcast_ref::<Option<String>>() {
        return option.is_none() || is_empty(option.as_ref().unwrap());
    }

    // 对于其他类型，返回false
    false
}

pub fn encode_to_gbk(text: &str) -> Cow<'_, [u8]> {
    let (encoded, _, _) = encoding_rs::GBK.encode(text);
    encoded
}

pub fn decode_gbk(data: &[u8]) -> String {
    let (str, _, _) = encoding_rs::GBK.decode(data);
    str.to_string()
}

pub fn decode_gbk_trim_zero(data: &[u8]) -> String {
    let mut str = decode_gbk(data);
    str.retain(|c| c != '\0');
    str
}

pub trait CRead {
    fn read_buf(&mut self, len: usize) -> std::io::Result<Vec<u8>>;
}

impl<R: Read> CRead for BufReader<R> {
    fn read_buf(&mut self, len: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
}

pub trait ProgressbarBase: Send + Sync {
    fn update(&mut self, increase: usize);
    fn reset(&mut self);
    fn set_total(&mut self, total: usize);
    fn set_current(&mut self, current: usize);
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ProgressBar {
    pub current: usize,
    pub total: usize,
}


impl ProgressBar {
    pub fn new_with_total(total: usize) -> Self {
        Self { current: 0, total }
    }

    pub fn is_finished(&self) -> bool {
        self.current >= self.total
    }

    pub fn progress(&self) -> f32 {
        self.current as f32 / self.total as f32
    }
}


impl ProgressbarBase for ProgressBar {
    fn update(&mut self, increase: usize) {
        self.current += increase;
    }

    fn reset(&mut self) {
        self.current = 0;
    }

    fn set_total(&mut self, total: usize) {
        self.total = total;
    }

    fn set_current(&mut self, current: usize) {
        self.current = current;
    }
}