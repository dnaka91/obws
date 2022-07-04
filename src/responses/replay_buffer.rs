//! Responses related to the replay buffer.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct OutputActive {
    /// New state of the stream output.
    #[serde(rename = "outputActive")]
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SavedReplayPath {
    #[serde(rename = "savedReplayPath")]
    pub saved_replay_path: String,
}
