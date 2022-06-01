//! Responses related to recording.

use serde::Deserialize;
use time::Duration;

/// Response value for [`crate::client::Recording::status`].
#[derive(Debug, Deserialize)]
pub struct RecordStatus {
    /// Whether the output is active.
    #[serde(rename = "outputActive")]
    pub active: bool,
    /// Whether the output is paused.
    #[serde(rename = "outputPaused")]
    pub paused: bool,
    /// Current formatted time code string for the output.
    #[serde(
        rename = "outputTimecode",
        deserialize_with = "crate::de::duration_timecode"
    )]
    pub timecode: Duration,
    /// Current duration in milliseconds for the output.
    #[serde(
        rename = "outputDuration",
        deserialize_with = "crate::de::duration_millis"
    )]
    pub duration: Duration,
    /// Number of bytes sent by the output.
    #[serde(rename = "outputBytes")]
    pub bytes: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputActive {
    /// New state of the stream output.
    #[serde(rename = "outputActive")]
    pub active: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputPaused {
    #[serde(rename = "outputPaused")]
    pub paused: bool,
}
