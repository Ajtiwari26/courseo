use crate::error::DrmError;

/// Number of bits in the short-window DCT chrominance subscriber ID payload.
pub const DCT_PAYLOAD_BITS: usize = 32;
/// Mid-frequency DCT coefficient indices (zig-zag 12..16) resilient to JPEG/H.264 quantization.
pub const MID_FREQ_START: usize = 12;
pub const MID_FREQ_END: usize = 16;
/// Chrominance DCT modulation strength.
pub const DCT_MODULATION_DELTA: f64 = 8.0;

pub struct DctWatermarkCodec;

impl DctWatermarkCodec {
    /// Embeds a 32-bit subscriber ID into 8x8 chrominance (Cb/Cr) DCT blocks by modulating
    /// mid-frequency coefficients at indices 12..16 across blocks.
    pub fn embed_8x8_chroma(blocks: &mut [[f64; 64]], subscriber_id: u32) -> Result<(), DrmError> {
        if blocks.len() < DCT_PAYLOAD_BITS {
            return Err(DrmError::DctWatermarkError(format!(
                "need at least {DCT_PAYLOAD_BITS} 8x8 DCT blocks, got {}",
                blocks.len()
            )));
        }

        for (block_idx, block) in blocks.iter_mut().enumerate() {
            let bit_idx = block_idx % DCT_PAYLOAD_BITS;
            let sign = if ((subscriber_id >> bit_idx) & 1) == 1 {
                1.0_f64
            } else {
                -1.0_f64
            };
            for coeff in &mut block[MID_FREQ_START..MID_FREQ_END] {
                *coeff += sign * DCT_MODULATION_DELTA;
            }
        }

        Ok(())
    }

    /// Blindly extracts the 32-bit subscriber ID and normalized confidence score (0.0..=1.0)
    /// from 8x8 chrominance DCT blocks.
    pub fn extract_8x8_chroma(blocks: &[[f64; 64]]) -> Result<(u32, f64), DrmError> {
        if blocks.len() < DCT_PAYLOAD_BITS {
            return Err(DrmError::DctWatermarkError(format!(
                "need at least {DCT_PAYLOAD_BITS} 8x8 DCT blocks, got {}",
                blocks.len()
            )));
        }

        let mut bit_sums = [0.0_f64; DCT_PAYLOAD_BITS];
        let mut bit_counts = [0_usize; DCT_PAYLOAD_BITS];

        for (block_idx, block) in blocks.iter().enumerate() {
            let bit_idx = block_idx % DCT_PAYLOAD_BITS;
            let mid_sum: f64 = block[MID_FREQ_START..MID_FREQ_END].iter().sum();
            bit_sums[bit_idx] += mid_sum;
            bit_counts[bit_idx] += 1;
        }

        let coeffs_per_block = (MID_FREQ_END - MID_FREQ_START) as f64;
        let mut recovered_id: u32 = 0;
        let mut confidence_acc = 0.0_f64;

        for bit_idx in 0..DCT_PAYLOAD_BITS {
            let sum = bit_sums[bit_idx];
            if sum >= 0.0 {
                recovered_id |= 1_u32 << bit_idx;
            }
            let expected_mag =
                (bit_counts[bit_idx] as f64) * coeffs_per_block * DCT_MODULATION_DELTA;
            let bit_conf = if expected_mag > 0.0 {
                (sum.abs() / expected_mag).min(1.0)
            } else {
                0.0
            };
            confidence_acc += bit_conf;
        }

        let confidence = confidence_acc / (DCT_PAYLOAD_BITS as f64);
        Ok((recovered_id, confidence))
    }
}
