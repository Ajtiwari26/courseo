# SENIOR ARCHITECTURAL DIRECTIVE & SYSTEM DESIGN
## Project: Courseo SecureWatch (Zero-Leak Video DRM & Forensic Privacy Engine)
**Document ID:** ARCH-COURSEO-2026-001  
**Classification:** Senior Architectural Blueprint (Opus-Exclusive Sovereign Record)  
**Authority:** Deploymate Engineering, Etta Architecture & Senior System Design  
**Target:** `/Users/ajaytiwari/Desktop/Projects/courseo/docs/architecture/SENIOR_DIRECTIVE_AND_SYSTEM_DESIGN.md`

---

## 1. Executive Vision & Core Problem Formulation

Traditional Digital Rights Management (DRM) technologies—such as Google Widevine L3, Apple FairPlay, and Microsoft PlayReady—operate on the assumption that content leaks occur strictly within software boundaries. However, in the high-stakes EdTech and premium education industry (where courses command \$500–\$5,000 per seat), pirated video leaks overwhelmingly bypass software DRM through two catastrophic avenues:
1. **The Analog Hole / External Camcording**: A student aims a modern smartphone or DSLR at their high-resolution monitor and records the entire lecture. Widevine and OS-level capture APIs are completely blind to physical cameras.
2. **Anonymous Re-Hosting**: Pirated videos are stripped of metadata, re-encoded at 720p, and anonymously distributed across Telegram channels, torrent trackers, and Mega links without any trace of whose account was breached.

**Courseo SecureWatch** solves this end-to-end by architecting a unified, zero-leak video privacy platform combining:
* **Optical Anti-Camcording Modulation (TAC)**: Inducing optical rolling-shutter moiré patterns and dark banding bars in external camera sensors without degrading human visual persistence.
* **Dual-Variant A/B Forensic Watermarking**: Embedding a zero-knowledge 64-bit session fingerprint $(UserID \parallel IP \parallel Timestamp)$ directly into the sequence of delivered video chunks, allowing blind forensic attribution from as little as 15 seconds of re-encoded pirated video.
* **Cryptographic Common Encryption (CENC) with Ephemeral Key Rotation**: AES-128-CBCS stream encryption with 30-second rotating keys signed via asymmetric Ed25519 tokens.
* **Client-Side WebGL Compositor & Tamper Barrier**: Hardware-accelerated canvas rendering with deep prototype freezing (`toDataURL`, `getImageData`, `getDisplayMedia`), DevTools timing traps, and floating visible steganographic overlays.
* **Real-Time Telemetry & Revocation Gateway**: Sub-200ms license revocation upon screen-sharing detection, multi-monitor cloning, or abnormal playback anomalies.

---

## 2. Mathematical & Physical Foundations

### 2.1 Optical Rolling Shutter & Anti-Camcording Moiré Induction

Consumer smartphone cameras (CMOS sensors) utilize an **electronic rolling shutter**, exposing the sensor row-by-row sequentially from top to bottom over a finite readout time ($T_{\text{readout}} \approx 10\text{ms}$–$25\text{ms}$) with per-row exposure durations ($T_{\text{exp}} \approx 1/60\text{s}$–$1/1000\text{s}$).

In contrast, the human eye operates via continuous photochemical integration with a **critical flicker fusion threshold** ($\approx 50\text{Hz}$–$60\text{Hz}$).

```
[ Human Eye Perception ]
Frame t (Luminance +α)   ──┐
                           ├──> Integrated by retina (Δt > 20ms) ──> Crystal-Clear Video
Frame t+1 (Luminance -α) ──┘

[ Smartphone CMOS Rolling Shutter ]
Row 0..30 (t = 0.0ms)    ──> Samples (+α)  ──> Bright horizontal bar
Row 31..60 (t = 4.0ms)   ──> Samples (-α)  ──> Dark horizontal band
Row 61..90 (t = 8.0ms)   ──> Samples (+α)  ──> Bright horizontal bar
                           └──> Recorded output: Violent rolling black bands & moiré blur!
```

