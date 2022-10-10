//! Responses related to media inputs.

use serde::{Deserialize, Serialize};
use time::Duration;

/// Response value for [`crate::client::MediaInputs::status`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MediaStatus {
    /// State of the media input.
    #[serde(rename = "mediaState")]
    pub state: MediaState,
    /// Total duration of the playing media. [`None`] if not playing.
    #[serde(
        rename = "mediaDuration",
        with = "crate::serde::duration_millis::option"
    )]
    pub duration: Option<Duration>,
    /// Position of the cursor. [`None`] if not playing.
    #[serde(rename = "mediaCursor", with = "crate::serde::duration_millis::option")]
    pub cursor: Option<Duration>,
}

/// Response value for [`crate::client::MediaInputs::status`] as part of [`MediaStatus`].
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum MediaState {
    /// No state.
    #[default]
    #[serde(rename = "OBS_MEDIA_STATE_NONE")]
    None,
    /// Media is playing.
    #[serde(rename = "OBS_MEDIA_STATE_PLAYING")]
    Playing,
    /// Opening file for replay.
    #[serde(rename = "OBS_MEDIA_STATE_OPENING")]
    Opening,
    /// Buffering data for replay.
    #[serde(rename = "OBS_MEDIA_STATE_BUFFERING")]
    Buffering,
    /// Media is paused.
    #[serde(rename = "OBS_MEDIA_STATE_PAUSED")]
    Paused,
    /// Media stopped.
    #[serde(rename = "OBS_MEDIA_STATE_STOPPED")]
    Stopped,
    /// All media in the play-list played.
    #[serde(rename = "OBS_MEDIA_STATE_ENDED")]
    Ended,
    /// Error occurred while trying to play the media.
    #[serde(rename = "OBS_MEDIA_STATE_ERROR")]
    Error,
    /// Unknown state.
    #[serde(other)]
    Unknown,
}
