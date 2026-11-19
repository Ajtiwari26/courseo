use courseo_core::anti_cam::{AntiCamEngine, RollingShutterParams};
use courseo_core::audio_watermark::{AudioWatermarkCodec, PAYLOAD_BITS, PN_SEQUENCE_LEN};
use courseo_core::crypto::{decrypt_cbcs, encrypt_cbcs, KeyLadder, KEY_ROTATION_INTERVAL_SECS};
use courseo_core::dct_watermark::DctWatermarkCodec;
use courseo_core::hardware_protection::{
    DisplayAffinityPolicy, DrmSystem, EmeLicenseChallenge, EmeLicenseManager, HdcpLevel,
    OsDisplayProtection, PsshBox,
};
use courseo_core::hypervisor_detector::HypervisorDetector;
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

#[test]
fn test_pssh_binary_box_synthesis() {
    let kid = [0xAB_u8; 16];
    let payload = vec![0x08, 0x01, 0x12, 0x10];

    let expected_systems = [
        (
            DrmSystem::Widevine,
            [
                0xed, 0xef, 0x8b, 0xa9, 0x79, 0xd6, 0x4a, 0xce, 0xa3, 0xc8, 0x27, 0xdc, 0xd5, 0x1d,
                0x21, 0xed,
            ],
        ),
        (
            DrmSystem::FairPlay,
            [
                0x94, 0xce, 0x86, 0xfb, 0x07, 0xff, 0x4f, 0x43, 0xad, 0xb8, 0x93, 0xd2, 0xfa, 0x96,
                0x8c, 0xa2,
            ],
        ),
        (
            DrmSystem::PlayReady,
            [
                0x9a, 0x04, 0xf0, 0x79, 0x98, 0x40, 0x42, 0x86, 0xab, 0x92, 0xe6, 0x5b, 0xe0, 0x88,
                0x5f, 0x95,
            ],
        ),
    ];

    for (system, expected_uuid) in expected_systems {
        // Version 1 box with KID array
        let pssh_v1 = PsshBox::new(system, vec![kid], payload.clone());
        let bytes = pssh_v1.to_bytes();

        let box_len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        assert_eq!(box_len, bytes.len());
        assert_eq!(&bytes[4..8], b"pssh");
        assert_eq!(bytes[8], 1); // version 1
        assert_eq!(&bytes[9..12], &[0, 0, 0]); // flags
        assert_eq!(&bytes[12..28], &expected_uuid);
        let kid_count = u32::from_be_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);
        assert_eq!(kid_count, 1);
        assert_eq!(&bytes[32..48], &kid);
        let data_len = u32::from_be_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]) as usize;
        assert_eq!(data_len, payload.len());
        assert_eq!(&bytes[52..52 + data_len], payload.as_slice());

        // Version 0 box without KIDs
        let pssh_v0 = PsshBox::new(system, vec![], payload.clone());
        let bytes_v0 = pssh_v0.to_bytes();
        assert_eq!(bytes_v0[8], 0);
        assert_eq!(&bytes_v0[12..28], &expected_uuid);
        let v0_data_len =
            u32::from_be_bytes([bytes_v0[28], bytes_v0[29], bytes_v0[30], bytes_v0[31]]) as usize;
        assert_eq!(v0_data_len, payload.len());
        assert_eq!(&bytes_v0[32..32 + v0_data_len], payload.as_slice());
    }
}

#[test]
fn test_eme_license_hdcp_enforcement() {
    let manager = EmeLicenseManager::new(HdcpLevel::V2_2, 300);
    let now = 1_700_000_000_u64;

    let compliant_challenge = EmeLicenseChallenge {
        subscriber_id: 1001,
        session_id: "sess_hdcp_ok".to_string(),
        token_signature_valid: true,
        token_expiry_unix: now + 600,
        reported_hdcp: HdcpLevel::V2_2,
        drm_system: DrmSystem::Widevine,
    };

    let grant = manager
        .evaluate_challenge(&compliant_challenge, now)
        .expect("HDCP 2.2 compliant display should be granted license");
    assert_eq!(grant.enforced_hdcp, HdcpLevel::V2_2);
    assert_eq!(grant.drm_system, DrmSystem::Widevine);
    assert_eq!(grant.license_ttl_secs, 300);

    let non_compliant_challenge = EmeLicenseChallenge {
        reported_hdcp: HdcpLevel::None,
        ..compliant_challenge.clone()
    };
    let err = manager.evaluate_challenge(&non_compliant_challenge, now);
    assert!(
        matches!(err, Err(DrmError::Unauthorized(_))),
        "HDCP 2.2 stream must reject display reporting HdcpLevel::None"
    );

    let downgraded_challenge = EmeLicenseChallenge {
        reported_hdcp: HdcpLevel::V1_4,
        ..compliant_challenge
    };
    let err_v14 = manager.evaluate_challenge(&downgraded_challenge, now);
    assert!(
        matches!(err_v14, Err(DrmError::Unauthorized(_))),
        "HDCP 2.2 stream must reject display reporting HdcpLevel::V1_4"
    );
}