#### The TAC Modulation Invariant:
For display refresh rate $f_d \ge 60\text{Hz}$ and camera frame rate $f_c \approx 30\text{Hz}$ or $60\text{Hz}$:
1. We interleave high-frequency spatial raster lines with micro-luminance offsets:
   $$L(x, y, t) = I(x, y, t) + (-1)^{t \cdot f_d + y \bmod 2} \cdot \alpha(x, y)$$
   where $\alpha(x, y)$ is an adaptive just-noticeable-difference (JND) threshold.
2. For the human eye:
   $$\lim_{\Delta t \to \text{retinal persistence}} \int_0^{\Delta t} L(x, y, t) \, dt = \bar{I}(x, y)$$
   Result: **Zero perceived flicker to the student**.
3. For the CMOS sensor with rolling shutter row readout time $t_{\text{row}} = y \cdot \tau$:
   $$S(y) = \int_{y \cdot \tau}^{y \cdot \tau + T_{\text{exp}}} L(x, y, t) \, dt$$
   The high-frequency term beats against the line readout frequency, inducing severe periodic dark/light striations (spatial-temporal moiré bands) that ruin OCR, face detection, and video recording clarity.

---

### 2.2 Dual-Variant A/B Forensic Watermarking & Leak Attribution

To track leaks even if an attacker uses an analog camcorder or re-encodes the stream at low bitrates:
1. **Offline Dual-Encoding**: Every 2-second video segment $S_k$ is pre-encoded into two bitstreams:
   * **Variant A** ($S_{k, 0}$): Encodes bit `0` via high-frequency DCT chrominance coefficient bias.
   * **Variant B** ($S_{k, 1}$): Encodes bit `1` via complementary DCT chrominance coefficient bias.
2. **Session Fingerprint Embedding**:
   Each user session is assigned a 64-bit binary codeword:
   $$\mathbf{w} = \text{BCH\_Encode}(\text{UserID}_{32} \parallel \text{IP}_{16} \parallel \text{Timestamp}_{16})$$
   protected by Bose-Chaudhuri-Hocquenghem (BCH) error-correcting codes to withstand up to 15% bit-flip errors during camcording or compression.
3. **Dynamic Manifest Synthesis**:
   When the client requests the master HLS/DASH manifest, the edge gateway constructs a customized playlist stitching $S_{k, w_k}$.
4. **Blind Forensic Extraction**:
   Given an arbitrary 15–30 second pirated video clip:
   * The extractor computes normalized cross-correlation across the DCT frames for each 2-second window.
   * Recovers the bit sequence $\hat{\mathbf{w}}$.
   * Applies BCH error correction to resolve the exact 32-bit User ID and 16-bit subnet IP address with statistical significance ($p < 10^{-6}$).

---

## 3. Subsystem Architecture & Decomposition

The Courseo SecureWatch engine is structured into 5 decoupled subsystems:

```
+---------------------------------------------------------------------------------------+
|                                    COURSEO SECUREWATCH                                 |
+---------------------------------------------------------------------------------------+
| 1. Cryptographic Packaging & DRM Gateway (Rust Core / Actix / Tokio)                  |
|    - CENC AES-128-CBCS segment encryptor                                              |
|    - Dynamic 30s rotating key generator with Ed25519 token signatures                 |
|    - Edge manifest stitcher for A/B segment fingerprinting                            |
+---------------------------------------------------------------------------------------+
                                           |
                                           v
+---------------------------------------------------------------------------------------+
| 2. Forensic Watermark & Leak Attribution Engine (Rust Core)                           |
|    - Dual-variant DCT coefficient modulator                                           |
|    - BCH(64, 48) error-correcting codec                                               |
|    - Blind forensic extractor & Bayesian leak probability estimator                   |
+---------------------------------------------------------------------------------------+
                                           |
                                           v
+---------------------------------------------------------------------------------------+
| 3. Optical Anti-Camcording Shutter Interrupter (WebGL GLSL Shaders)                   |
|    - Temporal phase-inversion shader (60Hz / 120Hz alternating luminance pulse)       |
|    - Spatial subpixel moiré induction grid                                            |
|    - JND adaptive masking (preserves text legibility for human viewer)                |
+---------------------------------------------------------------------------------------+
                                           |
                                           v
+---------------------------------------------------------------------------------------+
| 4. Client-Side Anti-Tamper & Compositor (TypeScript / WebGL / WebWorker)              |
|    - Object.freeze on Canvas, MediaDevices, and DisplayMedia prototypes               |
|    - High-frequency DevTools timing trap (performance.now delta analysis)            |
|    - Non-repeating pseudo-random drifting visible watermark overlay                   |
+---------------------------------------------------------------------------------------+
                                           |
                                           v
+---------------------------------------------------------------------------------------+
| 5. Real-Time Telemetry & Revocation Gateway (WebSocket / Tokio)                       |
|    - Sub-200ms heartbeat & anomaly detection (window blur, screen-share events)       |
|    - Instant cryptographic license revocation & blacklisting                          |
+---------------------------------------------------------------------------------------+
```

