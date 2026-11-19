use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::RwLock;

pub const DEFAULT_KEY_DURATION_SECS: u64 = 30;
pub const DEFAULT_GRACE_PERIOD_SECS: u64 = 15;
pub const DEFAULT_PREISSUE_LEAD_SECS: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NextKeyPreview {
    pub key_id: u64,
    pub content_key: [u8; 16],
    pub valid_from_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyDecision {
    ValidKey {
        content_key: [u8; 16],
        expires_in_secs: u64,
        next_key_preview: Option<NextKeyPreview>,
    },
    ExpiredKeyGraceAllowed {
        content_key: [u8; 16],
        warning: &'static str,
    },
    RejectedExpired,
}

#[derive(Debug)]
pub struct SlidingWindowKeyManager {
    epoch_start_secs: u64,
    key_duration_secs: u64,
    grace_period_secs: u64,
    preissue_lead_secs: u64,
    master_secret: [u8; 32],
    key_cache: RwLock<HashMap<u64, [u8; 16]>>,
}

impl SlidingWindowKeyManager {
    pub fn new(
        epoch_start_secs: u64,
        key_duration_secs: u64,
        grace_period_secs: u64,
        preissue_lead_secs: u64,
        master_secret: [u8; 32],
    ) -> Self {
        Self {
            epoch_start_secs,
            key_duration_secs: key_duration_secs.max(1),
            grace_period_secs,
            preissue_lead_secs,
            master_secret,
            key_cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn with_defaults(epoch_start_secs: u64, master_secret: [u8; 32]) -> Self {
        Self::new(
            epoch_start_secs,
            DEFAULT_KEY_DURATION_SECS,
            DEFAULT_GRACE_PERIOD_SECS,
            DEFAULT_PREISSUE_LEAD_SECS,
            master_secret,
        )
    }

    pub fn active_key_id_at(&self, timestamp_epoch_secs: u64) -> Option<u64> {
        if timestamp_epoch_secs < self.epoch_start_secs {
            return None;
        }
        Some((timestamp_epoch_secs - self.epoch_start_secs) / self.key_duration_secs)
    }

    pub fn key_window(&self, key_id: u64) -> Option<(u64, u64)> {
        let offset = key_id.checked_mul(self.key_duration_secs)?;
        let start = self.epoch_start_secs.checked_add(offset)?;
        let end = start.checked_add(self.key_duration_secs)?;
        Some((start, end))
    }

    fn derive_or_get_key(&self, key_id: u64) -> [u8; 16] {
        if let Ok(guard) = self.key_cache.read() {
            if let Some(&cached) = guard.get(&key_id) {
                return cached;
            }
        }

        let mut hasher = Sha256::new();
        hasher.update(self.master_secret);
        hasher.update(b"courseo-sliding-window-kms-v1");
        hasher.update(key_id.to_be_bytes());
        let digest = hasher.finalize();

        let mut key = [0u8; 16];
        key.copy_from_slice(&digest[..16]);

        if let Ok(mut guard) = self.key_cache.write() {
            guard.insert(key_id, key);
        }

        key
    }

    pub fn evaluate_key_request(
        &self,
        client_timestamp_epoch_secs: u64,
        requested_key_id: u64,
    ) -> KeyDecision {
        let Some((window_start, window_end)) = self.key_window(requested_key_id) else {
            return KeyDecision::RejectedExpired;
        };

        // Case 1: Client is inside the primary validity window for requested_key_id.
        if client_timestamp_epoch_secs >= window_start && client_timestamp_epoch_secs < window_end {
            let expires_in_secs = window_end - client_timestamp_epoch_secs;
            let content_key = self.derive_or_get_key(requested_key_id);

            let next_key_preview = if expires_in_secs <= self.preissue_lead_secs {
                requested_key_id
                    .checked_add(1)
                    .map(|next_id| NextKeyPreview {
                        key_id: next_id,
                        content_key: self.derive_or_get_key(next_id),
                        valid_from_secs: window_end,
                    })
            } else {
                None
            };

            return KeyDecision::ValidKey {
                content_key,
                expires_in_secs,
                next_key_preview,
            };
        }

        // Case 2: Client requests lookahead key N+1 within preissue_lead_secs before window_start.
        if client_timestamp_epoch_secs < window_start {
            let lead_delta = window_start - client_timestamp_epoch_secs;
            if lead_delta <= self.preissue_lead_secs {
                let content_key = self.derive_or_get_key(requested_key_id);
                let expires_in_secs = window_end - client_timestamp_epoch_secs;
                return KeyDecision::ValidKey {
                    content_key,
                    expires_in_secs,
                    next_key_preview: None,
                };
            }
            return KeyDecision::RejectedExpired;
        }

        // Case 3: Key expired, check sliding grace window for NTP drift / clock skew.
        let grace_deadline = window_end.saturating_add(self.grace_period_secs);
        if client_timestamp_epoch_secs <= grace_deadline {
            let content_key = self.derive_or_get_key(requested_key_id);
            return KeyDecision::ExpiredKeyGraceAllowed {
                content_key,
                warning: "clock_skew_detected",
            };
        }

        KeyDecision::RejectedExpired
    }
}
