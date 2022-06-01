//! Responses related to the OBS configuration.

use serde::Deserialize;

/// Response value for [`crate::client::Config::video_settings`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSettings {
    /// Numerator of the fractional FPS value.
    pub fps_numerator: u32,
    /// Denominator of the fractional FPS value.
    pub fps_denominator: u32,
    /// Width of the base (canvas) resolution in pixels.
    pub base_width: u32,
    /// Height of the base (canvas) resolution in pixels.
    pub base_height: u32,
    /// Width of the output resolution in pixels.
    pub output_width: u32,
    /// Height of the output resolution in pixels.
    pub output_height: u32,
}

/// Response value for [`crate::client::Config::stream_service_settings`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamServiceSettings<T> {
    /// Stream service type, like `rtmp_custom` or `rtmp_common`.
    #[serde(rename = "streamServiceType")]
    pub r#type: String,
    /// Stream service settings.
    #[serde(rename = "streamServiceSettings")]
    pub settings: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecordDirectory {
    /// Output directory.
    pub record_directory: String,
}