---

## 4. Invariants & Security Guarantees

1. **INV-COURSEO-01 (Zero Cleartext on Disk)**: Unencrypted video segments must never touch persistent storage on client devices. Decrypted frames exist strictly inside GPU WebGL textures.
2. **INV-COURSEO-02 (Anti-Tamper Canvas Integrity)**: Any invocation of `canvas.toDataURL()`, `canvas.getContext('2d').getImageData()`, or `canvas.captureStream()` must return black frames or throw security violations.
3. **INV-COURSEO-03 (BCH Error-Corrected Attribution)**: Forensic leak attribution must achieve $\ge 99.99\%$ recovery accuracy on video clips degraded by 50% Gaussian blur, 720p downscaling, or 30fps camcorder re-recording.
4. **INV-COURSEO-04 (Zero-Flicker Human Tolerance)**: Under standard 60Hz/120Hz monitors, human observers must experience zero conscious visual flicker (delta luminance variance $< 2.5\%$).
5. **INV-COURSEO-05 (Deterministic Revocation)**: Detection of screen capture or multi-monitor mirroring must terminate playback and revoke active DRM keys in $\le 200\text{ms}$.

---

## 5. Senior Directive on Implementation

All agents working on Courseo (Argon, Neon, Hydrogen) must strictly adhere to this architectural directive:
* Backend modules (`crypto`, `watermark`, `manifest`, `telemetry`, `hardware_protection`) must be implemented as a clean, idiomatic Rust crate under `#![forbid(unsafe_code)]` and `#![deny(clippy::all)]`.
* Client compositor and anti-tamper shields must be implemented in clean TypeScript/JavaScript using standard WebGL2 shaders and Web APIs with zero external heavyweight frameworks.
* All test suites and adversarial attack harnesses must be located strictly under `testscript/`.

---

## 6. Hardware Content Protection, EME/CDM Integration & Desktop Blind Surface Architecture

### 6.1 OS-Level Window Blinding & Display Affinity
To prevent OS-level screen capture tools (QuickTime, OBS, Cmd+Shift+5, ShadowPlay) from capturing the video stream, the system binds OS hardware display affinity:
1. **macOS WindowServer (`NSWindowSharingNone`)**: The OS window is marked with `sharingType = 0` (`NSWindowSharingNone`), legally and cryptographically instructing WindowServer to omit the window buffer from all screen captures, rendering a pure black surface.
2. **Windows DWM (`SetWindowDisplayAffinity`)**: Marked with `WDA_MONITOR` (0x01) or `WDA_EXCLUDEFROMCAPTURE` (0x11), rendering a black box to non-monitor capture APIs.
3. **Android SurfaceView (`FLAG_SECURE`)**: Window flag `0x00002000` prevents screenshots and screen recording.

### 6.2 ISO/IEC 23001-7 Protection System Specific Header (PSSH) Box Engine
Media init segments require standard PSSH metadata boxes for browser EME (Encrypted Media Extensions) to route keys to the hardware Content Decryption Module (CDM):
* **Widevine System ID**: `edef8ba9-79d6-4ace-a3c8-27dcd51d21ed`
* **FairPlay System ID**: `94ce86fb-07ff-4f43-adb8-93d2fa968ca2`
* **PlayReady System ID**: `9a04f079-9840-4286-ab92-e65be0885f95`
* **Box Structure**: 4-byte big-endian length + 4-byte fourcc `'pssh'` + 1-byte version (0 or 1) + 3-byte flags (0) + 16-byte SystemID + (if v1: 4-byte key count + 16-byte KID array) + 4-byte data length + payload.

