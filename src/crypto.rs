use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::DrmError;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub const KEY_ROTATION_INTERVAL_SECS: u64 = 30;
const AES_BLOCK_SIZE: usize = 16;
const CBCS_CRYPT_BLOCKS: usize = 9;
const CBCS_SKIP_BLOCKS: usize = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaybackTokenClaims {
    pub user_id: u32,
    pub client_ip: String,
    pub session_id: String,
    pub expiry_timestamp: u64,
    pub allowed_renditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignedPlaybackToken {
    pub claims: PlaybackTokenClaims,
    pub signature: Vec<u8>,
}

pub struct TokenAuthority {
    signing_key: SigningKey,
}

impl TokenAuthority {
    pub fn generate() -> Self {
        let mut csprng = rand::rngs::OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn issue_token(
        &self,
        claims: PlaybackTokenClaims,
    ) -> Result<SignedPlaybackToken, DrmError> {
        let payload = serde_json::to_vec(&claims)
            .map_err(|e| DrmError::CryptoError(format!("token serialization failed: {e}")))?;
        let signature = self.signing_key.sign(&payload);
        Ok(SignedPlaybackToken {
            claims,
            signature: signature.to_bytes().to_vec(),
        })
    }

    pub fn verify_token(
        verifying_key: &VerifyingKey,
        token: &SignedPlaybackToken,
        now_secs: u64,
    ) -> Result<PlaybackTokenClaims, DrmError> {
        if now_secs > token.claims.expiry_timestamp {
            return Err(DrmError::Unauthorized("playback token expired".to_string()));
        }
        let payload = serde_json::to_vec(&token.claims)
            .map_err(|e| DrmError::CryptoError(format!("token serialization failed: {e}")))?;
        let sig_bytes: [u8; 64] = token
            .signature
            .as_slice()
            .try_into()
            .map_err(|_| DrmError::Unauthorized("malformed signature length".to_string()))?;
        let signature = Signature::from_bytes(&sig_bytes);
        verifying_key
            .verify(&payload, &signature)
            .map_err(|_| DrmError::Unauthorized("invalid Ed25519 signature".to_string()))?;
        Ok(token.claims.clone())
    }
}

#[derive(Debug, Clone)]
pub struct EphemeralKey {
    pub key_id: u64,
    pub key_bytes: [u8; 16],
    pub issued_at: u64,
    pub expires_at: u64,
}

pub struct KeyLadder {
    keys: HashMap<u64, EphemeralKey>,
    next_id: u64,
}

impl Default for KeyLadder {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyLadder {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn rotate_key(&mut self, now_secs: u64) -> EphemeralKey {
        let mut key_bytes = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut key_bytes);
        let key = EphemeralKey {
            key_id: self.next_id,
            key_bytes,
            issued_at: now_secs,
            expires_at: now_secs + KEY_ROTATION_INTERVAL_SECS,
        };
        self.next_id += 1;
        self.keys.insert(key.key_id, key.clone());
        key
    }

    pub fn get_valid_key(&self, key_id: u64, now_secs: u64) -> Result<&EphemeralKey, DrmError> {
        let key = self
            .keys
            .get(&key_id)
            .ok_or_else(|| DrmError::Unauthorized(format!("unknown key_id {key_id}")))?;
        if now_secs > key.expires_at {
            return Err(DrmError::KeyExpired {
                expired_at: key.expires_at,
                now: now_secs,
            });
        }
        Ok(key)
    }
}

/// Encrypts a media chunk using ISO/IEC 23001-7 AES-128-CBCS pattern encryption (9 crypt : 1 skip).
pub fn encrypt_cbcs(key: &[u8; 16], iv: &[u8; 16], plaintext: &[u8]) -> Vec<u8> {
    let mut output = plaintext.to_vec();
    let total_blocks = output.len() / AES_BLOCK_SIZE;
    let pattern_len = CBCS_CRYPT_BLOCKS + CBCS_SKIP_BLOCKS;

    let mut enc = Aes128CbcEnc::new(key.into(), iv.into());
    for block_idx in 0..total_blocks {
        if block_idx % pattern_len < CBCS_CRYPT_BLOCKS {
            let start = block_idx * AES_BLOCK_SIZE;
            let block = &mut output[start..start + AES_BLOCK_SIZE];
            enc.encrypt_block_mut(block.into());
        }
    }
    output
}

/// Decrypts an AES-128-CBCS pattern-encrypted media chunk (9 crypt : 1 skip).
pub fn decrypt_cbcs(key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> Vec<u8> {
    let mut output = ciphertext.to_vec();
    let total_blocks = output.len() / AES_BLOCK_SIZE;
    let pattern_len = CBCS_CRYPT_BLOCKS + CBCS_SKIP_BLOCKS;

    let mut dec = Aes128CbcDec::new(key.into(), iv.into());
    for block_idx in 0..total_blocks {
        if block_idx % pattern_len < CBCS_CRYPT_BLOCKS {
            let start = block_idx * AES_BLOCK_SIZE;
            let block = &mut output[start..start + AES_BLOCK_SIZE];
            dec.decrypt_block_mut(block.into());
        }
    }
    output
}
