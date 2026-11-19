use crate::error::DrmError;

#[derive(Debug, Clone)]
pub struct RollingShutterParams {
    pub display_refresh_hz: f64,
    pub exposure_sec: f64,
    pub sensor_height: usize,
    pub readout_total_sec: f64,
    pub modulation_alpha: f64,
    pub base_luminance: f64,
}

impl Default for RollingShutterParams {
    fn default() -> Self {
        Self {
            display_refresh_hz: 60.0,
            exposure_sec: 1.0 / 250.0,
            sensor_height: 1080,
            readout_total_sec: 0.015,
            modulation_alpha: 0.35,
            base_luminance: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpticalSimulationReport {
    pub camera_contrast_ratio: f64,
    pub human_retinal_error: f64,
    pub row_profile: Vec<f64>,
}

pub struct AntiCamEngine;

impl AntiCamEngine {
    /// Simulates a CMOS rolling shutter camera capturing the temporally modulated display.
    pub fn simulate_rolling_shutter(
        params: &RollingShutterParams,
    ) -> Result<OpticalSimulationReport, DrmError> {
        if params.display_refresh_hz <= 0.0
            || params.exposure_sec <= 0.0
            || params.sensor_height == 0
            || params.readout_total_sec <= 0.0
        {
            return Err(DrmError::OpticalParamError(
                "all timing and sensor dimensions must be positive".to_string(),
            ));
        }

        let tau = params.readout_total_sec / (params.sensor_height as f64);
        let sub_steps = 32usize;
        let dt = params.exposure_sec / (sub_steps as f64);

        let mut row_profile = Vec::with_capacity(params.sensor_height);
        for row in 0..params.sensor_height {
            let t_start = (row as f64) * tau;
            let mut acc = 0.0f64;
            for step in 0..sub_steps {
                let t = t_start + (step as f64 + 0.5) * dt;
                let frame_idx = (t * params.display_refresh_hz).floor() as i64;
                let polarity = if frame_idx % 2 == 0 { 1.0 } else { -1.0 };
                let lum =
                    (params.base_luminance + polarity * params.modulation_alpha).clamp(0.0, 1.0);
                acc += lum;
            }
            row_profile.push(acc / (sub_steps as f64));
        }

        let min_lum = row_profile.iter().copied().fold(f64::INFINITY, f64::min);
        let max_lum = row_profile
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        let camera_contrast_ratio = if (max_lum + min_lum) > 0.0 {
            (max_lum - min_lum) / (max_lum + min_lum)
        } else {
            0.0
        };

        // Human eye integrates over two consecutive opposite-polarity frames
        let frame_even = (params.base_luminance + params.modulation_alpha).clamp(0.0, 1.0);
        let frame_odd = (params.base_luminance - params.modulation_alpha).clamp(0.0, 1.0);
        let human_perceived = 0.5 * (frame_even + frame_odd);
        let human_retinal_error = (human_perceived - params.base_luminance).abs();

        Ok(OpticalSimulationReport {
            camera_contrast_ratio,
            human_retinal_error,
            row_profile,
        })
    }

    /// Generates the WebGL2 GLSL ES 3.00 fragment shader for real-time anti-camcording modulation.
    pub fn generate_webgl2_fragment_shader(alpha: f32, refresh_hz: f32) -> String {
        format!(
            r#"#version 300 es
precision highp float;

in vec2 v_texCoord;
uniform sampler2D u_videoFrame;
uniform float u_timeSec;
uniform vec2 u_resolution;

out vec4 outColor;

void main() {{
    vec4 src = texture(u_videoFrame, v_texCoord);
    float frameIndex = floor(u_timeSec * {refresh_hz:.1});
    float rowParity = mod(floor(gl_FragCoord.y), 2.0);
    float polarity = (mod(frameIndex + rowParity, 2.0) < 0.5) ? 1.0 : -1.0;

    float headroom = min(min(src.r, src.g), min(src.b, 1.0 - max(max(src.r, src.g), src.b)));
    float localAlpha = min({alpha:.3}, headroom * 0.95);
    vec3 modulated = clamp(src.rgb + vec3(polarity * localAlpha), 0.0, 1.0);

    outColor = vec4(modulated, src.a);
}}
"#
        )
    }
}
