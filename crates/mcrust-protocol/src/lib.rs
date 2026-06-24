//! Internal contract between network bridge and game core.

mod events;
mod platform;
mod player;

pub use events::{InboundEvent, OutboundCommand};
pub use platform::Platform;
pub use player::{Gamemode, Player, PlayerId};
