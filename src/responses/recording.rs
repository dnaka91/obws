//! Responses related to recording.

use serde::{Deserialize, Serialize};
use time::Duration;

/// Response value for [`crate::client::Recording::status`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RecordStatus {
    /// Whether the output is active.
    #[serde(rename = "outputActive")]
    pub active: bool,
    /// Whether the output is paused.
    #[serde(rename = "outputPaused")]
    pub paused: bool,
    /// Current formatted time code string for the output.
    #[serde(rename = "outputTimecode", with = "crate::serde::duration_timecode")]
    pub timecode: Duration,
    /// Current duration in milliseconds for the output.
    #[serde(rename = "outputDuration", with = "crate::serde::duration_millis")]
    pub duration: Duration,
    /// Number of bytes sent by the output.
    #[serde(rename = "outputBytes")]
    pub bytes: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputActive {
    /// New state of the stream output.
    #[serde(rename = "outputActive")]
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputStopped {
    /// File name for the saved recording.
    #[serde(rename = "outputPath")]
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct OutputPaused {
    #[serde(rename = "outputPaused")]
    pub paused: bool,
}
