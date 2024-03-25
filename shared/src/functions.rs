use std::fmt::Write;

pub fn buffer_as_hex(bytes: Vec<u8>) -> String { 
    let hex_str = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    return hex_str;
}

pub fn bytes_to_string(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{:02x}", b);
        output
    })
}