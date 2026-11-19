use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DrmError {
    #[error("unauthorized playback request: {0}")]
    Unauthorized(String),

    #[error("decryption key expired at {expired_at}, current time {now}")]
    KeyExpired { expired_at: u64, now: u64 },

    #[error("session has been revoked: {0}")]
    SessionRevoked(String),

    #[error("cryptographic operation failed: {0}")]
    CryptoError(String),

    #[error("watermark decoding failed: {0}")]
    WatermarkError(String),

    #[error("invalid optical simulation parameter: {0}")]
    OpticalParamError(String),

    #[error("virtual machine / hypervisor environment detected: {0}")]
    HypervisorDetected(String),

    #[error("audio spread-spectrum watermark error: {0}")]
    AudioWatermarkError(String),

    #[error("short-window DCT watermark error: {0}")]
    DctWatermarkError(String),
}
