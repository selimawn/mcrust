use aes::Aes128;
use cfb_mode::cipher::{KeyIvInit, StreamCipher};
use cfb_mode::{BufDecryptor, BufEncryptor};
use rand::RngCore;
use rsa::pkcs8::EncodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

type Aes128Enc = BufEncryptor<Aes128>;
type Aes128Dec = BufDecryptor<Aes128>;

pub struct ServerKeys {
    pub public_key_der: Vec<u8>,
    private_key: RsaPrivateKey,
}

impl ServerKeys {
    pub fn generate() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 1024)?;
        let public_key_der = private_key
            .to_public_key()
            .to_public_key_der()
            .map_err(|e| e.to_string())?
            .into_vec();
        Ok(Self {
            public_key_der,
            private_key,
        })
    }

    pub fn decrypt_pair(
        &self,
        encrypted_secret: &[u8],
        encrypted_token: &[u8],
        verify_token: &[u8],
    ) -> Result<[u8; 16], String> {
        let padding = Pkcs1v15Encrypt;
        let secret = self
            .private_key
            .decrypt(padding, encrypted_secret)
            .map_err(|e| e.to_string())?;
        let token = self
            .private_key
            .decrypt(padding, encrypted_token)
            .map_err(|e| e.to_string())?;
        if token != verify_token {
            return Err("verify token mismatch".into());
        }
        if secret.len() != 16 {
            return Err("bad secret length".into());
        }
        let mut out = [0u8; 16];
        out.copy_from_slice(&secret);
        Ok(out)
    }
}

pub struct AesStream {
    enc: Aes128Enc,
    dec: Aes128Dec,
}

impl AesStream {
    pub fn from_secret(secret: [u8; 16]) -> Self {
        let key = secret;
        let iv = secret;
        Self {
            enc: Aes128Enc::new(&key.into(), &iv.into()),
            dec: Aes128Dec::new(&key.into(), &iv.into()),
        }
    }

    pub fn encrypt(&mut self, buf: &mut [u8]) {
        self.enc.encrypt(buf);
    }

    pub fn decrypt(&mut self, buf: &mut [u8]) {
        self.dec.decrypt(buf);
    }
}

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut v = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut v);
    v
}