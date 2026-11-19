# Courseo SecureWatch — System Architecture & Design (DESIGN.md)

## 1. High-Level Architecture
Courseo SecureWatch is structured as a modular Rust core library paired with a hardened WebGL2 browser player. The backend/core crate handles media encryption (CENC AES-128-CBCS), forensic watermark payload encoding/decoding (BCH(64,48)), and optical anti-camcording simulation and shader generation.

```
[Raw H.264/HEVC Stream]
        |
        v
+-------------------------------+
| Forensic Watermark Embedder   | <--- Subscriber ID (48-bit) -> BCH(64,48)
+-------------------------------+
        |
        v
+-------------------------------+
| CENC AES-128-CBCS Encryptor   | <--- 1:9 Pattern Encryption, Constant IV
+-------------------------------+
        |
        v
[HLS / DASH Delivery Pipeline]
        |
        v
+-------------------------------+
| Hardened WebGL2 Client Player | <--- EME/CDM Decryption + Anti-Cam Shader
+-------------------------------+
```

## 2. Rolling Shutter Optical Interference Math
Modern smartphone and DSLR cameras use CMOS sensors with an electronic rolling shutter that exposes rows sequentially.

### 2.1 Sensor Timing Model
Given sensor height $H$ (e.g., $1080$ rows) and total readout duration $T_{\text{readout}}$ (e.g., $15\text{ ms}$), the per-row readout offset is:
$$\tau = \frac{T_{\text{readout}}}{H}$$
Row $r \in [0, H-1]$ begins exposure at $t_{\text{start}}(r) = r \cdot \tau$ and integrates incoming display luminance over exposure window $T_{\text{exp}}$ (e.g., $1/250\text{ s}$):
$$I(r) = \frac{1}{T_{\text{exp}}} \int_{t_{\text{start}}(r)}^{t_{\text{start}}(r) + T_{\text{exp}}} L(t)\,dt$$

### 2.2 Temporal Luminance Modulation
The display alternates frame polarity at refresh rate $f_{\text{refresh}} \in \{60\text{ Hz}, 120\text{ Hz}\}$:
$$p(t) = (-1)^{\lfloor t \cdot f_{\text{refresh}} \rfloor}$$
$$L(t) = \text{clamp}(L_0 + p(t) \cdot \alpha, 0, 1)$$
With $L_0 = 0.5$ and $\alpha = 0.35$:
- **Camera Contrast Banding:** Rows integrating primarily across a single polarity frame capture $L_0 \pm \alpha$, while rows spanning a transition capture intermediate values. The resulting Michelson contrast ratio $C = \frac{I_{\max} - I_{\min}}{I_{\max} + I_{\min}}$ exceeds $0.30$ across both 60 Hz and 120 Hz refresh rates.
- **Human Visual System (HVS) Invisibility:** Because the temporal flicker frequency exceeds or matches critical fusion thresholds and consecutive opposite-polarity frames average to $0.5 \cdot ((L_0 + \alpha) + (L_0 - \alpha)) = L_0$, the retinal integration error is strictly $< 10^{-6}$.

## 3. Forensic Watermarking & BCH(64,48) Coding
To survive aggressive transcoding, cropping, and camcording noise, the 48-bit subscriber/session identifier $m(x)$ is protected by a systematic BCH(64,48) code over $GF(2)$:
- **Generator Polynomial:** Degree-16 primitive/BCH generator polynomial $g(x)$.
- **Systematic Encoding:** Compute remainder $r(x) = x^{16} m(x) \bmod g(x)$ and form the 64-bit codeword $c(x) = x^{16} m(x) + r(x)$.
- **Syndrome Decoding:** Extracted noisy bits $y(x) = c(x) + e(x)$ are evaluated against syndromes $S_i = y(\alpha^i)$ to locate and correct bit flips before mapping back to the subscriber ID.

