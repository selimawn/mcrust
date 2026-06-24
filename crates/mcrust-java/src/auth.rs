use serde::Deserialize;
use sha1::{Digest, Sha1};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct HasJoinedProfile {
    pub id: String,
    pub name: String,
}

pub async fn has_joined(
    client: &reqwest::Client,
    username: &str,
    server_id: &str,
    ip: Option<&str>,
) -> Result<Option<HasJoinedProfile>, reqwest::Error> {
    let mut url = reqwest::Url::parse("https://sessionserver.mojang.com/session/minecraft/hasJoined")
        .expect("url");
    url.query_pairs_mut()
        .append_pair("username", username)
        .append_pair("serverId", server_id);
    if let Some(ip) = ip {
        url.query_pairs_mut().append_pair("ip", ip);
    }
    let resp = client.get(url).send().await?;
    if resp.status() == reqwest::StatusCode::NO_CONTENT {
        return Ok(None);
    }
    let profile = resp.json::<HasJoinedProfile>().await?;
    Ok(Some(profile))
}

pub fn minecraft_server_hash(server_id: &str, secret: &[u8; 16], public_key_der: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(server_id.as_bytes());
    hasher.update(secret);
    hasher.update(public_key_der);
    let digest = hasher.finalize();
    java_digest_hex(&digest)
}

fn java_digest_hex(digest: &[u8]) -> String {
    if digest[0] & 0x80 != 0 {
        let mut val = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Minus, digest);
        format!("-{val:x}")
    } else {
        let mut val = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, digest);
        format!("{val:x}")
    }
}

pub fn offline_uuid(username: &str) -> Uuid {
    Uuid::new_v3(&Uuid::NAMESPACE_DNS, format!("OfflinePlayer:{username}").as_bytes())
}

pub fn format_uuid_with_dashes(hex: &str) -> Uuid {
    Uuid::parse_str(hex).unwrap_or_else(|_| {
        if hex.len() == 32 {
            Uuid::parse_str(&format!(
                "{}-{}-{}-{}-{}",
                &hex[0..8],
                &hex[8..12],
                &hex[12..16],
                &hex[16..20],
                &hex[20..32]
            ))
            .unwrap_or(Uuid::nil())
        } else {
            Uuid::nil()
        }
    })
}