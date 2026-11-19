use crate::error::DrmError;

const BLACKLISTED_RENDERERS: &[&str] = &[
    "llvmpipe",
    "vmware",
    "virtualbox",
    "qemu",
    "parallels",
    "angle (software",
    "microsoft basic render driver",
];

/// Maximum allowable timer jitter spike or spread in microseconds before flagging a hypervisor.
pub const MAX_ALLOWED_JITTER_US: u64 = 50;

pub struct HypervisorDetector;

impl HypervisorDetector {
    /// Returns `true` if the reported GPU renderer string matches a known virtualized
    /// or software-emulated graphics adapter.
    pub fn is_blacklisted_renderer(renderer: &str) -> bool {
        let normalized = renderer.to_ascii_lowercase();
        BLACKLISTED_RENDERERS
            .iter()
            .any(|pattern| normalized.contains(pattern))
    }

    /// Analyzes high-resolution timer latency samples (in microseconds) and returns `true`
    /// if hypervisor context-switch jitter spikes or spread exceed 50 microseconds.
    pub fn analyze_timing_jitter(samples: &[u64]) -> bool {
        if samples.len() < 2 {
            return false;
        }

        let min_val = samples.iter().copied().min().unwrap_or(0);
        let max_val = samples.iter().copied().max().unwrap_or(0);
        if max_val.saturating_sub(min_val) > MAX_ALLOWED_JITTER_US {
            return true;
        }

        let n = samples.len() as f64;
        let mean = samples.iter().map(|&s| s as f64).sum::<f64>() / n;
        let variance = samples
            .iter()
            .map(|&s| {
                let diff = (s as f64) - mean;
                diff * diff
            })
            .sum::<f64>()
            / n;

        variance.sqrt() > (MAX_ALLOWED_JITTER_US as f64)
    }

    /// Evaluates both GPU renderer identity and timer jitter telemetry.
    /// Returns `Err(DrmError::HypervisorDetected)` if a virtualized environment is detected.
    pub fn evaluate_environment(renderer: &str, timing_samples_us: &[u64]) -> Result<(), DrmError> {
        if Self::is_blacklisted_renderer(renderer) {
            return Err(DrmError::HypervisorDetected(format!(
                "blacklisted virtualized renderer detected: {renderer}"
            )));
        }

        if Self::analyze_timing_jitter(timing_samples_us) {
            return Err(DrmError::HypervisorDetected(
                "hypervisor timer jitter spike (>50us) detected".to_string(),
            ));
        }

        Ok(())
    }
}
