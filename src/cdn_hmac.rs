use crate::error::DrmError;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const HMAC_BLOCK_SIZE: usize = 64;

#[derive(Debug, Clone)]
pub struct CdnTokenSigner {
    secret_key: Vec<u8>,
    enforce_ip_binding: bool,
}

impl CdnTokenSigner {
    pub fn new(secret_key: impl Into<Vec<u8>>, enforce_ip_binding: bool) -> Self {
        Self {
            secret_key: secret_key.into(),
            enforce_ip_binding,
        }
    }

    pub fn sign_url_params(&self, url_path: &str, client_ip: &str, expires_at: u64) -> String {
        let payload = build_canonical_payload(url_path, expires_at, client_ip);
        let mac = hmac_sha256(&self.secret_key, payload.as_bytes());
        let hmac_hex = to_hex(&mac);
        format!("token=exp={expires_at}&ip={client_ip}&hmac={hmac_hex}")
    }

    pub fn validate_token(
        &self,
        url_path: &str,
        query_params: &str,
        client_ip: &str,
        current_time: u64,
    ) -> Result<(), DrmError> {
        let normalized = query_params.strip_prefix("token=").unwrap_or(query_params);
        let mut fields: HashMap<&str, &str> = HashMap::new();

        for pair in normalized.split('&') {
            if pair.is_empty() {
                continue;
            }
            let Some((k, v)) = pair.split_once('=') else {
                return Err(DrmError::CdnTokenError(
                    "malformed token query parameter".to_string(),
                ));
            };
            fields.insert(k, v);
        }

        let exp_str = fields.get("exp").ok_or_else(|| {
            DrmError::CdnTokenError("missing 'exp' parameter in CDN token".to_string())
        })?;
        let expires_at: u64 = exp_str.parse().map_err(|_| {
            DrmError::CdnTokenError("invalid 'exp' timestamp in CDN token".to_string())
        })?;

        if current_time > expires_at {
            return Err(DrmError::CdnTokenError(format!(
                "CDN token expired at {expires_at}, current time {current_time}"
            )));
        }

        let token_ip = fields.get("ip").copied().ok_or_else(|| {
            DrmError::CdnTokenError("missing 'ip' parameter in CDN token".to_string())
        })?;

        if self.enforce_ip_binding && !ip_matches(token_ip, client_ip) {
            return Err(DrmError::CdnTokenError(format!(
                "CDN token IP mismatch: token bound to {token_ip}, request from {client_ip}"
            )));
        }

        let provided_hmac_hex = fields.get("hmac").copied().ok_or_else(|| {
            DrmError::CdnTokenError("missing 'hmac' signature in CDN token".to_string())
        })?;
        let provided_mac = from_hex(provided_hmac_hex).ok_or_else(|| {
            DrmError::CdnTokenError("malformed hex in CDN token 'hmac'".to_string())
        })?;

        let payload = build_canonical_payload(url_path, expires_at, token_ip);
        let expected_mac = hmac_sha256(&self.secret_key, payload.as_bytes());

        if !constant_time_eq(&provided_mac, &expected_mac) {
            return Err(DrmError::CdnTokenError(
                "CDN token HMAC signature verification failed".to_string(),
            ));
        }

        Ok(())
    }
}

fn build_canonical_payload(url_path: &str, expires_at: u64, client_ip: &str) -> String {
    format!("{url_path}|exp={expires_at}|ip={client_ip}")
}

fn ip_matches(token_ip: &str, client_ip: &str) -> bool {
    if token_ip == client_ip {
        return true;
    }
    if let Some(prefix) = token_ip.strip_suffix("/24") {
        let token_octets: Vec<&str> = prefix.split('.').collect();
        let client_octets: Vec<&str> = client_ip.split('.').collect();
        if token_octets.len() == 4 && client_octets.len() == 4 {
            return token_octets[0..3] == client_octets[0..3];
        }
    }
    false
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut key_block = [0u8; HMAC_BLOCK_SIZE];
    if key.len() > HMAC_BLOCK_SIZE {
        let digest = Sha256::digest(key);
        key_block[..32].copy_from_slice(&digest);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut ipad = [0x36u8; HMAC_BLOCK_SIZE];
    let mut opad = [0x5cu8; HMAC_BLOCK_SIZE];
    for i in 0..HMAC_BLOCK_SIZE {
        ipad[i] ^= key_block[i];
        opad[i] ^= key_block[i];
    }

    let mut inner = Sha256::new();
    inner.update(ipad);
    inner.update(message);
    let inner_hash = inner.finalize();

    let mut outer = Sha256::new();
    outer.update(opad);
    outer.update(inner_hash);
    let outer_hash = outer.finalize();

    let mut out = [0u8; 32];
    out.copy_from_slice(&outer_hash);
    out
}

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn from_hex(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let bytes = hex.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for &[hi_byte, lo_byte] in bytes.as_chunks::<2>().0 {
        let hi = hex_nibble(hi_byte)?;
        let lo = hex_nibble(lo_byte)?;
        out.push((hi << 4) | lo);
    }
    Some(out)
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
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
