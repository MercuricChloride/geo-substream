use crate::{ADMIN_ROLE, EDITOR_CONTROLLER_ROLE, EDITOR_ROLE};
use substreams::Hex;

/// This function will return the hex representation of the address in lowercase
pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}

pub fn role_to_enum_value(role: [u8; 32]) -> i32 {
    match role {
        EDITOR_CONTROLLER_ROLE => 1,
        EDITOR_ROLE => 2,
        ADMIN_ROLE => 3,
        _ => 0,
    }
}
