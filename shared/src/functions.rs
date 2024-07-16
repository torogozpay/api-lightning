use base64::decode; 
use hex::encode; 
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

pub fn base64_to_hex(base64_str : String) -> String { 
    // Decode base64 string into bytes 
    let decoded_bytes = decode(base64_str).unwrap();
    // Convert bytes to hexadecimal string 
    let hex_str = encode(&decoded_bytes); 
    return hex_str;
 }
