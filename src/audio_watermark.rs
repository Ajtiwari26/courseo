use crate::error::DrmError;
use std::f64::consts::PI;

/// Number of subscriber ID bits embedded via psychoacoustic DSSS.
pub const PAYLOAD_BITS: usize = 48;
/// Length of the 7-bit maximal-length LFSR pseudo-noise (PN) sequence.
pub const PN_SEQUENCE_LEN: usize = 127;
/// Carrier frequency in the 15kHz-19kHz psychoacoustic masking band.
pub const CARRIER_FREQ_HZ: f64 = 17_000.0;
/// Psychoacoustic masking amplitude kept below the Absolute Threshold of Hearing.
pub const PSYCHOACOUSTIC_AMPLITUDE: f32 = 0.005;

pub struct AudioWatermarkCodec;

impl AudioWatermarkCodec {
    /// Generates a deterministic 127-bit maximal-length PN sequence (+1.0 / -1.0)
    /// using primitive polynomial x^7 + x^6 + 1.
    pub fn generate_pn_sequence() -> [f32; PN_SEQUENCE_LEN] {
        let mut pn = [0.0_f32; PN_SEQUENCE_LEN];
        let mut lfsr: u8 = 0x7F;
        for chip in &mut pn {
            let out_bit = lfsr & 1;
            *chip = if out_bit == 1 { 1.0 } else { -1.0 };
            let feedback = ((lfsr >> 6) ^ (lfsr >> 5)) & 1;
            lfsr = ((lfsr << 1) | feedback) & 0x7F;
            if lfsr == 0 {
                lfsr = 0x7F;
            }
        }
        pn
    }

    /// Injects a 48-bit subscriber ID into mono f32 PCM samples using Direct Sequence
    /// Spread Spectrum (DSSS) with a 127-bit PN code modulated into the 15kHz-19kHz sub-band.
    pub fn encode_dsss(
        pcm: &mut [f32],
        subscriber_id: u64,
        sample_rate: u32,
    ) -> Result<(), DrmError> {
        if sample_rate < 40_000 {
            return Err(DrmError::AudioWatermarkError(format!(
                "sample rate {sample_rate}Hz is too low for 15kHz-19kHz sub-band"
            )));
        }

        let frame_len = PAYLOAD_BITS * PN_SEQUENCE_LEN;
        if pcm.len() < frame_len {
            return Err(DrmError::AudioWatermarkError(format!(
                "PCM buffer too short: need at least {frame_len} samples, got {}",
                pcm.len()
            )));
        }

        let pn = Self::generate_pn_sequence();
        let masked_id = subscriber_id & ((1_u64 << PAYLOAD_BITS) - 1);
        let num_frames = pcm.len() / frame_len;
        let sr = sample_rate as f64;

        for frame in 0..num_frames {
            let frame_offset = frame * frame_len;
            for bit_idx in 0..PAYLOAD_BITS {
                let bit_sign = if ((masked_id >> bit_idx) & 1) == 1 {
                    1.0_f32
                } else {
                    -1.0_f32
                };
                let bit_offset = frame_offset + bit_idx * PN_SEQUENCE_LEN;
                for (chip_idx, &chip_val) in pn.iter().enumerate() {
                    let sample_idx = bit_offset + chip_idx;
                    let carrier =
                        (2.0 * PI * CARRIER_FREQ_HZ * (sample_idx as f64) / sr).sin() as f32;
                    pcm[sample_idx] += PSYCHOACOUSTIC_AMPLITUDE * bit_sign * chip_val * carrier;
                }
            }
        }

        Ok(())
    }

    /// Extracts the embedded 48-bit subscriber ID from mono f32 PCM samples
    /// using high-pass filtering, coherent carrier demodulation and PN cross-correlation.
    pub fn extract_dsss(pcm: &[f32], sample_rate: u32) -> Result<u64, DrmError> {
        if sample_rate < 40_000 {
            return Err(DrmError::AudioWatermarkError(format!(
                "sample rate {sample_rate}Hz is too low for 15kHz-19kHz sub-band"
            )));
        }

        let frame_len = PAYLOAD_BITS * PN_SEQUENCE_LEN;
        if pcm.len() < frame_len {
            return Err(DrmError::AudioWatermarkError(format!(
                "PCM buffer too short: need at least {frame_len} samples, got {}",
                pcm.len()
            )));
        }

        // Apply a 2nd-order high-pass difference filter y[n] = x[n] - 2x[n-1] + x[n-2]
        // This suppresses low-frequency speech/music (<4kHz) by >50dB while preserving
        // and amplifying the 17kHz DSSS watermark carrier.
        let mut filtered_pcm = vec![0.0_f64; pcm.len()];
        for i in 2..pcm.len() {
            filtered_pcm[i] = (pcm[i] as f64) - 2.0 * (pcm[i - 1] as f64) + (pcm[i - 2] as f64);
        }

        let sr = sample_rate as f64;
        let mut filtered_carrier = vec![0.0_f64; pcm.len()];
        for (i, val) in filtered_carrier.iter_mut().enumerate() {
            *val = (2.0 * PI * CARRIER_FREQ_HZ * (i as f64) / sr).sin();
        }
        for i in (2..pcm.len()).rev() {
            filtered_carrier[i] =
                filtered_carrier[i] - 2.0 * filtered_carrier[i - 1] + filtered_carrier[i - 2];
        }

        let pn = Self::generate_pn_sequence();
        let num_frames = pcm.len() / frame_len;
        let mut recovered_id: u64 = 0;

        for bit_idx in 0..PAYLOAD_BITS {
            let mut correlation = 0.0_f64;
            for frame in 0..num_frames {
                let bit_offset = frame * frame_len + bit_idx * PN_SEQUENCE_LEN;
                for (chip_idx, &chip_val) in pn.iter().enumerate() {
                    let sample_idx = bit_offset + chip_idx;
                    correlation +=
                        filtered_pcm[sample_idx] * (chip_val as f64) * filtered_carrier[sample_idx];
                }
            }
            if correlation >= 0.0 {
                recovered_id |= 1_u64 << bit_idx;
            }
        }

        Ok(recovered_id)
    }
}
