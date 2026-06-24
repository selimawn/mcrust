use crate::error::{WireError, WireResult};
use crate::varint::{read_var_int, write_var_int};

/// Java edition: VarInt length (in bytes) + UTF-8 payload.
pub fn read_string(buf: &[u8], max_len: usize) -> WireResult<(String, &[u8])> {
    let (len, rest) = read_var_int(buf)?;
    if len < 0 {
        return Err(WireError::StringTooLong {
            max: max_len,
            len: len as usize,
        });
    }
    let len = len as usize;
    if len > max_len {
        return Err(WireError::StringTooLong { max: max_len, len });
    }
    if rest.len() < len {
        return Err(WireError::Eof);
    }
    let (s_bytes, rest) = rest.split_at(len);
    let s = std::str::from_utf8(s_bytes)?.to_owned();
    Ok((s, rest))
}

pub fn write_string(s: &str, out: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    write_var_int(bytes.len() as i32, out);
    out.extend_from_slice(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut buf = Vec::new();
        write_string("Notch", &mut buf);
        let (s, rest) = read_string(&buf, 32767).unwrap();
        assert_eq!(s, "Notch");
        assert!(rest.is_empty());
    }
}