#[test]
fn test_os_display_affinity_bitmasks() {
    assert_eq!(OsDisplayProtection::MacSharingNone.bitmask(), 0);
    assert_eq!(
        OsDisplayProtection::WindowsExcludeFromCapture.bitmask(),
        0x11
    );
    assert_eq!(
        OsDisplayProtection::AndroidFlagSecure.bitmask(),
        0x0000_2000
    );

    let mac_policy = DisplayAffinityPolicy::new(OsDisplayProtection::MacSharingNone);
    assert!(mac_policy.validate(0).is_ok());
    assert!(matches!(
        mac_policy.validate(1),
        Err(DrmError::Unauthorized(_))
    ));

    let win_policy = DisplayAffinityPolicy::new(OsDisplayProtection::WindowsExcludeFromCapture);
    assert!(win_policy.validate(0x11).is_ok());
    assert!(matches!(
        win_policy.validate(0x01),
        Err(DrmError::Unauthorized(_))
    ));
    assert!(matches!(
        win_policy.validate(0x00),
        Err(DrmError::Unauthorized(_))
    ));

    let android_policy = DisplayAffinityPolicy::new(OsDisplayProtection::AndroidFlagSecure);
    assert!(android_policy.validate(0x0000_2000).is_ok());
    assert!(android_policy.validate(0x0000_2004).is_ok());
    assert!(matches!(
        android_policy.validate(0x0000_0000),
        Err(DrmError::Unauthorized(_))
    ));
}

#[test]
fn test_anti_averaging_tblend_resilience() {
    let params = RollingShutterParams::default();
    let residual_contrast = AntiCamEngine::simulate_two_frame_average_attack(&params)
        .expect("2-frame averaging simulation should succeed");
    assert!(
        residual_contrast >= 0.15,
        "expected residual spatial contrast variance >= 15% after 2-frame tblend, got {:.4}",
        residual_contrast
    );
}

#[test]
fn test_hypervisor_detection() {
    let blacklisted = [
        "llvmpipe (LLVM 15.0.7, 256 bits)",
        "VMware SVGA 3D",
        "VirtualBox Graphics Adapter",
        "QEMU Standard VGA",
        "Parallels Display Adapter",
        "ANGLE (Software Renderer)",
        "Microsoft Basic Render Driver",
    ];
    let clean_timing = [10_u64, 12, 11, 13, 12, 11];

    for renderer in blacklisted {
        assert!(HypervisorDetector::is_blacklisted_renderer(renderer));
        let res = HypervisorDetector::evaluate_environment(renderer, &clean_timing);
        assert!(
            matches!(res, Err(DrmError::HypervisorDetected(_))),
            "expected HypervisorDetected for renderer {renderer}"
        );
    }

    assert!(!HypervisorDetector::is_blacklisted_renderer("Apple M5 GPU"));
    assert!(HypervisorDetector::evaluate_environment("Apple M5 GPU", &clean_timing).is_ok());

    let spiky_timing = [10_u64, 12, 95, 11, 14];
    assert!(HypervisorDetector::analyze_timing_jitter(&spiky_timing));
    assert!(matches!(
        HypervisorDetector::evaluate_environment("Apple M5 GPU", &spiky_timing),
        Err(DrmError::HypervisorDetected(_))
    ));
}

#[test]
fn test_audio_spread_spectrum_watermark() {
    let sample_rate = 48_000_u32;
    let subscriber_id: u64 = 0x0000_ABCD_1234_5678;
    let num_samples = PAYLOAD_BITS * PN_SEQUENCE_LEN * 2;
    let mut pcm: Vec<f32> = (0..num_samples)
        .map(|i| {
            0.1 * ((2.0 * std::f64::consts::PI * 440.0 * (i as f64) / (sample_rate as f64)).sin()
                as f32)
        })
        .collect();

    AudioWatermarkCodec::encode_dsss(&mut pcm, subscriber_id, sample_rate)
        .expect("DSSS audio watermark encoding should succeed");
    let extracted = AudioWatermarkCodec::extract_dsss(&pcm, sample_rate)
        .expect("DSSS audio watermark extraction should succeed");
    assert_eq!(extracted, subscriber_id);
}

#[test]
fn test_short_window_dct_watermark() {
    let subscriber_id: u32 = 0xCAFE_BABE;
    let mut blocks = vec![[0.0_f64; 64]; 128];

    DctWatermarkCodec::embed_8x8_chroma(&mut blocks, subscriber_id)
        .expect("DCT 8x8 chroma embedding should succeed");
    let (extracted, confidence) = DctWatermarkCodec::extract_8x8_chroma(&blocks)
        .expect("DCT 8x8 chroma extraction should succeed");

    assert_eq!(extracted, subscriber_id);
    assert!(
        confidence >= 0.99,
        "expected high confidence, got {confidence}"
    );
}
