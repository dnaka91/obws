//! Responses related to outputs.

use serde::Deserialize;
use time::Duration;

#[derive(Debug, Deserialize)]
pub(crate) struct OutputList {
    pub outputs: Vec<Output>,
}

/// Response value for [`crate::client::Outputs::list`].
#[derive(Debug, Deserialize)]
pub struct Output {
    /// Name of this output.
    #[serde(rename = "outputName")]
    pub name: String,
    /// The kind of output.
    #[serde(rename = "outputKind")]
    pub kind: String,
    /// Horizontal dimension of the output (if it is a video output).
    #[serde(rename = "outputWidth")]
    pub width: u32,
    /// Vertical dimension of the output (if it is a video output).
    #[serde(rename = "outputHeight")]
    pub height: u32,
    /// Whether this output is currently active.
    #[serde(rename = "outputActive")]
    pub active: bool,
    /// Additional flags to describe capabilities of the output.
    #[serde(rename = "outputFlags")]
    pub flags: OutputFlags,
}

/// Response value for [`crate::client::Outputs::list`] as part of [`Output`].
#[derive(Debug, Deserialize)]
pub struct OutputFlags {
    /// Output supports audio.
    #[serde(rename = "OBS_OUTPUT_AUDIO")]
    pub audio: bool,
    /// Output supports video.
    #[serde(rename = "OBS_OUTPUT_VIDEO")]
    pub video: bool,
    /// Output encodes data.
    #[serde(rename = "OBS_OUTPUT_ENCODED")]
    pub encoded: bool,
    /// Output supports multiple audio/video tracks.
    #[serde(rename = "OBS_OUTPUT_MULTI_TRACK")]
    pub multi_track: bool,
    /// Output is a service.
    #[serde(rename = "OBS_OUTPUT_SERVICE")]
    pub service: bool,
}

/// Response value for [`crate::client::Outputs::status`].
#[derive(Debug, Deserialize)]
pub struct OutputStatus {
    /// Whether the output is active.
    #[serde(rename = "outputActive")]
    pub active: bool,
    /// Whether the output is currently reconnecting.
    #[serde(rename = "outputReconnecting")]
    pub reconnecting: bool,
    /// Current time code for the output.
    #[serde(rename = "outputTimecode", with = "crate::serde::duration_timecode")]
    pub timecode: Duration,
    /// Current duration for the output.
    #[serde(rename = "outputDuration", with = "crate::serde::duration_millis")]
    pub duration: Duration,
    /// Congestion of the output.
    #[serde(rename = "outputCongestion")]
    pub congestion: f32,
    /// Number of bytes sent by the output.
    #[serde(rename = "outputBytes")]
    pub bytes: u64,
    /// Number of frames skipped by the output's process.
    #[serde(rename = "outputSkippedFrames")]
    pub skipped_frames: u32,
    /// Total number of frames delivered by the output's process.
    #[serde(rename = "outputTotalFrames")]
    pub total_frames: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputActive {
    /// New state of the stream output.
    #[serde(rename = "outputActive")]
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputSettings<T> {
    #[serde(rename = "outputSettings")]
    pub settings: T,
}
