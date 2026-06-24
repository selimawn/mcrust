use crate::error::{WireError, WireResult};

const MAX_VARINT_BYTES: usize = 5;
const VARINT_CONTINUE_BIT: u8 = 0x80;
const VARINT_VALUE_MASK: u8 = 0x7f;

/// Read a signed 32-bit VarInt (Java protocol).
pub fn read_var_int(mut buf: &[u8]) -> WireResult<(i32, &[u8])> {
    let mut value: u32 = 0;
    let mut size = 0u32;
    loop {
        if buf.is_empty() {
            return Err(WireError::Eof);
        }
        let b = buf[0];
        buf = &buf[1..];
        value |= ((b & VARINT_VALUE_MASK) as u32) << size;
        size += 7;
        if b & VARINT_CONTINUE_BIT == 0 {
            break;
        }
        if size > (MAX_VARINT_BYTES as u32) * 7 {
            return Err(WireError::VarIntTooLong);
        }
    }
    Ok((value as i32, buf))
}

/// Write a signed 32-bit VarInt into `out`.
pub fn write_var_int(mut value: i32, out: &mut Vec<u8>) {
    loop {
        let mut temp = (value as u8) & VARINT_VALUE_MASK;
        value = ((value as u32) >> 7) as i32;
        if value != 0 {
            temp |= VARINT_CONTINUE_BIT;
        }
        out.push(temp);
        if value == 0 {
            break;
        }
    }
}

/// Length in bytes of encoded VarInt.
pub fn var_int_len(value: i32) -> usize {
    let mut len = 0;
    let mut v = value;
    loop {
        len += 1;
        v = ((v as u32) >> 7) as i32;
        if v == 0 {
            break;
        }
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_values() {
        for v in [0, 1, 127, 128, 255, 256, -1, i32::MAX] {
            let mut buf = Vec::new();
            write_var_int(v, &mut buf);
            let (decoded, rest) = read_var_int(&buf).unwrap();
            assert!(rest.is_empty());
            assert_eq!(decoded, v);
        }
    }

    #[test]
    fn wiki_zero() {
        let mut buf = Vec::new();
        write_var_int(0, &mut buf);
        assert_eq!(buf, vec![0]);
    }
}
