use crate::error::{WireError, WireResult};

pub fn write_u8(out: &mut Vec<u8>, v: u8) {
    out.push(v);
}

pub fn write_u16_le(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn write_u32_le(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn write_i32_le(out: &mut Vec<u8>, v: i32) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn write_i32_be(out: &mut Vec<u8>, v: i32) {
    out.extend_from_slice(&v.to_be_bytes());
}

pub fn write_f32_le(out: &mut Vec<u8>, v: f32) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub fn write_varuint32(mut v: u32, out: &mut Vec<u8>) {
    loop {
        let mut b = (v & 0x7f) as u8;
        v >>= 7;
        if v != 0 {
            b |= 0x80;
        }
        out.push(b);
        if v == 0 {
            break;
        }
    }
}

pub fn write_varint32(mut v: i32, out: &mut Vec<u8>) {
    loop {
        let mut b = (v as u8) & 0x7f;
        v = ((v as u32) >> 7) as i32;
        if v != 0 {
            b |= 0x80;
        }
        out.push(b);
        if v == 0 {
            break;
        }
    }
}

pub fn write_string_be_len(out: &mut Vec<u8>, s: &str) {
    let b = s.as_bytes();
    write_u32_le(out, b.len() as u32);
    out.extend_from_slice(b);
}

pub fn write_byte_slice(out: &mut Vec<u8>, data: &[u8]) {
    write_varuint32(data.len() as u32, out);
    out.extend_from_slice(data);
}

pub fn read_varuint32(buf: &[u8]) -> WireResult<(u32, &[u8])> {
    let mut value: u32 = 0;
    let mut size = 0u32;
    let mut rest = buf;
    loop {
        if rest.is_empty() {
            return Err(WireError::Eof);
        }
        let b = rest[0];
        rest = &rest[1..];
        value |= ((b & 0x7f) as u32) << size;
        size += 7;
        if b & 0x80 == 0 {
            break;
        }
        if size > 35 {
            return Err(WireError::VarIntTooLong);
        }
    }
    Ok((value, rest))
}

pub fn read_byte_slice(buf: &[u8]) -> WireResult<(Vec<u8>, &[u8])> {
    let (len, rest) = read_varuint32(buf)?;
    let len = len as usize;
    if rest.len() < len {
        return Err(WireError::Eof);
    }
    let (data, rest) = rest.split_at(len);
    Ok((data.to_vec(), rest))
}

pub fn read_i32_be(buf: &[u8]) -> WireResult<(i32, &[u8])> {
    if buf.len() < 4 {
        return Err(WireError::Eof);
    }
    Ok((i32::from_be_bytes(buf[0..4].try_into().unwrap()), &buf[4..]))
}