## 4. CENC AES-128-CBCS Frame Format
Media segments adhere to ISO/IEC 23001-7 (`cbcs`):
- **Subsample Structure:** Each video NAL unit is split into a clear header (`bytes_of_clear_data`) and an encrypted payload (`bytes_of_protected_data`).
- **Pattern Encryption:** Within the protected slice, 16-byte AES-CBC blocks follow a `crypt_byte_block : skip_byte_block` pattern (standard 1:9 for video). The cipher state resets to the constant 16-byte IV at the start of each subsample.

## 5. Client-Side WebGL2 Player Protection
- **Dynamic Headroom Clamping:** The GLSL ES 3.00 fragment shader computes per-pixel RGB headroom `min(r, g, b, 1.0 - max(r, g, b))` and clamps `localAlpha = min(alpha, headroom * 0.95)` to avoid color clipping.
- **Spatial-Temporal Checkering:** Combining `frameIndex` with `rowParity` (`mod(floor(gl_FragCoord.y), 2.0)`) further disrupts spatial de-banding filters.
- **Anti-Tamper Guards:** EME/CDM hardware-backed decryption, `preserveDrawingBuffer: false`, and runtime integrity verification of shader uniforms and canvas prototypes.

## 6. Hardware Content Protection & EME/CDM Integration
- **PSSH Box Engine (`src/hardware_protection.rs`):**
  - Synthesizes ISO/IEC 23001-7 Protection System Specific Header boxes for Widevine, FairPlay, and PlayReady.
  - Generates binary `pssh` atoms injected into media initialization segments (`moov/trak/mdia/minf/stbl/stsd`).
- **EME License Manager (`EmeLicenseManager`):**
  - Validates client challenges, checks subscriber token validity and expiration, and evaluates display link HDCP compliance.
  - Rejects streams requiring `HDCP_V2_2` when the client reports `HdcpLevel::None` or `HdcpLevel::V1_4`.
- **OS Window Capture Blinding (`DisplayAffinityPolicy`):**
  - Models OS-level display affinity masks:
    * macOS: `NSWindowSharingNone` (0)
    * Windows: `WDA_EXCLUDEFROMCAPTURE` (0x11)
    * Android: `FLAG_SECURE` (0x00002000)
- **Desktop Electron/Tauri Bridge (`desktop/courseo_secure_desktop.js`):**
  - Enforces `mainWindow.setContentProtection(true)` on window creation to blind macOS WindowServer and Windows DWM during QuickTime/OBS/ShadowPlay.
  - Intercepts and suppresses screenshot shortcuts (`PrintScreen`, `Cmd+Shift+3/4/5`).

## 7. Adversarial Threat Matrix
| Attack Class | Vector | Mitigation | Residual Risk |
|---|---|---|---|
| Optical Capture | Smartphone CMOS rolling shutter | $\alpha = 0.35$ temporal modulation ($C \ge 0.30$) | Global-shutter cinema cameras (mitigated by forensic watermark) |
| Network Sniffing | TLS MITM / segment scraping | CENC AES-128-CBCS + EME license rotation | Compromised CDM key (mitigated by short TTL) |
| Piracy Redistribution | Screen recorder / HDMI capture | BCH(64,48) forensic watermark | Collusion attacks (mitigated by anti-collusion codes) |
| OS Screen Recording | QuickTime, OBS, Cmd+Shift+5 | Hardware content protection (`NSWindowSharingNone`, `WDA_EXCLUDEFROMCAPTURE`, `FLAG_SECURE`) | Root-level kernel display tap (mitigated by HDCP 2.2) |
| Insecure HDMI Output | External capture card / unencrypted display | EME License Manager enforcing HDCP 2.2 output protection | HDCP stripper hardware (mitigated by forensic watermark) |
| Client Tampering | Shader patching / DevTools | Active prototype traps, EME binding, integrity heartbeats | Custom Chromium builds (mitigated by attestation) |
