//! Java Configuration state (1.20.2+) — path to Play.

use fastnbt::nbt;
use mcrust_wire::packet::write_packet;
use mcrust_wire::string::write_string;
use mcrust_wire::varint::write_var_int;

use crate::protocol_ids::configuration;

pub const S_ACK_FINISH: i32 = configuration::S_ACK_FINISH;
pub const S_KEEP_ALIVE: i32 = 0x04;
pub use crate::protocol_ids::login::S_LOGIN_ACKNOWLEDGED;

fn write_registry_data_packet(registry: &str, entry_name: &str, nbt_bytes: &[u8]) -> Vec<u8> {
    let mut p = Vec::new();
    write_string(registry, &mut p);
    write_var_int(1, &mut p);
    write_string(entry_name, &mut p);
    p.push(1); // has data
    p.extend_from_slice(nbt_bytes);
    let mut out = Vec::new();
    write_packet(configuration::C_REGISTRY_DATA, &p, &mut out);
    out
}

/// `minecraft:dimension_type` with a single overworld entry (1.21.x).
pub fn registry_dimension_type_overworld() -> Vec<u8> {
    let value = nbt!({
        "height": 384i32,
        "min_y": -64i32,
        "logical_height": 384i32,
        "infiniburn": "minecraft:infiniburn_overworld",
        "effects": "minecraft:overworld",
        "ambient_light": 0.0f32,
        "has_skylight": 1i8,
        "has_ceiling": 0i8,
        "ultrawarm": 0i8,
        "natural": 1i8,
        "coordinate_scale": 1.0f64,
        "bed_works": 1i8,
        "respawn_anchor_works": 0i8,
        "has_raids": 1i8,
        "monster_spawn_light_level": {
            "type": "minecraft:uniform",
            "value": {
                "min_inclusive": 0i32,
                "max_inclusive": 7i32
            }
        },
        "monster_spawn_block_light_limit": 0i32
    });
    let nbt_bytes = fastnbt::to_bytes(&value).expect("dimension_type nbt");
    write_registry_data_packet("minecraft:dimension_type", "minecraft:overworld", &nbt_bytes)
}

pub fn registry_data_minimal() -> Vec<u8> {
    registry_dimension_type_overworld()
}

pub fn finish_configuration() -> Vec<u8> {
    let mut out = Vec::new();
    write_packet(configuration::C_FINISH, &[], &mut out);
    out
}

pub fn configuration_keep_alive(id: i64) -> Vec<u8> {
    let mut p = Vec::new();
    mcrust_wire::varint::write_var_long(id, &mut p);
    let mut out = Vec::new();
    write_packet(configuration::C_KEEP_ALIVE, &p, &mut out);
    out
}