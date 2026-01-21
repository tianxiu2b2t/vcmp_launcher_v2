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