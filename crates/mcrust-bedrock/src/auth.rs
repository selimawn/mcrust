use std::collections::BTreeMap;

use base64::Engine;
use serde::Deserialize;
use md5::{Digest, Md5};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct ChainWrapper {
    #[serde(default, rename = "Certificate")]
    certificate: Option<CertWrap>,
    #[serde(default)]
    chain: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct CertWrap {
    chain: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExtraData {
    #[serde(default, rename = "displayName")]
    display_name: String,
    #[serde(default)]
    identity: String,
    #[serde(default, rename = "XUID")]
    xuid: String,
}

#[derive(Debug)]
pub struct BedrockIdentity {
    pub display_name: String,
    pub uuid: Uuid,
    pub xuid: Option<String>,
    pub online: bool,
}

pub fn parse_connection_request(data: &[u8]) -> Result<(Vec<u8>, String), String> {
    if data.len() < 4 {
        return Err("connection request too short".into());
    }
    let chain_len = i32::from_le_bytes(data[0..4].try_into().unwrap());
    if chain_len <= 0 || data.len() < 4 + chain_len as usize {
        return Err("invalid chain length".into());
    }
    let chain_json = &data[4..4 + chain_len as usize];
    let rest = &data[4 + chain_len as usize..];
    if rest.len() < 4 {
        return Err("missing client jwt len".into());
    }
    let jwt_len = i32::from_le_bytes(rest[0..4].try_into().unwrap());
    if rest.len() < 4 + jwt_len as usize {
        return Err("invalid jwt length".into());
    }
    let client_jwt = String::from_utf8(rest[4..4 + jwt_len as usize].to_vec())
        .map_err(|e| e.to_string())?;
    Ok((chain_json.to_vec(), client_jwt))
}

pub fn verify_login_chain(
    chain_json: &[u8],
    online_mode: bool,
) -> Result<BedrockIdentity, String> {
    let wrapper: ChainWrapper =
        serde_json::from_slice(chain_json).map_err(|e| format!("chain json: {e}"))?;
    let chain = wrapper
        .certificate
        .map(|c| c.chain)
        .or(wrapper.chain)
        .ok_or_else(|| "no chain".to_string())?;
    if chain.is_empty() {
        return Err("empty chain".into());
    }
    let online = chain.len() >= 3;
    if online_mode && !online {
        return Err("online mode requires xbox chain".into());
    }
    let extra = decode_extra_data_from_chain(&chain[chain.len() - 1])?;
    let uuid = if !extra.identity.is_empty() {
        Uuid::parse_str(&extra.identity).unwrap_or_else(|_| Uuid::new_v4())
    } else if !extra.xuid.is_empty() {
        identity_from_xuid(&extra.xuid)
    } else {
        Uuid::new_v4()
    };
    Ok(BedrockIdentity {
        display_name: if extra.display_name.is_empty() {
            "Bedrock".into()
        } else {
            extra.display_name
        },
        uuid,
        xuid: if extra.xuid.is_empty() {
            None
        } else {
            Some(extra.xuid)
        },
        online,
    })
}

fn decode_extra_data_from_chain(jwt: &str) -> Result<ExtraData, String> {
    let payload = jwt_payload_b64(jwt)?;
    let v: serde_json::Value = serde_json::from_str(&payload).map_err(|e| e.to_string())?;
    if let Some(ed) = v.get("extraData") {
        return serde_json::from_value(ed.clone()).map_err(|e| e.to_string());
    }
    Ok(ExtraData {
        display_name: v
            .get("xname")
            .and_then(|x| x.as_str())
            .unwrap_or("Bedrock")
            .to_string(),
        identity: v
            .get("identity")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string(),
        xuid: v
            .get("xid")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string(),
    })
}

fn jwt_payload_b64(jwt: &str) -> Result<String, String> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() < 2 {
        return Err("bad jwt".into());
    }
    let padded = parts[1];
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(padded)
        .or_else(|_| base64::engine::general_purpose::STANDARD.decode(padded))
        .map_err(|e| e.to_string())?;
    String::from_utf8(decoded).map_err(|e| e.to_string())
}

pub fn identity_from_xuid(xuid: &str) -> Uuid {
    let mut hasher = Md5::new();
    hasher.update(b"pocket-auth-1-xuid:");
    hasher.update(xuid.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest);
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

pub fn handshake_jwt_offline() -> Vec<u8> {
    let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(br#"{"alg":"ES256","typ":"JWT"}"#);
    let mut claims = BTreeMap::new();
    claims.insert("salt", "");
    claims.insert("exp", "9999999999");
    let payload = serde_json::to_string(&claims).unwrap();
    let payload_b64 =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload.as_bytes());
    format!("{header}.{payload_b64}.offline").into_bytes()
}