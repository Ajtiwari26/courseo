# Problem Specification: Courseo SecureWatch
## Zero-Leak Anti-Camcording DRM & Forensic Leak Attribution Engine

**Classification:** Hardest End-to-End SDLC Benchmark (Problem 6)  
**Target Workspace:** `/Users/ajaytiwari/Desktop/Projects/courseo`  
**Target Artifacts:** `SPEC.md`, `DESIGN.md`, `PLAN.md`, Production Rust Crate (`courseo-core`), Web Player Compositor (`courseo-player`), and Adversarial Test Suite (`testscript/securewatch_adversarial_test.rs`).

---

## 1. Problem Overview & Threat Model

Online educational platforms lose hundreds of millions of dollars annually to course piracy. Malicious actors bypass software-level DRM via:
1. **Analog Camcording**: Pointing a smartphone or DSLR camera at a computer screen to record high-value video lectures without triggering OS-level screen capture detection.
2. **Stream Interception & Key Extraction**: Snooping HLS `.m3u8` manifests and network calls to download raw encrypted segments and decryption keys.
3. **Browser Prototype Tampering**: Injecting browser extensions or DevTools scripts to intercept canvas buffers via `canvas.toDataURL()` or `getDisplayMedia()`.
4. **Anonymous Distribution**: Stripping metadata and re-encoding video to 720p/480p, leaving no trace of the pirating user's identity.

The **Courseo SecureWatch** challenge requires implementing an end-to-end, multi-layered video security and forensic attribution platform capable of neutralizing software and hardware recording attacks while deterministically tracking any leaked footage back to the exact user account and IP address.

---

## 2. Core Functional Requirements

### Subsystem 1: Cryptographic Packaging & DRM Engine (`crypto`)
- Implement AES-128-CBCS Common Encryption (CENC) for media chunks with 16-byte initialization vectors and subsample pattern encryption (10% clear / 90% crypt).
- Implement ephemeral key rotation: decryption keys expire every 30 seconds.
- Implement asymmetric Ed25519 session authorization: client requests must include a cryptographically signed playback token encoding `user_id`, `client_ip`, `session_id`, `expiry_timestamp`, and `allowed_renditions`.
- Replay and forge prevention: expired or signature-mismatched license requests return `Err(DrmError::Unauthorized)`.

### Subsystem 2: Forensic A/B Variant Watermarking & Leak Tracer (`watermark`)
- Implement **BCH(63, 45)** or **BCH(64, 48)** error-correcting code for a 64-bit payload:
  $$\text{Payload} = \text{UserID}_{32} \parallel \text{SubnetIP}_{16} \parallel \text{Timestamp}_{16}$$
- Support dual-variant segment mapping: each 2-second video segment exists as Variant A (bit `0`) or Variant B (bit `1`).
- Implement dynamic playlist synthesis: stitching Variant A and B segments based on the user's 64-bit codeword.
- Implement **Blind Forensic Extractor**: Given an array of detected segment variants (even with up to 15% bit-flip corruption caused by camcorder noise, downsampling, or compression), recover the original 64-bit payload and decode the exact User ID, IP subnet, and timestamp with $\ge 99.99\%$ statistical confidence.

### Subsystem 3: Optical Anti-Camcording Modulation Engine (`anti_cam`)
- Formulate the mathematical model for CMOS electronic rolling shutter beat-frequency interference:
  - Display refresh rate: $f_d \in \{60, 120\}\text{Hz}$.
  - Camera shutter speed: $T_{\text{exp}} \in [1/60, 1/1000]\text{s}$.
  - CMOS sensor row readout period: $\tau \approx 15\text{ms} / H_{\text{sensor}}$.
- Generate GLSL shader instructions for the client-side WebGL compositor:
  - Interleave high-frequency spatial raster lines with micro-luminance offsets:
    $$L(x, y, t) = I(x, y, t) + (-1)^{t \cdot f_d + y \bmod 2} \cdot \alpha(x, y)$$
  - Ensure retinal persistence integration error is below human perceptual threshold ($\Delta L < 2.5\%$).
  - Simulate the optical camera output: assert that a rolling shutter sensor captures prominent horizontal dark/light banding bars (minimum 30% contrast ratio) across the image, destroying external camera capture readability.

### Subsystem 4: Client-Side Anti-Tamper & Secure Compositor (`compositor`)
- WebGL2 canvas framebuffer protection:
  - Deep prototype freezing: override and freeze `HTMLCanvasElement.prototype.toDataURL`, `CanvasRenderingContext2D.prototype.getImageData`, `HTMLMediaElement.prototype.captureStream`, and `navigator.mediaDevices.getDisplayMedia`.
  - Attempting to capture the canvas via JavaScript APIs returns a blank/black frame or triggers an immediate security trap.
- High-precision DevTools detection trap:
  - Measure execution timing differentials via `performance.now()`.
  - Detect debugger breakpoints and active console evaluation traps.
- Dynamic pseudo-random floating watermark:
  - Semi-transparent, drifting overlay displaying masked user credentials.
  - Position changes via an encrypted pseudo-random walk every 3 seconds to defeat static watermark delogos/removal filters.