### 6.3 EME Hardware License Handshake Protocol
The license server validates client certificate claims and enforces output protection:
* **HDCP Policy**: Requires `HDCP_V2_2` for 1080p/4K streams; falls back to 480p or rejects if external display lacks HDCP 2.2.
* **Key Derivation**: Signs key responses with AES-128-CBCS session keys tied to active subscriber claims.

### 6.4 Desktop Application Bridge (`desktop/courseo_secure_desktop.js`)
For standalone desktop deployments (Electron/Tauri):
* Enforces `mainWindow.setContentProtection(true)` on window creation.
* Disables DevTools and keyboard shortcuts for screenshots (e.g. `PrintScreen`).

---

## 7. Advanced Exploit Mitigation Suite (Hardened Defense-in-Depth)

### 7.1 Non-Linear 2D Spatial-Temporal Anti-Camcording Modulation
To defeat 2-frame temporal averaging attacks (`ffmpeg -vf "tblend=all_mode=average"` that cancel simple $I \pm \alpha$ sine waves):
* **2D Pseudo-Random Spatial-Temporal Hash**:
  $$p(x, y, t) = \text{sign}\left(\sin(2\pi \cdot f_d \cdot t + \phi(x, y)) + \gamma \cdot \text{hash2D}(\lfloor x/B \rfloor, \lfloor y/B \rfloor, t)\right)$$
* **Anti-Averaging Invariant**: When two consecutive frames $t$ and $t+1$ are averaged:
  $$\frac{L(x, y, t) + L(x, y, t+1)}{2} = L_0 + \Delta(x, y)$$
  where residual distortion $\Delta(x, y)$ maintains $\ge 15\%$ contrast variance across 8x8 spatial tiles, rendering blended/de-banded footage heavily ghosted and unwatchable while maintaining smooth human retinal integration.

### 7.2 Hypervisor & Virtual Machine Environment Detection
To defeat host-level screen recording of guest VMs (VirtualBox, VMware, QEMU, Cloud Desktops):
* **GPU Renderer Inspection**: Queries `WEBGL_debug_renderer_info` and native display drivers for virtualized GPU adapters (`llvmpipe`, `vmware`, `virtualbox`, `qemu`, `parallels`, `angle (software)`, `microsoft basic render driver`).
* **Timing Jitter Analysis**: Measures High-Resolution Timer (`performance.now()` / `clock_gettime`) variance across 100 iterations; hypervisor context-switching induces microsecond jitter spikes $>50\mu\text{s}$.
* **Revocation Policy**: Detected VMs trigger immediate session termination (`DrmError::HypervisorDetected`).

### 7.3 Psychoacoustic Audio Spread-Spectrum Watermarking (DSSS)
To protect against audio extraction (virtual audio loopbacks, BlackHole, mic recording):
* **Modulation**: Direct Sequence Spread Spectrum (DSSS) modulating a 48-bit subscriber ID across 15kHz–19kHz psychoacoustic masking bands using a 127-bit Gold / m-sequence pseudo-noise (PN) code.
* **Perceptual Inaudibility**: Signal energy kept below the Absolute Threshold of Hearing (ATH) and psychoacoustically masked by lecture audio frequencies.
* **Attribution**: Blind cross-correlation extraction recovers subscriber ID from audio-only rips even after lossy AAC/MP3 transcoding.

### 7.4 Short-Window Chrominance / DCT Per-Frame Watermarking
To attribute leaks from short 5–15 second social media clips (TikTok, Telegram, YouTube Shorts):
* Embeds mid-frequency DCT coefficients in $Cb/Cr$ chrominance blocks across keyframes.
* Blind extractor accumulates energy across $\le 300$ frames to achieve $\ge 99.99\%$ leaker identification without requiring entire video chunks.
