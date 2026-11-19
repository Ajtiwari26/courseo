use crate::error::DrmError;

/// Supported Hardware DRM / Content Decryption Module (CDM) systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmSystem {
    /// Google Widevine (`edef8ba9-79d6-4ace-a3c8-27dcd51d21ed`)
    Widevine,
    /// Apple FairPlay (`94ce86fb-07ff-4f43-adb8-93d2fa968ca2`)
    FairPlay,
    /// Microsoft PlayReady (`9a04f079-9840-4286-ab92-e65be0885f95`)
    PlayReady,
}

impl DrmSystem {
    /// Returns the exact 16-byte ISO/IEC 23001-7 SystemID UUID.
    pub const fn system_id(&self) -> [u8; 16] {
        match self {
            Self::Widevine => [
                0xed, 0xef, 0x8b, 0xa9, 0x79, 0xd6, 0x4a, 0xce, 0xa3, 0xc8, 0x27, 0xdc, 0xd5, 0x1d,
                0x21, 0xed,
            ],
            Self::FairPlay => [
                0x94, 0xce, 0x86, 0xfb, 0x07, 0xff, 0x4f, 0x43, 0xad, 0xb8, 0x93, 0xd2, 0xfa, 0x96,
                0x8c, 0xa2,
            ],
            Self::PlayReady => [
                0x9a, 0x04, 0xf0, 0x79, 0x98, 0x40, 0x42, 0x86, 0xab, 0x92, 0xe6, 0x5b, 0xe0, 0x88,
                0x5f, 0x95,
            ],
        }
    }
}

/// ISO/IEC 23001-7 Protection System Specific Header (`pssh`) box.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PsshBox {
    pub system: DrmSystem,
    pub version: u8,
    pub flags: [u8; 3],
    pub key_ids: Vec<[u8; 16]>,
    pub data: Vec<u8>,
}

impl PsshBox {
    /// Creates a new `PsshBox`. Uses version 1 if `key_ids` is non-empty, otherwise version 0.
    pub fn new(system: DrmSystem, key_ids: Vec<[u8; 16]>, data: Vec<u8>) -> Self {
        let version = if key_ids.is_empty() { 0 } else { 1 };
        Self {
            system,
            version,
            flags: [0, 0, 0],
            key_ids,
            data,
        }
    }

    /// Synthesizes the ISO/IEC 23001-7 binary `'pssh'` box:
    /// - 4-byte big-endian total box length
    /// - 4-byte fourcc `b"pssh"`
    /// - 1-byte version + 3-byte flags
    /// - 16-byte SystemID
    /// - (if version >= 1) 4-byte big-endian KID count + N * 16-byte KIDs
    /// - 4-byte big-endian data length + binary payload
    pub fn to_bytes(&self) -> Vec<u8> {
        let kid_section_len = if self.version >= 1 {
            4 + (self.key_ids.len() * 16)
        } else {
            0
        };
        // 4 (size) + 4 (fourcc) + 4 (version+flags) + 16 (SystemID) + kid_section + 4 (data_len) + data
        let total_len = 4 + 4 + 4 + 16 + kid_section_len + 4 + self.data.len();
        let mut buf = Vec::with_capacity(total_len);

        buf.extend_from_slice(&(total_len as u32).to_be_bytes());
        buf.extend_from_slice(b"pssh");
        buf.push(self.version);
        buf.extend_from_slice(&self.flags);
        buf.extend_from_slice(&self.system.system_id());

        if self.version >= 1 {
            buf.extend_from_slice(&(self.key_ids.len() as u32).to_be_bytes());
            for kid in &self.key_ids {
                buf.extend_from_slice(kid);
            }
        }

        buf.extend_from_slice(&(self.data.len() as u32).to_be_bytes());
        buf.extend_from_slice(&self.data);
        buf
    }
}

/// HDCP output protection compliance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HdcpLevel {
    None = 0,
    V1_4 = 1,
    V2_2 = 2,
}

