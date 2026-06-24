use mcrust_wire::le::{
    write_byte_slice, write_f32_le, write_i32_be, write_i32_le, write_string_be_len, write_u8,
    write_varint32, write_varuint32,
};

use crate::codec::wrap_gamepacket;

pub const PACKET_NETWORK_SETTINGS: u32 = 143;
pub const PACKET_PLAY_STATUS: u32 = 2;
pub const PACKET_SERVER_HANDSHAKE: u32 = 3;
pub const PACKET_RESOURCE_PACKS_INFO: u32 = 6;
pub const PACKET_RESOURCE_PACK_STACK: u32 = 7;
pub const PACKET_START_GAME: u32 = 11;
pub const PACKET_SET_LOCAL_PLAYER: u32 = 113;
pub const PACKET_MOVE_PLAYER: u32 = 19;
pub const PACKET_REQUEST_NETWORK_SETTINGS: u32 = 193;
pub const PACKET_LOGIN: u32 = 1;

pub fn network_settings(threshold: u16, algorithm: u16) -> Vec<u8> {
    let mut p = Vec::new();
    mcrust_wire::le::write_u16_le(&mut p, threshold);
    mcrust_wire::le::write_u16_le(&mut p, algorithm);
    p.push(0);
    p.push(0);
    write_f32_le(&mut p, 0.0);
    wrap_gamepacket(PACKET_NETWORK_SETTINGS, &p)
}

pub fn play_status(status: i32) -> Vec<u8> {
    let mut p = Vec::new();
    write_i32_be(&mut p, status);
    wrap_gamepacket(PACKET_PLAY_STATUS, &p)
}

pub fn server_handshake(jwt: &[u8]) -> Vec<u8> {
    let mut p = Vec::new();
    write_byte_slice(&mut p, jwt);
    wrap_gamepacket(PACKET_SERVER_HANDSHAKE, &p)
}

pub fn resource_packs_info() -> Vec<u8> {
    let mut p = Vec::new();
    write_varuint32(0, &mut p);
    write_varuint32(0, &mut p);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_varuint32(0, &mut p);
    wrap_gamepacket(PACKET_RESOURCE_PACKS_INFO, &p)
}

pub fn resource_pack_stack() -> Vec<u8> {
    let mut p = Vec::new();
    write_varuint32(0, &mut p);
    write_u8(&mut p, 0);
    wrap_gamepacket(PACKET_RESOURCE_PACK_STACK, &p)
}

pub fn set_local_player_initialized(runtime_id: u64) -> Vec<u8> {
    let mut p = Vec::new();
    write_varuint32(runtime_id as u32, &mut p);
    wrap_gamepacket(PACKET_SET_LOCAL_PLAYER, &p)
}

pub fn move_player(
    runtime_id: u64,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
) -> Vec<u8> {
    let mut p = Vec::new();
    write_varuint32(runtime_id as u32, &mut p);
    write_varint32(0, &mut p);
    write_f32_le(&mut p, x);
    write_f32_le(&mut p, y);
    write_f32_le(&mut p, z);
    write_f32_le(&mut p, pitch);
    write_f32_le(&mut p, yaw);
    write_u8(&mut p, 0);
    write_u8(&mut p, on_ground as u8);
    write_varuint32(0, &mut p);
    wrap_gamepacket(PACKET_MOVE_PLAYER, &p)
}

pub fn start_game_minimal(
    entity_unique_id: i64,
    runtime_id: u64,
    name: &str,
    x: f32,
    y: f32,
    z: f32,
) -> Vec<u8> {
    let mut p = Vec::new();
    write_varint32(entity_unique_id as i32, &mut p);
    write_varuint32(runtime_id as u32, &mut p);
    write_varint32(1, &mut p);
    write_f32_le(&mut p, x);
    write_f32_le(&mut p, y);
    write_f32_le(&mut p, z);
    write_f32_le(&mut p, 0.0);
    write_f32_le(&mut p, 0.0);
    p.extend_from_slice(&0i64.to_le_bytes());
    mcrust_wire::le::write_i32_le(&mut p, 0);
    write_string_be_len(&mut p, "plains");
    write_varint32(0, &mut p);
    write_varint32(1, &mut p);
    write_varint32(1, &mut p);
    write_u8(&mut p, 0);
    write_varint32(1, &mut p);
    write_varint32(0, &mut p);
    write_varint32(0, &mut p);
    write_u8(&mut p, 1);
    write_varint32(0, &mut p);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_varint32(0, &mut p);
    write_varint32(0, &mut p);
    write_u8(&mut p, 0);
    write_string_be_len(&mut p, "");
    write_f32_le(&mut p, 0.0);
    write_f32_le(&mut p, 0.0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 1);
    write_u8(&mut p, 0);
    write_u8(&mut p, 1);
    write_u8(&mut p, 0);
    write_u8(&mut p, 1);
    write_u8(&mut p, 0);
    write_varuint32(0, &mut p);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_varint32(2, &mut p);
    write_varint32(4, &mut p);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_string_be_len(&mut p, "1.21.50");
    write_i32_le(&mut p, 0);
    write_i32_le(&mut p, 0);
    write_u8(&mut p, 1);
    write_string_be_len(&mut p, "");
    write_u8(&mut p, 0);
    write_string_be_len(&mut p, name);
    write_string_be_len(&mut p, "");
    write_u8(&mut p, 0);
    write_f32_le(&mut p, 0.16);
    write_f32_le(&mut p, 0.16);
    write_u8(&mut p, 1);
    write_u8(&mut p, 1);
    write_u8(&mut p, 1);
    write_u8(&mut p, 1);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    p.extend_from_slice(&0i64.to_le_bytes());
    write_varint32(0, &mut p);
    write_varuint32(0, &mut p);
    let corr = uuid::Uuid::new_v4().to_string();
    write_string_be_len(&mut p, &corr);
    write_u8(&mut p, 0);
    write_string_be_len(&mut p, "1.21.50");
    write_varuint32(0, &mut p);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_string_be_len(&mut p, "");
    write_string_be_len(&mut p, "");
    write_string_be_len(&mut p, "");
    write_string_be_len(&mut p, "");
    wrap_gamepacket(PACKET_START_GAME, &p)
}