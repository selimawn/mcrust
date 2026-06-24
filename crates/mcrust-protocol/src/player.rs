use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

/// Single player model for Java and Bedrock (see docs/architecture/player.md).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub platform: Platform,
    pub name: String,
    pub uuid: Uuid,
    pub xuid: Option<String>,
    pub gamemode: Gamemode,
}
