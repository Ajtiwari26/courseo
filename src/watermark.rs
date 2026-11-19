use crate::error::DrmError;

/// 64-bit forensic payload: UserID (32) || SubnetIP (16) || Timestamp (16)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WatermarkPayload {
    pub user_id: u32,
    pub subnet_ip: u16,
    pub timestamp: u16,
}

impl WatermarkPayload {
    pub fn pack(&self) -> u64 {
        ((self.user_id as u64) << 32) | ((self.subnet_ip as u64) << 16) | (self.timestamp as u64)
    }

    pub fn unpack(raw: u64) -> Self {
        Self {
            user_id: (raw >> 32) as u32,
            subnet_ip: ((raw >> 16) & 0xFFFF) as u16,
            timestamp: (raw & 0xFFFF) as u16,
        }
    }
}

/// Systematic BCH(15, 5, t=3) block codec applied across 16-bit/4-bit nibbles + repetition framing.
/// Each 64-bit payload is split into 16 four-bit symbols, each encoded into a 15-bit BCH codeword
/// (240 bits per cycle), repeated across `REPETITION_CYCLES` for resilience up to 15%+ bit-flip noise.
pub const BCH_CODEWORD_BITS: usize = 15;
pub const PAYLOAD_NIBBLES: usize = 16;
pub const SINGLE_CYCLE_BITS: usize = BCH_CODEWORD_BITS * PAYLOAD_NIBBLES; // 240 bits
pub const REPETITION_CYCLES: usize = 5;

// Generator polynomial for BCH(15, 5) with d_min = 7 (corrects up to t = 3 bit errors per 15-bit block):
// g(x) = x^10 + x^8 + x^5 + x^4 + x^2 + x + 1 = 0b10100110111 (0x537)
const BCH_15_5_GEN: u16 = 0b101_0011_0111;

fn encode_bch15_4(nibble: u8) -> u16 {
    let data = ((nibble & 0x0F) as u16) << 10;
    let mut rem = data;
    for shift in (0..5).rev() {
        if (rem & (1 << (shift + 10))) != 0 {
            rem ^= BCH_15_5_GEN << shift;
        }
    }
    data | (rem & 0x03FF)
}

fn decode_bch15_4(received: u16) -> u8 {
    let mut best_nibble = 0u8;
    let mut best_dist = u32::MAX;
    for candidate in 0u8..16 {
        let codeword = encode_bch15_4(candidate);
        let dist = (codeword ^ (received & 0x7FFF)).count_ones();
        if dist < best_dist {
            best_dist = dist;
            best_nibble = candidate;
        }
    }
    best_nibble
}

pub struct WatermarkCodec;

impl WatermarkCodec {
    /// Encodes a 64-bit payload into a robust binary sequence for A/B segment mapping.
    pub fn encode(payload: WatermarkPayload) -> Vec<u8> {
        let packed = payload.pack();
        let mut single_cycle = Vec::with_capacity(SINGLE_CYCLE_BITS);

        for nibble_idx in (0..PAYLOAD_NIBBLES).rev() {
            let nibble = ((packed >> (nibble_idx * 4)) & 0x0F) as u8;
            let cw = encode_bch15_4(nibble);
            for bit_pos in (0..BCH_CODEWORD_BITS).rev() {
                single_cycle.push(((cw >> bit_pos) & 1) as u8);
            }
        }

        let mut full_stream = Vec::with_capacity(SINGLE_CYCLE_BITS * REPETITION_CYCLES);
        for _ in 0..REPETITION_CYCLES {
            full_stream.extend_from_slice(&single_cycle);
        }
        full_stream
    }

    /// Blind forensic extractor: recovers the 64-bit payload from noisy A/B segment detections
    /// (tolerates >= 15% bit-flip corruption with near-100% accuracy).
    pub fn extract(detected_bits: &[u8]) -> Result<(WatermarkPayload, f64), DrmError> {
        if detected_bits.len() < SINGLE_CYCLE_BITS {
            return Err(DrmError::WatermarkError(format!(
                "insufficient segments: got {}, need at least {}",
                detected_bits.len(),
                SINGLE_CYCLE_BITS
            )));
        }

        let cycles = detected_bits.len() / SINGLE_CYCLE_BITS;
        let mut voted_cycle = vec![0u8; SINGLE_CYCLE_BITS];
        let mut agreement_sum = 0.0f64;

        for pos in 0..SINGLE_CYCLE_BITS {
            let mut ones = 0usize;
            for c in 0..cycles {
                if detected_bits[c * SINGLE_CYCLE_BITS + pos] != 0 {
                    ones += 1;
                }
            }
            let zeros = cycles - ones;
            if ones >= zeros {
                voted_cycle[pos] = 1;
                agreement_sum += ones as f64 / cycles as f64;
            } else {
                voted_cycle[pos] = 0;
                agreement_sum += zeros as f64 / cycles as f64;
            }
        }

        let mut packed = 0u64;
        for nibble_idx in 0..PAYLOAD_NIBBLES {
            let mut cw = 0u16;
            for bit_pos in 0..BCH_CODEWORD_BITS {
                let bit = voted_cycle[nibble_idx * BCH_CODEWORD_BITS + bit_pos] as u16;
                cw = (cw << 1) | bit;
            }
            let nibble = decode_bch15_4(cw) as u64;
            packed = (packed << 4) | nibble;
        }

        let confidence = agreement_sum / (SINGLE_CYCLE_BITS as f64);
        Ok((WatermarkPayload::unpack(packed), confidence))
    }

    /// Synthesizes an HLS `.m3u8` playlist stitching Variant A (0) and Variant B (1) 2-second segments.
    pub fn synthesize_m3u8(base_url: &str, payload: WatermarkPayload) -> String {
        let bits = Self::encode(payload);
        let mut manifest = String::from(
            "#EXTM3U\n#EXT-X-VERSION:6\n#EXT-X-TARGETDURATION:2\n#EXT-X-MEDIA-SEQUENCE:0\n",
        );
        for (idx, bit) in bits.iter().enumerate() {
            let variant = if *bit == 0 { 'A' } else { 'B' };
            manifest.push_str(&format!(
                "#EXTINF:2.000,\n{base_url}/seg_{idx:04}_{variant}.ts\n"
            ));
        }
        manifest.push_str("#EXT-X-ENDLIST\n");
        manifest
    }
}
