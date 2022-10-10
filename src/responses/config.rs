//! Responses related to the OBS configuration.

use serde::{Deserialize, Serialize};

/// Response value for [`crate::client::Config::video_settings`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VideoSettings {
    /// Numerator of the fractional FPS value.
    #[serde(rename = "fpsNumerator")]
    pub fps_numerator: u32,
    /// Denominator of the fractional FPS value.
    #[serde(rename = "fpsDenominator")]
    pub fps_denominator: u32,
    /// Width of the base (canvas) resolution in pixels.
    #[serde(rename = "baseWidth")]
    pub base_width: u32,
    /// Height of the base (canvas) resolution in pixels.
    #[serde(rename = "baseHeight")]
    pub base_height: u32,
    /// Width of the output resolution in pixels.
    #[serde(rename = "outputWidth")]
    pub output_width: u32,
    /// Height of the output resolution in pixels.
    #[serde(rename = "outputHeight")]
    pub output_height: u32,
}

/// Response value for [`crate::client::Config::stream_service_settings`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StreamServiceSettings<T> {
    /// Stream service type, like `rtmp_custom` or `rtmp_common`.
    #[serde(rename = "streamServiceType")]
    pub r#type: String,
    /// Stream service settings.
    #[serde(rename = "streamServiceSettings")]
    pub settings: T,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RecordDirectory {
    /// Output directory.
    #[serde(rename = "recordDirectory")]
    pub record_directory: String,
}
