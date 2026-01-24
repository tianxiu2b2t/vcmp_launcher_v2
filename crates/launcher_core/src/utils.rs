use std::any::Any;

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