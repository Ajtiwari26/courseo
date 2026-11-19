use courseo_core::anti_cam::{AntiCamEngine, RollingShutterParams};
use courseo_core::crypto::{decrypt_cbcs, encrypt_cbcs, KeyLadder, KEY_ROTATION_INTERVAL_SECS};
use courseo_core::telemetry::{AnomalyEvent, SessionState, TelemetryGateway};
use courseo_core::watermark::{WatermarkCodec, WatermarkPayload};
use courseo_core::DrmError;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[test]
fn test_rolling_shutter_contrast_banding() {
    for refresh_hz in [60.0_f64, 120.0_f64] {
        let params = RollingShutterParams {
            display_refresh_hz: refresh_hz,
            ..RollingShutterParams::default()
        };
        let report = AntiCamEngine::simulate_rolling_shutter(&params)
            .expect("rolling shutter simulation should succeed");

        assert!(
            report.camera_contrast_ratio >= 0.30,
            "expected >= 30% contrast banding at {refresh_hz}Hz, got {:.4}",
            report.camera_contrast_ratio
        );
        assert!(
            report.human_retinal_error < 1e-6,
            "expected imperceptible human retinal error at {refresh_hz}Hz, got {:.6}",
            report.human_retinal_error
        );
        assert_eq!(report.row_profile.len(), params.sensor_height);
    }
}

#[test]
fn test_bch_forensic_watermark_recovery_clean() {
    let payload = WatermarkPayload {
        user_id: 0xDEAD_BEEF,
        subnet_ip: 0xC0A8,
        timestamp: 0x1A2B,
    };

    let encoded = WatermarkCodec::encode(payload);
    let (recovered, confidence) =
        WatermarkCodec::extract(&encoded).expect("clean watermark extraction should succeed");

    assert_eq!(recovered, payload);
    assert!((confidence - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_bch_forensic_watermark_recovery_with_15_percent_bitflips() {
    let payload = WatermarkPayload {
        user_id: 42_901_337,
        subnet_ip: 0x0A14,
        timestamp: 0x7F01,
    };
    let clean_bits = WatermarkCodec::encode(payload);

    for (seed, flip_rate) in [(0x1010_u64, 0.10_f64), (0x1515_u64, 0.15_f64)] {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut corrupted = clean_bits.clone();
        let total_flips = ((corrupted.len() as f64) * flip_rate).round() as usize;

        let mut indices: Vec<usize> = (0..corrupted.len()).collect();
        for i in 0..total_flips {
            let swap_idx = rng.gen_range(i..indices.len());
            indices.swap(i, swap_idx);
            let pos = indices[i];
            corrupted[pos] ^= 1;
        }

        let (recovered, _confidence) = WatermarkCodec::extract(&corrupted)
            .expect("BCH extraction should recover payload under bit-flip noise");

        assert_eq!(
            recovered.user_id,
            payload.user_id,
            "user_id mismatch at {:.0}% bit-flips",
            flip_rate * 100.0
        );
        assert_eq!(
            recovered.subnet_ip,
            payload.subnet_ip,
            "subnet_ip mismatch at {:.0}% bit-flips",
            flip_rate * 100.0
        );
        assert_eq!(
            recovered.timestamp,
            payload.timestamp,
            "timestamp mismatch at {:.0}% bit-flips",
            flip_rate * 100.0
        );
    }
}

#[test]
fn test_dynamic_manifest_stitching_variant_sequence() {
    let payload = WatermarkPayload {
        user_id: 987_654_321,
        subnet_ip: 0xAC10,
        timestamp: 0x0409,
    };
    let base_url = "https://cdn.courseo.io/hls/course_101";
    let expected_bits = WatermarkCodec::encode(payload);
    let manifest = WatermarkCodec::synthesize_m3u8(base_url, payload);

    let segment_urls: Vec<&str> = manifest
        .lines()
        .filter(|line| line.starts_with(base_url))
        .collect();

    assert_eq!(segment_urls.len(), expected_bits.len());

    for (idx, (&url, &bit)) in segment_urls.iter().zip(expected_bits.iter()).enumerate() {
        let expected_variant = if bit == 0 { 'A' } else { 'B' };
        let expected_url = format!("{base_url}/seg_{idx:04}_{expected_variant}.ts");
        assert_eq!(url, expected_url);
    }
}

#[test]
fn test_ephemeral_key_rotation_and_expiry() {
    let mut ladder = KeyLadder::new();
    let issued_at = 1_700_000_000_u64;
    let key = ladder.rotate_key(issued_at);
    let iv = [0x42_u8; 16];
    let plaintext: Vec<u8> = (0..320_u32).map(|i| (i % 251) as u8).collect();

    let ciphertext = encrypt_cbcs(&key.key_bytes, &iv, &plaintext);
    assert_ne!(ciphertext, plaintext);

    let valid_key = ladder
        .get_valid_key(key.key_id, issued_at + 10)
        .expect("fresh key should be valid");
    let decrypted = decrypt_cbcs(&valid_key.key_bytes, &iv, &ciphertext);
    assert_eq!(decrypted, plaintext);

    let expired_result =
        ladder.get_valid_key(key.key_id, issued_at + KEY_ROTATION_INTERVAL_SECS + 1);
    assert!(
        matches!(expired_result, Err(DrmError::KeyExpired { .. })),
        "expected KeyExpired error after rotation interval"
    );
}

#[test]
fn test_telemetry_heartbeat_and_instant_revocation() {
    let mut gateway = TelemetryGateway::new();
    let session_id = "sess_live_001";
    let client_ip = "203.0.113.42";

    gateway
        .register_session(session_id.to_string(), 777, client_ip.to_string(), 100)
        .expect("session registration should succeed");

    let state = gateway
        .record_heartbeat(session_id, client_ip, 105)
        .expect("valid heartbeat should succeed");
    assert_eq!(state, SessionState::Active);

    let revoked_state = gateway
        .report_anomaly(session_id, AnomalyEvent::ScreenShareAttempt)
        .expect("anomaly reporting should succeed");
    assert_eq!(revoked_state, SessionState::Revoked);
    assert!(gateway.is_revoked(session_id));

    let post_revoke = gateway.record_heartbeat(session_id, client_ip, 110);
    assert!(
        matches!(post_revoke, Err(DrmError::SessionRevoked(_))),
        "subsequent heartbeat after revocation must fail"
    );
}
