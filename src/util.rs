use ethers::core::types::U256;
use std::str::FromStr;

pub(crate) fn convert_hex_to_u256(hex_str: &str) -> Result<U256, &'static str> {
    // Remove the '0x' prefix if present
    let clean_hex = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };

    // Convert to U256
    U256::from_str_radix(clean_hex, 16).map_err(|_| "Invalid hex string")
}