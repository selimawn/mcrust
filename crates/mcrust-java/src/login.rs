use mcrust_wire::packet::write_packet;
use mcrust_wire::string::{read_string, write_string};
use mcrust_wire::varint::{read_var_int, write_var_int};

pub const LOGIN_START: i32 = crate::protocol_ids::login::S_LOGIN_START;
pub const ENCRYPTION_REQUEST: i32 = crate::protocol_ids::login::C_ENCRYPTION_BEGIN;
pub const ENCRYPTION_RESPONSE: i32 = crate::protocol_ids::login::S_ENCRYPTION_BEGIN;
pub const LOGIN_SUCCESS: i32 = crate::protocol_ids::login::C_LOGIN_SUCCESS;

pub fn decode_login_start(payload: &[u8]) -> Result<(String, Option<String>), mcrust_wire::WireError> {
    let (name, rest) = read_string(payload, 16)?;
    if rest.is_empty() {
        return Ok((name, None));
    }
    let (has_uuid, rest) = read_var_int(rest)?;
    if has_uuid != 0 && !rest.is_empty() {
        let (uuid, _) = read_string(rest, 36)?;
        Ok((name, Some(uuid)))
    } else {
        Ok((name, None))
    }
}

pub fn encode_encryption_request(
    server_id: &str,
    public_key: &[u8],
    verify_token: &[u8],
    should_auth: bool,
) -> Vec<u8> {
    let mut p = Vec::new();
    write_string(server_id, &mut p);
    write_var_int(public_key.len() as i32, &mut p);
    p.extend_from_slice(public_key);
    write_var_int(verify_token.len() as i32, &mut p);
    p.extend_from_slice(verify_token);
    p.push(should_auth as u8);
    let mut out = Vec::new();
    write_packet(ENCRYPTION_REQUEST, &p, &mut out);
    out
}

pub fn decode_encryption_response(
    payload: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), mcrust_wire::WireError> {
    let (secret_len, rest) = read_var_int(payload)?;
    let secret_len = secret_len as usize;
    if rest.len() < secret_len {
        return Err(mcrust_wire::WireError::Eof);
    }
    let (secret, rest) = rest.split_at(secret_len);
    let (token_len, rest) = read_var_int(rest)?;
    let token_len = token_len as usize;
    if rest.len() < token_len {
        return Err(mcrust_wire::WireError::Eof);
    }
    let (token, _) = rest.split_at(token_len);
    Ok((secret.to_vec(), token.to_vec()))
}

pub fn encode_login_success(uuid: &str, name: &str) -> Vec<u8> {
    let mut p = Vec::new();
    write_string(uuid, &mut p);
    write_string(name, &mut p);
    write_var_int(0, &mut p);
    let mut out = Vec::new();
    write_packet(LOGIN_SUCCESS, &p, &mut out);
    out
}