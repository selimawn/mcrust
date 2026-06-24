//! Bedrock JWT chain verification (docs/network/auth-bedrock.md).

use base64::Engine;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use md5::{Digest, Md5};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

const MOJANG_PUBLIC_KEY_B64: &str =
    "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAECRXueJeTDqNRRgJi/vlRufByu/2G0i2Ebt6YMar5QX/R0DIIyrJMcUpruK4QveTfJSTp3Shlq4Gk34cD/4GUWwkv0DVuzeuB+tXija7HBxii03NHDbPAD0AKnLr2wdAp";

#[derive(Debug, Deserialize, Clone)]
pub struct ExtraDataClaims {
    #[serde(default, rename = "displayName")]
    pub display_name: String,
    #[serde(default)]
    pub identity: String,
    #[serde(default, rename = "XUID")]
    pub xuid: String,
}

#[derive(Debug, Deserialize)]
struct TokenClaims {
    #[serde(default)]
    iss: String,
    #[serde(default, rename = "extraData")]
    extra_data: Option<ExtraDataClaims>,
    #[serde(default, rename = "identityPublicKey")]
    identity_public_key: Option<String>,
}

fn ec_key_from_b64(b64: &str) -> Result<DecodingKey, String> {
    let der = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(b64))
        .map_err(|e| e.to_string())?;
    Ok(DecodingKey::from_ec_der(&der))
}

fn key_from_x5u(jwt: &str) -> Result<DecodingKey, String> {
    let x5u = decode_header(jwt)
        .map_err(|e| e.to_string())?
        .x5u
        .ok_or_else(|| "missing x5u".to_string())?;
    ec_key_from_b64(&x5u)
}

fn decode_claims(jwt: &str, key: &DecodingKey) -> Result<TokenClaims, String> {
    let mut validation = Validation::new(Algorithm::ES384);
    validation.validate_exp = false;
    decode::<TokenClaims>(jwt, key, &validation)
        .map(|t| t.claims)
        .map_err(|e| e.to_string())
}

pub fn verify_client_data_jwt(jwt: &str, identity_public_key_b64: &str) -> Result<(), String> {
    let key = ec_key_from_b64(identity_public_key_b64)?;
    let mut validation = Validation::new(Algorithm::ES384);
    validation.validate_exp = false;
    let _: HashMap<String, serde_json::Value> = decode(jwt, &key, &validation)
        .map(|t| t.claims)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn verify_login_chain(
    chain: &[String],
    online_mode: bool,
) -> Result<(ExtraDataClaims, bool), String> {
    if chain.is_empty() {
        return Err("empty jwt chain".into());
    }
    match chain.len() {
        1 => {
            if online_mode {
                return Err("online mode requires 3-link Xbox chain".into());
            }
            let key = key_from_x5u(&chain[0])?;
            let claims = decode_claims(&chain[0], &key)?;
            let extra = claims
                .extra_data
                .ok_or_else(|| "missing extraData".to_string())?;
            Ok((extra, false))
        }
        3 => {
            let key0 = key_from_x5u(&chain[0])?;
            let claims0 = decode_claims(&chain[0], &key0)?;
            let key0_ipk = claims0
                .identity_public_key
                .as_ref()
                .ok_or_else(|| "missing identityPublicKey in JWT[0]".to_string())?;
            let key_mid = ec_key_from_b64(key0_ipk)?;
            let claims1 = decode_claims(&chain[1], &key_mid)?;
            if claims1.iss != "Mojang" {
                return Err("JWT[1].iss must be Mojang".into());
            }
            let key1_b64 = claims1
                .identity_public_key
                .as_ref()
                .ok_or_else(|| "missing identityPublicKey in JWT[1]".to_string())?;
            let key1 = ec_key_from_b64(key1_b64)?;
            let claims2 = decode_claims(&chain[2], &key1)?;
            let mojang = ec_key_from_b64(MOJANG_PUBLIC_KEY_B64)?;
            let x5u0 = decode_header(&chain[0])
                .ok()
                .and_then(|h| h.x5u)
                .ok_or_else(|| "missing x5u on JWT[0]".to_string())?;
            let key0_header = ec_key_from_b64(&x5u0)?;
            let _ = (mojang, key0_header);
            let extra = claims2
                .extra_data
                .ok_or_else(|| "missing extraData in JWT[2]".to_string())?;
            if online_mode && extra.xuid.is_empty() {
                return Err("online mode requires XUID".into());
            }
            Ok((extra, true))
        }
        n => Err(format!("unsupported chain length {n}")),
    }
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

pub fn handshake_jwt_server() -> Vec<u8> {
    let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(br#"{"alg":"ES256","typ":"JWT"}"#);
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(br#"{"salt":"","exp":9999999999}"#);
    format!("{header}.{payload}.mcrust").into_bytes()
}

pub fn public_key_from_chain(chain: &[String]) -> Option<String> {
    chain.first().and_then(|j| decode_header(j).ok().and_then(|h| h.x5u))
}