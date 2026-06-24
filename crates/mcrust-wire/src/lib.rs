//! Minecraft wire-format primitives (Java-oriented; Bedrock LE helpers later).

pub mod error;
pub mod le;
pub mod packet;
pub mod string;
pub mod varint;

pub use varint as var_int;

pub use error::WireError;
pub use fastnbt;
