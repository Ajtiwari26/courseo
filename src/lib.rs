#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod anti_cam;
pub mod audio_watermark;
pub mod cdn_hmac;
pub mod crypto;
pub mod dct_watermark;
pub mod error;
pub mod fairplay_ksm;
pub mod hardware_protection;
pub mod hdcp_degradation;
pub mod hypervisor_detector;
pub mod sliding_window_kms;
pub mod telemetry;
pub mod watermark;

pub use error::DrmError;
