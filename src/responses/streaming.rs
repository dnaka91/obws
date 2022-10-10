//! Responses related to streaming.

use serde::{Deserialize, Serialize};
use time::Duration;

/// Response value for [`crate::client::Streaming::status`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StreamStatus {
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
