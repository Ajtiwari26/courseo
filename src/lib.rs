#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod anti_cam;
pub mod audio_watermark;
pub mod crypto;
pub mod dct_watermark;
pub mod error;
pub mod hardware_protection;
pub mod hypervisor_detector;
pub mod telemetry;
pub mod watermark;

pub use error::DrmError;