### Subsystem 5: Real-Time Telemetry & Revocation Gateway (`telemetry`)
- Maintain active user playback sessions with periodic encrypted heartbeats (every 5 seconds).
- Detect anomalous events:
  - Window blur / tab backgrounding.
  - Screen sharing / display capture attempts.
  - Concurrent stream playback from geographically impossible IP addresses.
- Implement instant revocation: revoking a session invalidates its rotating key ladder in $<200\text{ms}$, halting subsequent segment playback.

### Subsystem 6: Hardware Content Protection & EME/CDM Integration (`hardware_protection`)
- ISO/IEC 23001-7 Protection System Specific Header (PSSH) Box generator:
  - Binary box synthesis for Widevine (`edef8ba9-79d6-4ace-a3c8-27dcd51d21ed`), Apple FairPlay (`94ce86fb-07ff-4f43-adb8-93d2fa968ca2`), and PlayReady (`9a04f079-9840-4286-ab92-e65be0885f95`).
  - Correct v0/v1 fourcc `'pssh'` encoding with SystemID, KID array, and binary payload.
- EME Hardware License Handshake Protocol:
  - Evaluates client challenge, validates digital signature, and enforces HDCP 2.2 output protection policies.
- OS Hardware Display Affinity & Window Protection abstractions:
  - Models macOS `NSWindowSharingNone`, Windows `WDA_MONITOR`, and Android `FLAG_SECURE`.
- Desktop Secure App bridge (`desktop/courseo_secure_desktop.js`):
  - Electron/Tauri content protection integration enforcing `mainWindow.setContentProtection(true)`.

### Subsystem 7: Non-Linear 2D Anti-Averaging Anti-Camcording Engine (`anti_cam`)
- 2D pseudo-random spatial-temporal checkerboard modulation defeating FFmpeg 2-frame temporal averaging (`tblend=all_mode=average`).
- Asserts that blended output maintains $\ge 15\%$ residual spatial contrast variance.

### Subsystem 8: Hypervisor & Virtual Machine Detector (`hypervisor_detector`)
- Detects virtualized graphics adapters (`llvmpipe`, `vmware`, `virtualbox`, `qemu`, `parallels`, `angle (software)`, `microsoft basic render driver`).
- Timer jitter evaluation detecting VM context-switch spikes ($>50\mu\text{s}$).

### Subsystem 9: Psychoacoustic Audio Spread-Spectrum Watermarking (`audio_watermark`)
- Injects 48-bit subscriber ID into 15kHz–19kHz psychoacoustic masking bands using DSSS with 127-bit PN code.
- Recovers subscriber ID from audio-only rips even after lossy audio transcoding.

### Subsystem 10: Short-Window Chrominance / DCT Bias Watermark (`dct_watermark`)
- Embeds forensic signature into 8x8 DCT blocks in $Cb/Cr$ channels across keyframes for 5–15 second clip attribution.

---

## 3. Required Deliverables

1. `SPEC.md`: Exhaustive technical specification with mathematical derivations for rolling shutter interference, anti-averaging math, audio DSSS, and DCT watermarks.
2. `DESIGN.md`: Concrete system design detailing module boundaries, data structures, state machines, and threat mitigation matrices.
3. `PLAN.md`: Implementation milestones, file-level ownership, and test matrix.
4. `courseo-core` (Rust Crate):
   - `Cargo.toml`: Standalone crate with zero heavyweight bloat.
   - `src/lib.rs`: Strict safety attributes (`#![forbid(unsafe_code)]`, `#![deny(clippy::all)]`).
   - `src/crypto.rs`: AES-128-CBCS CENC, Ed25519 tokens, and key rotation.
   - `src/watermark.rs`: BCH error correction, A/B manifest synthesis, and forensic leak extractor.
   - `src/anti_cam.rs`: Non-linear 2D anti-averaging simulator and GLSL shader generator.
   - `src/telemetry.rs`: Heartbeat monitor, anomaly detector, and revocation ledger.
   - `src/hardware_protection.rs`: ISO/IEC 23001-7 PSSH box generator, EME license evaluator, and OS display affinity.
   - `src/hypervisor_detector.rs`: VM/hypervisor renderer and timer jitter detection.
   - `src/audio_watermark.rs`: Psychoacoustic audio DSSS spread-spectrum watermarking engine.
   - `src/dct_watermark.rs`: Short-window 8x8 DCT chrominance bias per-frame watermark.
   - `src/error.rs`: Typed error domain (`HypervisorDetected`, `AudioWatermarkError`, etc.).
5. `courseo-player` (Web / TypeScript / GLSL & Desktop):
   - Secure WebGL video compositor with 2D non-linear anti-averaging shader and VM detection.
   - Prototype freezing and anti-tamper security shield.
   - DevTools timing trap and dynamic drifting watermark.
   - `desktop/courseo_secure_desktop.js`: Electron/Tauri desktop content protection wrapper.
6. `testscript/securewatch_adversarial_test.rs`:
   - Comprehensive integration test suite executing 100+ adversarial scenarios (bit-flip corruption, camcorder blur, key rotation expiry, replay attacks, concurrent stream revocation, PSSH binary encoding, HDCP 2.2 license evaluation, 2-frame tblend anti-averaging defense, hypervisor detection, audio DSSS extraction, and DCT short-window attribution).
   - Zero compiler warnings, zero Clippy warnings (`-D warnings`), 100% test pass rate.
