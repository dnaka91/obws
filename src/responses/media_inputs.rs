//! Responses related to media inputs.

use serde::Deserialize;
use time::Duration;

/// Response value for [`crate::client::MediaInputs::status`].
#[derive(Debug, Deserialize)]
pub struct MediaStatus {
    /// State of the media input.
    #[serde(rename = "mediaState")]
    pub state: MediaState,
    /// Total duration of the playing media. [`None`] if not playing.
    #[serde(
        rename = "mediaDuration",
        deserialize_with = "crate::de::duration_millis_opt"
    )]
    pub duration: Option<Duration>,
    /// Position of the cursor. [`None`] if not playing.
    #[serde(
        rename = "mediaCursor",
        deserialize_with = "crate::de::duration_millis_opt"
    )]
    pub cursor: Option<Duration>,
}

/// Response value for [`crate::client::MediaInputs::status`] as part of [`MediaStatus`].
#[derive(Copy, Clone, Debug, Deserialize)]
pub enum MediaState {
    /// No state.
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
