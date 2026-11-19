use crate::error::DrmError;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const SPC_MAGIC: &[u8; 4] = b"SPC1";
const CKC_MAGIC: &[u8; 4] = b"CKC1";
const SPC_MIN_LEN: usize = 4 + 4 + 8 + 16 + 16 + 2 + 16;
const CKC_MIN_LEN: usize = 4 + 4 + 8 + 16 + 16 + 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpcPayload {
    pub version: u32,
    pub asset_id: String,
    pub session_key: [u8; 16],
    pub iv: [u8; 16],
    pub timestamp_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CkcPayload {
    pub version: u32,
    pub asset_id: String,
    pub iv: [u8; 16],
    pub encrypted_content_key: [u8; 16],
    pub license_ttl_secs: u64,
}

#[derive(Debug, Clone)]
pub struct FairplayKsm {
    application_secret_key: [u8; 16],
    license_ttl_secs: u64,
    asset_keys: HashMap<String, [u8; 16]>,
}

impl FairplayKsm {
    pub fn new(application_secret_key: [u8; 16], license_ttl_secs: u64) -> Self {
        Self {
            application_secret_key,
            license_ttl_secs,
            asset_keys: HashMap::new(),
        }
    }

    pub fn register_asset_key(&mut self, asset_id: impl Into<String>, content_key: [u8; 16]) {
        self.asset_keys.insert(asset_id.into(), content_key);
    }

    pub fn build_spc(
        version: u32,
        asset_id: &str,
        session_key: [u8; 16],
        iv: [u8; 16],
        timestamp_secs: u64,
        application_secret_key: &[u8; 16],
    ) -> Result<Vec<u8>, DrmError> {
        let asset_bytes = asset_id.as_bytes();
        let asset_len = u16::try_from(asset_bytes.len()).map_err(|_| {
            DrmError::FairplayKsmError("asset_id exceeds maximum 65535 byte length".to_string())
        })?;

        let mask = derive_keystream(application_secret_key, &iv, b"FPS-SPC-SESSION-MASK");
        let wrapped_session_key = xor_block(&session_key, &mask);

        let mut buf = Vec::with_capacity(SPC_MIN_LEN + asset_bytes.len());
        buf.extend_from_slice(SPC_MAGIC);
        buf.extend_from_slice(&version.to_be_bytes());
        buf.extend_from_slice(&timestamp_secs.to_be_bytes());
        buf.extend_from_slice(&iv);
        buf.extend_from_slice(&wrapped_session_key);
        buf.extend_from_slice(&asset_len.to_be_bytes());
        buf.extend_from_slice(asset_bytes);

        let mac = compute_spc_mac(application_secret_key, &buf);
        buf.extend_from_slice(&mac);
        Ok(buf)
    }

    pub fn parse_spc(&self, spc_bytes: &[u8]) -> Result<SpcPayload, DrmError> {
        if spc_bytes.len() < SPC_MIN_LEN {
            return Err(DrmError::FairplayKsmError(
                "malformed SPC payload: truncated header".to_string(),
            ));
        }

        if &spc_bytes[0..4] != SPC_MAGIC {
            return Err(DrmError::FairplayKsmError(
                "malformed SPC payload: invalid magic header".to_string(),
            ));
        }

        let body_len = spc_bytes.len() - 16;
        let body = &spc_bytes[..body_len];
        let provided_mac = &spc_bytes[body_len..];
        let expected_mac = compute_spc_mac(&self.application_secret_key, body);

        if !constant_time_eq(provided_mac, &expected_mac) {
            return Err(DrmError::FairplayKsmError(
                "SPC authentication failed: invalid D-Key / ASK signature".to_string(),
            ));
        }

        let version = u32::from_be_bytes([body[4], body[5], body[6], body[7]]);
        let timestamp_secs = u64::from_be_bytes([
            body[8], body[9], body[10], body[11], body[12], body[13], body[14], body[15],
        ]);

        let mut iv = [0u8; 16];
        iv.copy_from_slice(&body[16..32]);

        let mut wrapped_session_key = [0u8; 16];
        wrapped_session_key.copy_from_slice(&body[32..48]);

        let asset_len = u16::from_be_bytes([body[48], body[49]]) as usize;
        if body.len() != 50 + asset_len {
            return Err(DrmError::FairplayKsmError(
                "malformed SPC payload: asset_id length mismatch".to_string(),
            ));
        }

        let asset_id = std::str::from_utf8(&body[50..50 + asset_len])
            .map_err(|_| {
                DrmError::FairplayKsmError(
                    "malformed SPC payload: asset_id is not valid UTF-8".to_string(),
                )
            })?
            .to_string();

        let mask = derive_keystream(&self.application_secret_key, &iv, b"FPS-SPC-SESSION-MASK");
        let session_key = xor_block(&wrapped_session_key, &mask);

        Ok(SpcPayload {
            version,
            asset_id,
            session_key,
            iv,
            timestamp_secs,
        })
    }

    pub fn process_spc(&self, spc_bytes: &[u8], now_secs: u64) -> Result<Vec<u8>, DrmError> {
        let spc = self.parse_spc(spc_bytes)?;

        let expires_at = spc.timestamp_secs.saturating_add(self.license_ttl_secs);
        if now_secs > expires_at {
            return Err(DrmError::FairplayKsmError(format!(
                "SPC license expired at {expires_at}, current time {now_secs}"
            )));
        }

        let content_key = self.asset_keys.get(&spc.asset_id).ok_or_else(|| {
            DrmError::FairplayKsmError(format!("unknown FairPlay asset_id: {}", spc.asset_id))
        })?;

        let wrap_stream = derive_keystream(&spc.session_key, &spc.iv, b"FPS-CKC-CONTENT-WRAP");
        let encrypted_content_key = xor_block(content_key, &wrap_stream);

        let asset_bytes = spc.asset_id.as_bytes();
        let asset_len = u16::try_from(asset_bytes.len()).map_err(|_| {
            DrmError::FairplayKsmError("asset_id exceeds maximum 65535 byte length".to_string())
        })?;

        let mut ckc = Vec::with_capacity(CKC_MIN_LEN + asset_bytes.len());
        ckc.extend_from_slice(CKC_MAGIC);
        ckc.extend_from_slice(&spc.version.to_be_bytes());
        ckc.extend_from_slice(&self.license_ttl_secs.to_be_bytes());
        ckc.extend_from_slice(&spc.iv);
        ckc.extend_from_slice(&encrypted_content_key);
        ckc.extend_from_slice(&asset_len.to_be_bytes());
        ckc.extend_from_slice(asset_bytes);

        Ok(ckc)
    }

    pub fn unwrap_ckc(
        ckc_bytes: &[u8],
        session_key: &[u8; 16],
    ) -> Result<(CkcPayload, [u8; 16]), DrmError> {
        if ckc_bytes.len() < CKC_MIN_LEN {
            return Err(DrmError::FairplayKsmError(
                "malformed CKC payload: truncated header".to_string(),
            ));
        }

        if &ckc_bytes[0..4] != CKC_MAGIC {
            return Err(DrmError::FairplayKsmError(
                "malformed CKC payload: invalid magic header".to_string(),
            ));
        }

        let version = u32::from_be_bytes([ckc_bytes[4], ckc_bytes[5], ckc_bytes[6], ckc_bytes[7]]);
        let license_ttl_secs = u64::from_be_bytes([
            ckc_bytes[8],
            ckc_bytes[9],
            ckc_bytes[10],
            ckc_bytes[11],
            ckc_bytes[12],
            ckc_bytes[13],
            ckc_bytes[14],
            ckc_bytes[15],
        ]);

        let mut iv = [0u8; 16];
        iv.copy_from_slice(&ckc_bytes[16..32]);

        let mut encrypted_content_key = [0u8; 16];
        encrypted_content_key.copy_from_slice(&ckc_bytes[32..48]);

        let asset_len = u16::from_be_bytes([ckc_bytes[48], ckc_bytes[49]]) as usize;
        if ckc_bytes.len() != 50 + asset_len {
            return Err(DrmError::FairplayKsmError(
                "malformed CKC payload: asset_id length mismatch".to_string(),
            ));
        }

        let asset_id = std::str::from_utf8(&ckc_bytes[50..50 + asset_len])
            .map_err(|_| {
                DrmError::FairplayKsmError(
                    "malformed CKC payload: asset_id is not valid UTF-8".to_string(),
                )
            })?
            .to_string();

        let wrap_stream = derive_keystream(session_key, &iv, b"FPS-CKC-CONTENT-WRAP");
        let content_key = xor_block(&encrypted_content_key, &wrap_stream);

        let payload = CkcPayload {
            version,
            asset_id,
            iv,
            encrypted_content_key,
            license_ttl_secs,
        };

        Ok((payload, content_key))
    }
}

fn derive_keystream(key: &[u8; 16], iv: &[u8; 16], domain: &[u8]) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(key);
    hasher.update(iv);
    hasher.update(domain);
    let digest = hasher.finalize();
    let mut out = [0u8; 16];
    out.copy_from_slice(&digest[..16]);
    out
}

fn compute_spc_mac(ask: &[u8; 16], body: &[u8]) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(ask);
    hasher.update(b"FPS-SPC-MAC-V1");
    hasher.update(body);
    let digest = hasher.finalize();
    let mut mac = [0u8; 16];
    mac.copy_from_slice(&digest[..16]);
    mac
}

fn xor_block(a: &[u8; 16], b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = a[i] ^ b[i];
    }
    out
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
