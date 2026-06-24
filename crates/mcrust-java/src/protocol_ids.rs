//! Packet IDs for Java Edition 1.21.1 (protocol 767) — minecraft-data.

pub mod login {
    pub const C_DISCONNECT: i32 = 0x00;
    pub const C_ENCRYPTION_BEGIN: i32 = 0x01;
    pub const C_LOGIN_SUCCESS: i32 = 0x02;
    pub const C_SET_COMPRESSION: i32 = 0x03;
    pub const S_LOGIN_START: i32 = 0x00;
    pub const S_ENCRYPTION_BEGIN: i32 = 0x01;
    pub const S_LOGIN_ACKNOWLEDGED: i32 = 0x03;
}

pub mod configuration {
    pub const C_FINISH: i32 = 0x03;
    pub const C_KEEP_ALIVE: i32 = 0x04;
    pub const S_ACK_FINISH: i32 = 0x02;
}

pub mod play {
    pub const C_LOGIN: i32 = 0x2b;
    pub const C_KEEP_ALIVE: i32 = 0x26;
    pub const C_PLAYER_POSITION: i32 = 0x40;
    pub const S_KEEP_ALIVE: i32 = 0x18;
    pub const S_POSITION: i32 = 0x1a;
    pub const S_POSITION_LOOK: i32 = 0x1b;
}

pub const PROTOCOL_1_21_1: i32 = 767;