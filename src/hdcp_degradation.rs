use crate::hardware_protection::HdcpLevel;

#[derive(Debug, Clone, PartialEq)]
pub struct HdcpPlaybackResolution {
    pub max_height: u32,
    pub watermark_opacity: f32,
    pub tier_name: &'static str,
    pub requires_downgrade: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HdcpPolicyEngine;

impl HdcpPolicyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate_resolution(&self, reported_hdcp: HdcpLevel) -> HdcpPlaybackResolution {
        match reported_hdcp {
            HdcpLevel::V2_2 => HdcpPlaybackResolution {
                max_height: 1080,
                watermark_opacity: 0.22,
                tier_name: "Pristine1080p",
                requires_downgrade: false,
            },
            HdcpLevel::V1_4 => HdcpPlaybackResolution {
                max_height: 720,
                watermark_opacity: 0.45,
                tier_name: "Degraded720p",
                requires_downgrade: true,
            },
            HdcpLevel::None => HdcpPlaybackResolution {
                max_height: 480,
                watermark_opacity: 0.55,
                tier_name: "Degraded480p",
                requires_downgrade: true,
            },
        }
    }
}
