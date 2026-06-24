use mcrust_wire::packet::{read_packet, write_packet};
use mcrust_wire::string::{read_string, write_string};
use mcrust_wire::varint::{read_var_int, write_var_int};
use mcrust_wire::WireError;
use serde::Serialize;

pub const STATE_HANDSHAKE: i32 = 0;
pub const STATE_STATUS: i32 = 1;
pub const STATE_LOGIN: i32 = 2;

pub const HANDSHAKE_ID: i32 = 0x00;
pub const STATUS_REQUEST_ID: i32 = 0x00;
pub const STATUS_RESPONSE_ID: i32 = 0x00;
pub const PING_ID: i32 = 0x01;
pub const PONG_ID: i32 = 0x01;

/// Known protocol versions (multi-version D-002). Extend per release.
pub fn version_name(protocol: i32) -> Option<&'static str> {
    match protocol {
        767 => Some("1.21"),
        768 => Some("1.21.2"),
        769 => Some("1.21.4"),
        _ => None,
    }
}

pub fn is_supported_protocol(protocol: i32) -> bool {
    version_name(protocol).is_some()
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

#[allow(dead_code)]
pub fn encode_status_request() -> Vec<u8> {
    let mut out = Vec::new();
    write_packet(STATUS_REQUEST_ID, &[], &mut out);
    out
}

pub fn decode_handshake(payload: &[u8]) -> Result<(Handshake, &[u8]), WireError> {
    let (protocol_version, rest) = read_var_int(payload)?;
    let (server_address, rest) = read_string(rest, 32767)?;
    if rest.len() < 2 {
        return Err(WireError::Eof);
    }
    let server_port = u16::from_be_bytes([rest[0], rest[1]]);
    let rest = &rest[2..];
    let (next_state, rest) = read_var_int(rest)?;
    Ok((
        Handshake {
            protocol_version,
            server_address,
            server_port,
            next_state,
        },
        rest,
    ))
}

#[derive(Serialize)]
pub struct StatusResponse<'a> {
    pub version: StatusVersion<'a>,
    pub players: StatusPlayers,
    #[serde(rename = "description")]
    pub description: StatusDescription<'a>,
}

#[derive(Serialize)]
pub struct StatusVersion<'a> {
    pub name: &'a str,
    pub protocol: i32,
}

#[derive(Serialize)]
pub struct StatusPlayers {
    pub max: u32,
    pub online: u32,
}

#[derive(Serialize)]
pub struct StatusDescription<'a> {
    pub text: &'a str,
}

pub fn encode_status_response(json: &str) -> Vec<u8> {
    let mut payload = Vec::new();
    write_string(json, &mut payload);
    let mut out = Vec::new();
    write_packet(STATUS_RESPONSE_ID, &payload, &mut out);
    out
}

pub fn encode_pong(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    write_packet(PONG_ID, payload, &mut out);
    out
}

pub fn read_packets_from_buffer(buffer: &mut Vec<u8>) -> Result<Vec<(i32, Vec<u8>)>, WireError> {
    let mut packets = Vec::new();
    loop {
        match read_packet(buffer) {
            Ok((id, payload, rest)) => {
                packets.push((id, payload));
                *buffer = rest.to_vec();
            }
            Err(WireError::Eof) => break,
            Err(e) => return Err(e),
        }
    }
    Ok(packets)
}

pub fn decode_ping_payload(payload: &[u8]) -> Result<i64, WireError> {
    if payload.len() < 8 {
        return Err(WireError::Eof);
    }
    Ok(i64::from_be_bytes(payload[0..8].try_into().unwrap()))
}

#[allow(dead_code)]
pub fn encode_handshake(protocol: i32, address: &str, port: u16, next_state: i32) -> Vec<u8> {
    let mut payload = Vec::new();
    write_var_int(protocol, &mut payload);
    write_string(address, &mut payload);
    payload.extend_from_slice(&port.to_be_bytes());
    write_var_int(next_state, &mut payload);
    let mut out = Vec::new();
    write_packet(HANDSHAKE_ID, &payload, &mut out);
    out
}
