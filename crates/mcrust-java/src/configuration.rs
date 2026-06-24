//! Java Configuration state (1.20.2+) — minimal path to Play.

use mcrust_wire::packet::write_packet;
use mcrust_wire::string::write_string;
use mcrust_wire::varint::write_var_int;

use crate::protocol_ids::configuration;

/// Minimal registry sync: empty dimension codec placeholder (client may still need more on strict versions).
pub fn registry_data_minimal() -> Vec<u8> {
    let mut p = Vec::new();
    write_string("minecraft:dimension_type", &mut p);
    write_var_int(0, &mut p);
    let mut out = Vec::new();
    write_packet(0x07, &p, &mut out);
    out
}

pub fn finish_configuration() -> Vec<u8> {
    let mut out = Vec::new();
    write_packet(configuration::C_FINISH, &[], &mut out);
    out
}

pub use crate::protocol_ids::login::S_LOGIN_ACKNOWLEDGED;
pub const S_ACK_FINISH: i32 = 0x02;