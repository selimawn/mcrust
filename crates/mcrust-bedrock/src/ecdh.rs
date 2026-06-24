//! Bedrock handshake ECDH + AES-CTR batch encryption (gophertunnel-compatible).

use base64::Engine;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use p256::ecdsa::SigningKey;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::pkcs8::EncodePrivateKey;
use p256::PublicKey;
use rand::RngCore;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
struct SaltClaims {
    salt: String,
}

pub struct BedrockSessionCrypto {
    pub server_signing: SigningKey,
    pub salt: [u8; 16],
    pub key_bytes: Option<[u8; 32]>,
}

impl BedrockSessionCrypto {
    pub fn generate() -> Self {
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        Self {
            server_signing: SigningKey::random(&mut rand::thread_rng()),
            salt,
            key_bytes: None,
        }
    }

    pub fn server_public_key_b64(&self) -> String {
        let point = self.server_signing.verifying_key().to_encoded_point(false);
        base64::engine::general_purpose::STANDARD.encode(point.as_bytes())
    }

    /// Build ServerToClientHandshake JWT (ES384, x5u = server pubkey DER).
    pub fn server_handshake_jwt(&self) -> Result<String, String> {
        let x5u = self.server_public_key_b64();
        let mut header = Header::new(Algorithm::ES384);
        header.x5u = Some(x5u);
        let claims = SaltClaims {
            salt: base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(self.salt),
        };
        let pkcs8 = self
            .server_signing
            .to_pkcs8_der()
            .map_err(|e| e.to_string())?;
        let key = EncodingKey::from_ec_der(pkcs8.as_bytes());
        encode(&header, &claims, &key).map_err(|e| e.to_string())
    }

    /// Derive AES key after client public key known (post-login).
    pub fn derive_key_from_client_public_key(&mut self, client_pub: &PublicKey) -> [u8; 32] {
        let shared = p256::ecdh::diffie_hellman(
            self.server_signing.as_nonzero_scalar(),
            client_pub.as_affine(),
        );
        let mut secret = shared.raw_secret_bytes().to_vec();
        while secret.len() < 48 {
            secret.insert(0, 0);
        }
        secret.truncate(48);
        let mut hasher = Sha256::new();
        hasher.update(self.salt);
        hasher.update(&secret);
        let key: [u8; 32] = hasher.finalize().into();
        self.key_bytes = Some(key);
        key
    }
}

pub fn parse_client_public_key_from_x5u(b64: &str) -> Result<PublicKey, String> {
    let der = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(b64))
        .map_err(|e| e.to_string())?;
    PublicKey::from_sec1_bytes(&der).map_err(|e| e.to_string())
}

pub fn encrypt_batch(payload: &[u8], key: &[u8; 32]) -> Vec<u8> {
    use aes::Aes256;
    use cipher::{KeyIvInit, StreamCipher};
    use ctr::Ctr128BE;
    let mut iv = [0u8; 16];
    iv[..12].copy_from_slice(&key[..12]);
    iv[15] = 2;
    let mut cipher = Ctr128BE::<Aes256>::new(key.into(), &iv.into());
    let mut out = payload.to_vec();
    cipher.apply_keystream(&mut out);
    out
}

pub fn decrypt_batch(payload: &[u8], key: &[u8; 32]) -> Vec<u8> {
    encrypt_batch(payload, key)
}