use mcrust_wire::packet::write_packet;
use mcrust_wire::string::write_string;
use mcrust_wire::varint::{write_var_int, write_var_long};

use crate::protocol_ids::{configuration, login, play};

pub fn set_compression(threshold: i32) -> Vec<u8> {
    let mut p = Vec::new();
    write_var_int(threshold, &mut p);
    let mut out = Vec::new();
    write_packet(login::C_SET_COMPRESSION, &p, &mut out);
    out
}

pub fn login_success(uuid: &str, username: &str) -> Vec<u8> {
    let mut p = Vec::new();
    write_string(uuid, &mut p);
    write_string(username, &mut p);
    write_var_int(0, &mut p);
    let mut out = Vec::new();
    write_packet(login::C_LOGIN_SUCCESS, &p, &mut out);
    out
}

pub fn finish_configuration() -> Vec<u8> {
    let mut out = Vec::new();
    write_packet(configuration::C_FINISH, &[], &mut out);
    out
}

pub fn configuration_keep_alive(id: i64) -> Vec<u8> {
    let mut p = Vec::new();
    write_var_long(id, &mut p);
    let mut out = Vec::new();
    write_packet(configuration::C_KEEP_ALIVE, &p, &mut out);
    out
}

/// Play state join — packet `login` (minecraft-data 1.21.1).
pub fn play_login(
    entity_id: i32,
    world_name: &str,
    dimension_name: &str,
    max_players: i32,
    view_distance: i32,
    simulation_distance: i32,
) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(&entity_id.to_be_bytes());
    p.push(0); // hardcore
    write_var_int(1, &mut p); // world count
    write_string(world_name, &mut p);
    write_var_int(max_players, &mut p);
    write_var_int(view_distance, &mut p);
    write_var_int(simulation_distance, &mut p);
    p.push(0); // reduced debug
    p.push(1); // enable respawn screen
    p.push(0); // limited crafting
    // SpawnInfo
    write_var_int(0, &mut p); // dimension id overworld
    write_string(dimension_name, &mut p);
    write_var_long(0, &mut p); // hashed seed
    p.push(1); // gamemode creative
    p.push(0xff); // previous gamemode undefined
    p.push(0); // is debug
    p.push(0); // is flat
    p.push(0); // no death
    write_var_int(0, &mut p); // portal cooldown
    p.push(0); // enforces secure chat (offline)
    let mut out = Vec::new();
    write_packet(play::C_LOGIN, &p, &mut out);
    out
}

pub fn synchronize_player_position(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(&x.to_be_bytes());
    p.extend_from_slice(&y.to_be_bytes());
    p.extend_from_slice(&z.to_be_bytes());
    p.extend_from_slice(&yaw.to_be_bytes());
    p.extend_from_slice(&pitch.to_be_bytes());
    p.push(0); // on ground
    write_var_int(1, &mut p); // teleport id
    let mut out = Vec::new();
    write_packet(play::C_PLAYER_POSITION, &p, &mut out);
    out
}

pub fn keep_alive(id: i64) -> Vec<u8> {
    let mut p = Vec::new();
    write_var_long(id, &mut p);
    let mut out = Vec::new();
    write_packet(play::C_KEEP_ALIVE, &p, &mut out);
    out
}

pub fn disconnect_login(msg: &str) -> Vec<u8> {
    let mut p = Vec::new();
    write_string(msg, &mut p);
    let mut out = Vec::new();
    write_packet(login::C_DISCONNECT, &p, &mut out);
    out
}

pub fn player_position_sync(
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(&x.to_be_bytes());
    p.extend_from_slice(&y.to_be_bytes());
    p.extend_from_slice(&z.to_be_bytes());
    p.extend_from_slice(&yaw.to_be_bytes());
    p.extend_from_slice(&pitch.to_be_bytes());
    p.push(on_ground as u8);
    let mut out = Vec::new();
    write_packet(play::S_POSITION_LOOK, &p, &mut out);
    out
}