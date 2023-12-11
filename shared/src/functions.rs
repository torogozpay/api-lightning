pub fn buffer_as_hex(bytes: Vec<u8>) -> String { 
    let hex_str = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    return hex_str;
}
