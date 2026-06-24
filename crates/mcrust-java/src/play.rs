use mcrust_wire::packet::write_packet;
use mcrust_wire::string::write_string;
use mcrust_wire::varint::write_var_int;

pub fn set_compression(threshold: i32) -> Vec<u8> {
    let mut p = Vec::new();
    write_var_int(threshold, &mut p);
    let mut out = Vec::new();
    write_packet(0x03, &p, &mut out);
    out
}

pub fn finish_configuration() -> Vec<u8> {
    let mut out = Vec::new();
    write_packet(0x03, &[], &mut out);
    out
}

pub fn login_play_join_game(entity_id: i32) -> Vec<u8> {
    let mut p = Vec::new();
    write_var_int(entity_id, &mut p);
    p.push(0);
    write_var_int(0, &mut p);
    write_string("minecraft:overworld", &mut p);
    write_var_int(0, &mut p);
    write_var_int(0, &mut p);
    write_var_int(100, &mut p);
    write_var_int(10, &mut p);
    write_var_int(10, &mut p);
    p.push(0);
    p.push(1);
    p.push(0);
    p.push(1);
    p.push(0);
    write_var_int(0, &mut p);
    write_string("minecraft:peaceful", &mut p);
    p.push(1);
    p.push(1);
    write_var_int(0, &mut p);
    let mut out = Vec::new();
    write_packet(0x28, &p, &mut out);
    out
}

pub fn synchronize_position(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(&x.to_be_bytes());
    p.extend_from_slice(&y.to_be_bytes());
    p.extend_from_slice(&z.to_be_bytes());
    p.extend_from_slice(&yaw.to_be_bytes());
    p.extend_from_slice(&pitch.to_be_bytes());
    p.push(0);
    write_var_int(1, &mut p);
    let mut out = Vec::new();
    write_packet(0x40, &p, &mut out);
    out
}

pub fn keep_alive(id: i64) -> Vec<u8> {
    let mut p = Vec::new();
    p.extend_from_slice(&id.to_be_bytes());
    let mut out = Vec::new();
    write_packet(0x26, &p, &mut out);
    out
}

pub fn disconnect_login(msg: &str) -> Vec<u8> {
    let mut p = Vec::new();
    write_string(msg, &mut p);
    let mut out = Vec::new();
    write_packet(0x00, &p, &mut out);
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
    write_packet(0x3c, &p, &mut out);
    out
}