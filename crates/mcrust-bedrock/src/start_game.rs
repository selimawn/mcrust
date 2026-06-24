//! StartGame packet encoder (PMMP BedrockProtocol field order).

use mcrust_wire::le::{
    write_f32_le, write_i32_le, write_string_be_len, write_u8, write_varint32, write_varuint32,
};
use uuid::Uuid;

use crate::codec::wrap_gamepacket;
use crate::packets::PACKET_START_GAME;

pub fn encode_start_game(
    entity_unique_id: i64,
    runtime_id: u64,
    player_name: &str,
    world_name: &str,
    x: f32,
    y: f32,
    z: f32,
    base_game_version: &str,
) -> Vec<u8> {
    let mut p = Vec::new();
    write_varint32(entity_unique_id as i32, &mut p);
    write_varuint32(runtime_id as u32, &mut p);
    write_varint32(1, &mut p); // creative
    write_f32_le(&mut p, x);
    write_f32_le(&mut p, y);
    write_f32_le(&mut p, z);
    write_f32_le(&mut p, 0.0); // pitch
    write_f32_le(&mut p, 0.0); // yaw
    p.extend_from_slice(&0i64.to_le_bytes()); // seed
    write_level_settings(&mut p, world_name, base_game_version);
    write_string_be_len(&mut p, "mcrust");
    write_string_be_len(&mut p, world_name);
    write_string_be_len(&mut p, "");
    write_u8(&mut p, 0); // trial
    write_varint32(40, &mut p); // rewind history
    write_u8(&mut p, 0); // server auth block break
    p.extend_from_slice(&0i64.to_le_bytes()); // current tick
    write_varint32(0, &mut p); // enchant seed
    write_varuint32(0, &mut p); // block palette count
    let corr = Uuid::new_v4().to_string();
    write_string_be_len(&mut p, &corr);
    write_u8(&mut p, 1); // new inventory
    write_string_be_len(&mut p, base_game_version);
    write_nbt_empty_compound(&mut p);
    p.extend_from_slice(&0u64.to_le_bytes()); // palette checksum
    write_uuid_le(&mut p, &Uuid::new_v4());
    write_u8(&mut p, 0); // client gen
    write_u8(&mut p, 0); // block hashes
    write_u8(&mut p, 0); // network perms
    write_u8(&mut p, 0); // logging chat
    write_optional_empty(&mut p); // server join info
    write_string_be_len(&mut p, "");
    write_string_be_len(&mut p, "");
    write_string_be_len(&mut p, "");
    write_string_be_len(&mut p, "");
    wrap_gamepacket(PACKET_START_GAME, &p)
}

fn write_level_settings(out: &mut Vec<u8>, _world: &str, vanilla: &str) {
    out.extend_from_slice(&0i64.to_le_bytes()); // seed
    write_u16_le(out, 0); // biome type default
    write_string_be_len(out, "plains");
    write_varint32(0, out); // overworld
    write_varint32(1, out); // generator infinite
    write_varint32(1, out); // gamemode creative
    write_u8(out, 0); // hardcore
    write_varint32(1, out); // easy
    write_block_pos(out, 0, 64, 0);
    write_u8(out, 1); // achievements disabled
    write_varint32(0, out); // editor type
    write_u8(out, 0);
    write_u8(out, 0);
    write_varint32(0, out); // time
    write_varint32(0, out); // edu offer
    write_u8(out, 0);
    write_string_be_len(out, "");
    write_f32_le(out, 0.0);
    write_f32_le(out, 0.0);
    write_u8(out, 0);
    write_u8(out, 1); // multiplayer
    write_u8(out, 1); // lan
    write_varint32(3, out); // xbl public
    write_varint32(3, out);
    write_u8(out, 1); // commands
    write_u8(out, 0); // texture packs required
    write_varuint32(0, out); // gamerules count
    write_u32_le(out, 0); // experiments count
    write_u8(out, 0);
    write_u8(out, 0);
    write_u8(out, 0);
    write_varint32(1, out); // member perm
    write_i32_le(out, 4); // chunk tick radius
    for _ in 0..15 {
        write_u8(out, 0);
    }
    write_string_be_len(out, vanilla);
    write_i32_le(out, 0);
    write_i32_le(out, 0);
    write_u8(out, 1); // new nether
    write_string_be_len(out, "");
    write_string_be_len(out, "");
    write_u8(out, 0); // optional bool
    write_u8(out, 0); // chat restriction
    write_u8(out, 0);
    write_varint32(0, out);
    write_u8(out, 0);
}

fn write_u16_le(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_u32_le(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

fn write_block_pos(out: &mut Vec<u8>, x: i32, y: i32, z: i32) {
    write_varint32(x, out);
    write_varint32(y, out);
    write_varint32(z, out);
}

fn write_nbt_empty_compound(out: &mut Vec<u8>) {
    out.push(0x0a); // TAG_Compound
    out.push(0);
    out.push(0); // name len
    out.push(0); // TAG_End
}

fn write_uuid_le(out: &mut Vec<u8>, u: &Uuid) {
    let b = u.as_bytes();
    out.extend_from_slice(b);
}

fn write_optional_empty(out: &mut Vec<u8>) {
    write_u8(out, 0);
}