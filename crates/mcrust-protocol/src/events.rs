use serde::{Deserialize, Serialize};

use crate::{Player, PlayerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InboundEvent {
    PlayerJoin { player: Player },
    PlayerLeave { player_id: PlayerId },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutboundCommand {
    Disconnect { player_id: PlayerId, reason: String },
}
