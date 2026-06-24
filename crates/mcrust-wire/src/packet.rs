use crate::error::{WireError, WireResult};
use crate::varint::{read_var_int, write_var_int};

/// Java edition packet: `length` VarInt covers `packet_id` + payload.
pub fn read_packet(buf: &[u8]) -> WireResult<(i32, Vec<u8>, &[u8])> {
    let (len, rest) = read_var_int(buf)?;
    if len < 0 {
        return Err(WireError::Eof);
    }
    let len = len as usize;
    if rest.len() < len {
        return Err(WireError::Eof);
    }
    let (packet, rest) = rest.split_at(len);
    let (id, payload) = read_var_int(packet)?;
    let payload = payload.to_vec();
    Ok((id, payload, rest))
}

pub fn write_packet(packet_id: i32, payload: &[u8], out: &mut Vec<u8>) {
    let mut inner = Vec::new();
    write_var_int(packet_id, &mut inner);
    inner.extend_from_slice(payload);
    write_var_int(inner.len() as i32, out);
    out.extend(inner);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut out = Vec::new();
        write_packet(0x00, &[1, 2, 3], &mut out);
        let (id, payload, rest) = read_packet(&out).unwrap();
        assert_eq!(id, 0);
        assert_eq!(payload, vec![1, 2, 3]);
        assert!(rest.is_empty());
    }
}
