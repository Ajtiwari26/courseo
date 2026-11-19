# Courseo SecureWatch — Implementation & Verification Plan (PLAN.md)

## 1. Objectives & Milestones
This plan details the engineering milestones, verification criteria, and rollout steps for the Courseo SecureWatch DRM, forensic watermarking, and optical anti-camcording suite.

| Phase | Deliverable | Status |
|---|---|---|
| Phase 1 | Optical Anti-Camcording Engine (`src/anti_cam.rs`) with $\alpha = 0.35$ | Completed |
| Phase 2 | Forensic Watermarking (`BCH(64,48)`) & CENC AES-128-CBCS Encryption | Completed |
| Phase 3 | Technical Documentation (`SPEC.md`, `DESIGN.md`, `PLAN.md`) | Completed |
| Phase 4 | Full Test Suite (`cargo test --all-targets`) & Lint Verification (`clippy -D warnings`) | Completed |
| Phase 5 | Hardware Content Protection & EME/CDM Engine (`src/hardware_protection.rs` & `desktop/`) | Completed |

## 2. Core Engineering Tasks

### 2.1 Rolling Shutter Optical Interference Tuning
- Update `RollingShutterParams::default()` in `src/anti_cam.rs` so `modulation_alpha` is `0.35`.
- Verify via `AntiCamEngine::simulate_rolling_shutter` that:
  - Michelson contrast ratio $C \ge 0.30$ across both 60 Hz and 120 Hz display refresh rates.
  - Human retinal integration error remains strictly $< 10^{-6}$.
- Validate `generate_webgl2_fragment_shader` outputs valid GLSL ES 3.00 with per-pixel luminance headroom clamping (`headroom * 0.95`).

### 2.2 Cryptographic & Watermarking Integrity
- **BCH(64,48):** Verify systematic encoding and syndrome decoding recover 48-bit subscriber IDs under multi-bit channel noise.
- **CENC AES-128-CBCS:** Verify 1:9 pattern encryption preserves clear NAL headers and decrypts cleanly with constant 16-byte IV per subsample.

### 2.3 Hardware Content Protection & EME/CDM Engine
- **ISO/IEC 23001-7 PSSH Box:** Synthesize valid binary `'pssh'` atoms for Widevine, FairPlay, and PlayReady.
- **EME License Manager:** Enforce `HDCP_V2_2` compliance on external displays; reject streams on insecure links.
- **OS Display Affinity:** Bind `NSWindowSharingNone`, `WDA_EXCLUDEFROMCAPTURE`, and `FLAG_SECURE`.
- **Desktop Launcher:** Implement Electron/Tauri bridge in `desktop/courseo_secure_desktop.js` enforcing `mainWindow.setContentProtection(true)`.

### 2.3 Client-Side WebGL2 Player Hardening
- Integrate the generated GLSL ES 3.00 fragment shader into the browser WebGL2 rendering loop.
- Enforce EME/CDM hardware-backed decryption, `preserveDrawingBuffer: false`, and canvas readback protections.

## 3. Verification & Quality Gates
1. **Unit & Integration Tests:** Execute `cargo test --all-targets` and confirm 100% pass rate across all optical, cryptographic, and watermarking test cases.
2. **Static Analysis:** Execute `cargo clippy --all-targets -- -D warnings` and confirm zero warnings or lint violations.
3. **Adversarial Review:** Cross-check implementation against the Adversarial Threat Matrix in `SPEC.md` and `DESIGN.md`.
