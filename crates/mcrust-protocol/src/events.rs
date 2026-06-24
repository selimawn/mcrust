use serde::{Deserialize, Serialize};

use crate::{Gamemode, Platform, Player, PlayerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InboundEvent {
    PlayerJoin { player: Player },
    PlayerLeave { player_id: PlayerId },
    PlayerInput {
        player_id: PlayerId,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    KeepAliveAck {
        player_id: PlayerId,
        payload: i64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutboundCommand {
    Disconnect {
        player_id: PlayerId,
        reason: String,
    },
    KeepAlive {
        player_id: PlayerId,
        payload: i64,
    },
    TeleportPlayer {
        player_id: PlayerId,
        position: Vec3f,
        yaw: f32,
        pitch: f32,
    },
    BroadcastMovement {
        player_id: PlayerId,
        position: Vec3f,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    /// Notify one client that another player exists (spawn / tab).
    PlayerInfo {
        target_session: PlayerId,
        player: Player,
        position: Vec3f,
    },
}

#[derive(Debug, Clone)]
pub struct JoinParams {
    pub name: String,
    pub uuid: uuid::Uuid,
    pub platform: Platform,
    pub xuid: Option<String>,
    pub gamemode: Gamemode,
}