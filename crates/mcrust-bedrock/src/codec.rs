use mcrust_wire::le::{read_i32_be, read_varuint32, write_byte_slice, write_i32_be, write_u8, write_varuint32};

pub const COMPRESSION_NONE: u16 = 0xffff;
pub const COMPRESSION_FLATE: u16 = 0;

pub fn wrap_gamepacket(packet_id: u32, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    write_varuint32(packet_id, &mut out);
    write_byte_slice(&mut out, payload);
    out
}

pub fn encode_batch(packets: &[Vec<u8>], compress: bool) -> Vec<u8> {
    let mut body = Vec::new();
    for pkt in packets {
        write_varuint32(pkt.len() as u32, &mut body);
        body.extend_from_slice(pkt);
    }
    if compress {
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&body, 6);
        let mut out = Vec::new();
        write_varuint32(compressed.len() as u32, &mut out);
        out.extend_from_slice(&compressed);
        out
    } else {
        let mut out = Vec::new();
        write_varuint32(body.len() as u32, &mut out);
        out.extend_from_slice(&body);
        out
    }
}

pub fn decode_batch_payload(data: &[u8], compressed: bool) -> Result<Vec<Vec<u8>>, String> {
    let (len, rest) = read_varuint32(data).map_err(|e| e.to_string())?;
    let len = len as usize;
    if rest.len() < len {
        return Err("batch eof".into());
    }
    let (raw, _) = rest.split_at(len);
    let body = if compressed {
        miniz_oxide::inflate::decompress_to_vec_zlib(raw).map_err(|e| e.to_string())?
    } else {
        raw.to_vec()
    };
    let mut packets = Vec::new();
    let mut buf = body.as_slice();
    while !buf.is_empty() {
        let (plen, rest) = read_varuint32(buf).map_err(|e| e.to_string())?;
        let plen = plen as usize;
        if rest.len() < plen {
            break;
        }
        let (p, rest) = rest.split_at(plen);
        packets.push(p.to_vec());
        buf = rest;
    }
    Ok(packets)
}

pub fn parse_gamepacket(buf: &[u8]) -> Result<(u32, Vec<u8>), String> {
    let (id, rest) = read_varuint32(buf).map_err(|e| e.to_string())?;
    let (payload, _) = mcrust_wire::le::read_byte_slice(rest).map_err(|e| e.to_string())?;
    Ok((id, payload))
}

pub fn encode_request_network_settings_response(client_proto: i32) -> Vec<u8> {
    let mut p = Vec::new();
    write_i32_be(&mut p, client_proto);
    wrap_gamepacket(0x8f, &p)
}

pub fn encode_network_settings() -> Vec<u8> {
    let mut p = Vec::new();
    mcrust_wire::le::write_u16_le(&mut p, 256);
    mcrust_wire::le::write_u16_le(&mut p, COMPRESSION_NONE);
    p.push(0);
    p.push(0);
    mcrust_wire::le::write_f32_le(&mut p, 0.0);
    wrap_gamepacket(0x8f, &p)
}

pub fn encode_play_status(status: i32) -> Vec<u8> {
    let mut p = Vec::new();
    write_i32_be(&mut p, status);
    wrap_gamepacket(0x02, &p)
}

pub fn encode_server_to_client_handshake(jwt: &[u8]) -> Vec<u8> {
    let mut p = Vec::new();
    write_byte_slice(&mut p, jwt);
    wrap_gamepacket(0x03, &p)
}

pub fn encode_resource_packs_info() -> Vec<u8> {
    let mut p = Vec::new();
    write_varuint32(0, &mut p);
    write_varuint32(0, &mut p);
    write_u8(&mut p, 0);
    write_u8(&mut p, 0);
    write_varuint32(0, &mut p);
    wrap_gamepacket(0x06, &p)
}

pub fn encode_resource_pack_stack() -> Vec<u8> {
    let mut p = Vec::new();
    write_varuint32(0, &mut p);
    write_u8(&mut p, 0);
    wrap_gamepacket(0x07, &p)
}

pub fn encode_set_local_player_initialized(runtime_id: u64) -> Vec<u8> {
    let mut p = Vec::new();
    mcrust_wire::le::write_varuint32(runtime_id as u32, &mut p);
    wrap_gamepacket(0x71, &p)
}

pub fn read_request_network_settings(payload: &[u8]) -> Result<i32, String> {
    let (proto, _) = read_i32_be(payload).map_err(|e| e.to_string())?;
    Ok(proto)
}