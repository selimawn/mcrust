//! RakNet unconnected ping / pong (MCPE discovery).
//! Reference: wiki.vg Bedrock / RakNet offline message.

pub const OFFLINE_MESSAGE_DATA_ID: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

pub const UNCONNECTED_PING: u8 = 0x01;
pub const UNCONNECTED_PONG: u8 = 0x1c;

pub fn is_unconnected_ping(buf: &[u8]) -> bool {
    buf.len() >= 25 && buf[0] == UNCONNECTED_PING && buf[9..25] == OFFLINE_MESSAGE_DATA_ID
}

/// Build pong: id + ping time (8) + magic + server guid (8) + MOTD string
pub fn build_unconnected_pong(ping_time: u64, server_guid: u64, motd: &str) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(UNCONNECTED_PONG);
    out.extend_from_slice(&ping_time.to_be_bytes());
    out.extend_from_slice(&OFFLINE_MESSAGE_DATA_ID);
    out.extend_from_slice(&server_guid.to_be_bytes());
    out.extend_from_slice(motd.as_bytes());
    out
}

pub fn parse_ping_time(buf: &[u8]) -> Option<u64> {
    if !is_unconnected_ping(buf) {
        return None;
    }
    let mut t = [0u8; 8];
    t.copy_from_slice(&buf[1..9]);
    Some(u64::from_be_bytes(t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pong_starts_with_id() {
        let p = build_unconnected_pong(1, 2, "MCPE;Test;;1;0;10;0;0;mcrust;0;");
        assert_eq!(p[0], UNCONNECTED_PONG);
    }
}
