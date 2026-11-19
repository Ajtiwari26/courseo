# Courseo SecureWatch — Technical Specification (SPEC.md)

## 1. System Overview
Courseo SecureWatch is a defense-in-depth DRM, forensic watermarking, and optical anti-camcording suite designed to protect premium educational video streams from screen capture, network interception, and physical camera recording.

## 2. Core Requirements & Subsystems

### 2.1 Optical Anti-Camcording (Rolling Shutter Interference)
- **Objective:** Induce severe horizontal contrast banding on CMOS rolling-shutter cameras while remaining imperceptible to the human visual system (HVS).
- **Temporal Modulation:** Alternating frame polarity $p(t) = (-1)^{\lfloor t \cdot f_{\text{refresh}} \rfloor}$ applied to base luminance $L_0$ with modulation amplitude $\alpha = 0.35$.
- **Performance Targets:**
  - Simulated CMOS rolling shutter Michelson contrast ratio $C = \frac{L_{\max} - L_{\min}}{L_{\max} + L_{\min}} \ge 0.30$ across both 60 Hz and 120 Hz display refresh rates.
  - Human retinal integration error $|0.5 \cdot (L_{\text{even}} + L_{\text{odd}}) - L_0| < 10^{-6}$.
- **Client Shader:** WebGL2 GLSL ES 3.00 fragment shader dynamically clamping `localAlpha` to available luminance headroom (`headroom * 0.95`) to prevent clipping artifacts.

### 2.2 Forensic Watermarking & BCH(64,48) Error Correction
- **Payload:** 48-bit subscriber/session identifier encoded into a 64-bit codeword using a systematic BCH(64,48) code over $GF(2)$.
- **Generator Polynomial:** 16-bit parity check polynomial $g(x)$ providing multi-bit error correction and detection resilience against re-encoding, cropping, and camcording noise.
- **Embedding:** Spread-spectrum / perceptual spatial-temporal embedding resilient to lossy H.264/HEVC transcoding.

### 2.3 CENC AES-128-CBCS Media Encryption
- **Standard:** ISO/IEC 23001-7 Common Encryption (`cbcs` scheme).
- **Cipher:** AES-128-CBC with pattern encryption (e.g., 1:9 crypt:skip block pattern on NAL unit video slices) and constant 16-byte Initialization Vector (IV) per sample/subsample.
- **Frame Format:** Clear NAL unit headers preserved for decoder synchronization; encrypted slice payloads with PKCS#7 / ISO CENC subsample mapping.

### 2.4 Client-Side WebGL Player Protection
- **Rendering Pipeline:** Decrypted frames uploaded directly to WebGL2 textures (`sampler2D u_videoFrame`), processed by the anti-camcording fragment shader.
- **Hardening:** Canvas readback (`readPixels`, `toDataURL`, `captureStream`) restricted/monitored; DevTools tamper detection and EME/CDM license binding.

### 2.5 Hardware Content Protection & EME/CDM Integration
- **ISO/IEC 23001-7 PSSH Box Synthesis:**
  - Standard `'pssh'` box with 4-byte length, fourcc `b"pssh"`, 1-byte version, 3-byte flags, 16-byte SystemID UUID:
    * Google Widevine: `edef8ba9-79d6-4ace-a3c8-27dcd51d21ed`
    * Apple FairPlay: `94ce86fb-07ff-4f43-adb8-93d2fa968ca2`
    * Microsoft PlayReady: `9a04f079-9840-4286-ab92-e65be0885f95`
  - Version 1 supports 4-byte big-endian KID count followed by $N \times 16$-byte KID array.
- **EME Hardware License Policy:**
  - Enforces `HDCP_V2_2` compliance on client display links for 1080p/4K content; displays reporting `HdcpLevel::None` or `HdcpLevel::V1_4` are rejected with `DrmError::Unauthorized`.
- **OS Hardware Display Affinity & Capture Blinding:**
  - macOS `NSWindowSharingNone` (sharingType = 0): WindowServer excludes window buffer from all screen captures.
  - Windows DWM `WDA_EXCLUDEFROMCAPTURE` (0x11): Renders black rectangle to screen recording software.
  - Android `FLAG_SECURE` (`0x00002000`): Suppresses screenshots and screen capture at SurfaceFlinger level.
- **Desktop Electron/Tauri Bridge:**
  - `desktop/courseo_secure_desktop.js` enforces `mainWindow.setContentProtection(true)` and suppresses capture hotkeys (`PrintScreen`, `Cmd+Shift+3/4/5`).

### 2.6 Adversarial Threat Matrix
| Threat Vector | Attack Mechanism | SecureWatch Countermeasure |
|---|---|---|
| Physical Camcording | Smartphone/DSLR CMOS rolling shutter recording | Temporal luminance modulation ($\alpha = 0.35$) causing $\ge 30\%$ contrast banding |
| Stream Ripping | Intercepting HLS/DASH segments | CENC AES-128-CBCS encryption with short-lived session keys |
| Screen Capture / Leaking | OS-level capture or HDMI splitter | Forensic BCH(64,48) watermark tracing leak to subscriber ID |
| OS Screen Recording | QuickTime, OBS, Cmd+Shift+5 | Hardware content protection (`NSWindowSharingNone`, `WDA_EXCLUDEFROMCAPTURE`, `FLAG_SECURE`) |
| Unencrypted HDMI Link | Insecure external display / capture card | EME License Manager enforcing HDCP 2.2 hardware policy |
| Browser Tampering | Overriding WebGL shaders or canvas readback | Active prototype traps, headroom-clamped shader injection, EME binding |
