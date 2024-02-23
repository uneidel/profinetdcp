pub fn mac_address_to_bytes(mac_address: &str) -> Vec<u8> {
    
    mac_address
        .trim()
        .split(':')
        .map(|byte| u8::from_str_radix(byte, 16).unwrap())
        .collect()
}

pub fn mac_address_to_string(mac_address: &[u8]) -> String {
    mac_address
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(":")
}

pub fn dotted_decimal_tostring(dd : &[u8]) -> String {
    return dd
        .iter()
        .map(|byte| format!("{}", byte))
        .collect::<Vec<String>>()
        .join(".");
}
