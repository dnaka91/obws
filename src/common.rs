//! Common data structures shared between [`requests`](crate::requests),
//! [`responses`](crate::responses) and [`events`](crate::events).

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::Error;

/// Monitoring type for audio outputs.
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum MonitorType {
    /// No monitoring.
    #[serde(rename = "OBS_MONITORING_TYPE_NONE")]
    None,
    /// Only monitor but don't output any sounds.
    #[serde(rename = "OBS_MONITORING_TYPE_MONITOR_ONLY")]
    MonitorOnly,
    /// Monitor the audio and output it at the same time.
    #[serde(rename = "OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT")]
    MonitorAndOutput,
}

bitflags! {
    /// Different flags for font display that can be combined.
    pub struct FontFlags: u8 {
        /// Make the text appear thicker.
        const BOLD = 1;
        /// Make the text appear cursive.
        const ITALIC = 2;
        /// Underline the text with a straight line.
        const UNDERLINE = 4;
        /// Strikeout the text.
        const STRIKEOUT = 8;
    }
}

impl TryFrom<u8> for FontFlags {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or(Error::UnknownFlags(value))
    }
}

impl From<FontFlags> for u8 {
    fn from(value: FontFlags) -> Self {
        value.bits
    }
}

bitflags! {
    /// Alignment for different items on the scene that is described in two axis. The default is
    /// center for both axis.
    ///
    /// For example, only using `LEFT` would arrange the target to the left horizontally and
    /// centered vertically. To align to the top right, the alignments can be combined to
    /// `LEFT | TOP`. Combining both values for a single axis is invalid, like `LEFT | RIGHT`.
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Alignment: u8 {
        /// Align to the center.
        const CENTER = 0;
        /// Align to the left side.
        const LEFT = 1 << 0;
        /// Align to the right side.
        const RIGHT = 1 << 1;
        /// Align to the top.
        const TOP = 1 << 2;
        /// Align to the bottom.
        const BOTTOM = 1 << 3;
    }
}

impl TryFrom<u8> for Alignment {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or(Error::UnknownFlags(value))
    }
}

impl From<Alignment> for u8 {
    fn from(value: Alignment) -> Self {
        value.bits
    }
}

/// Different kinds of bounds that can be applied to different items on the scene.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BoundsType {
    /// No bounds.
    #[serde(rename = "OBS_BOUNDS_NONE")]
    None,
    /// Stretch to bounds.
    #[serde(rename = "OBS_BOUNDS_STRETCH")]
    Stretch,
    /// Scale to inner bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_INNER")]
    ScaleInner,
    /// Scale to outer bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_OUTER")]
    ScaleOuter,
    /// Scale to width of bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_WIDTH")]
    ScaleToWidth,
    /// Scale to height of bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_HEIGHT")]
    ScaleToHeight,
    /// Maximum size only.
    #[serde(rename = "OBS_BOUNDS_MAX_ONLY")]
    MaxOnly,
}

/// Different kind of media actions that can be performed (or happen in events).
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MediaAction {
    /// No media action.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_NONE")]
    None,
    /// Start media playback.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PLAY")]
    Play,
    /// Pause the current playback.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PAUSE")]
    Pause,
    /// Stop media playback, resetting the playback position back to the start.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_STOP")]
    Stop,
    /// Reset playback to the start and continue playing.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_RESTART")]
    Restart,
    /// Play the next media in the list.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_NEXT")]
    Next,
    /// Play the previous media in the list.
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PREVIOUS")]
    Previous,
}