/// Client EME license challenge payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmeLicenseChallenge {
    pub subscriber_id: u32,
    pub session_id: String,
    pub token_signature_valid: bool,
    pub token_expiry_unix: u64,
    pub reported_hdcp: HdcpLevel,
    pub drm_system: DrmSystem,
}

/// Evaluated EME license response granted by the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmeLicenseGrant {
    pub session_id: String,
    pub drm_system: DrmSystem,
    pub enforced_hdcp: HdcpLevel,
    pub license_ttl_secs: u64,
}

/// Evaluates EME hardware license challenges, subscriber token claims, and HDCP policies.
#[derive(Debug, Clone)]
pub struct EmeLicenseManager {
    pub required_hdcp: HdcpLevel,
    pub default_license_ttl_secs: u64,
}

impl EmeLicenseManager {
    pub const fn new(required_hdcp: HdcpLevel, default_license_ttl_secs: u64) -> Self {
        Self {
            required_hdcp,
            default_license_ttl_secs,
        }
    }

    /// Evaluates a client challenge against subscriber claims, expiry, and HDCP requirements.
    pub fn evaluate_challenge(
        &self,
        challenge: &EmeLicenseChallenge,
        now_unix: u64,
    ) -> Result<EmeLicenseGrant, DrmError> {
        if !challenge.token_signature_valid || challenge.subscriber_id == 0 {
            return Err(DrmError::Unauthorized(
                "invalid subscriber token signature or identity".to_string(),
            ));
        }

        if now_unix > challenge.token_expiry_unix {
            return Err(DrmError::KeyExpired {
                expired_at: challenge.token_expiry_unix,
                now: now_unix,
            });
        }

        if challenge.reported_hdcp < self.required_hdcp {
            return Err(DrmError::Unauthorized(format!(
                "HDCP policy violation: required {:?}, display reported {:?}",
                self.required_hdcp, challenge.reported_hdcp
            )));
        }

        Ok(EmeLicenseGrant {
            session_id: challenge.session_id.clone(),
            drm_system: challenge.drm_system,
            enforced_hdcp: self.required_hdcp,
            license_ttl_secs: self.default_license_ttl_secs,
        })
    }
}

/// OS-level hardware display affinity & window blinding protection flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum OsDisplayProtection {
    /// macOS `NSWindowSharingNone` (`sharingType = 0`)
    MacSharingNone = 0,
    /// Windows `WDA_EXCLUDEFROMCAPTURE` (`0x11`)
    WindowsExcludeFromCapture = 0x11,
    /// Android `WindowManager.LayoutParams.FLAG_SECURE` (`0x00002000`)
    AndroidFlagSecure = 0x0000_2000,
}

impl OsDisplayProtection {
    /// Returns the raw OS bitmask/flag value.
    pub const fn bitmask(&self) -> u32 {
        *self as u32
    }

    /// Validates whether the observed OS window protection flags satisfy this protection policy.
    pub const fn is_enforced(&self, observed_flags: u32) -> bool {
        match self {
            // macOS NSWindowSharingNone requires sharingType == 0
            Self::MacSharingNone => observed_flags == 0,
            // Windows WDA_EXCLUDEFROMCAPTURE requires bitmask 0x11
            Self::WindowsExcludeFromCapture => (observed_flags & 0x11) == 0x11,
            // Android FLAG_SECURE requires bit 0x00002000 set
            Self::AndroidFlagSecure => (observed_flags & 0x0000_2000) == 0x0000_2000,
        }
    }
}

/// Display affinity policy validator across target operating systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayAffinityPolicy {
    pub protection: OsDisplayProtection,
}

impl DisplayAffinityPolicy {
    pub const fn new(protection: OsDisplayProtection) -> Self {
        Self { protection }
    }

    /// Validates that the window's observed OS flags enforce hardware capture blinding.
    pub fn validate(&self, observed_flags: u32) -> Result<(), DrmError> {
        if self.protection.is_enforced(observed_flags) {
            Ok(())
        } else {
            Err(DrmError::Unauthorized(format!(
                "OS display affinity policy {:?} not enforced (observed flags: 0x{:08x})",
                self.protection, observed_flags
            )))
        }
    }
}
