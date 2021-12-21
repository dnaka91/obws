use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Alignment: u32 {
        const CENTER = 0;
        const LEFT = 1 << 0;
        const RIGHT = 1 << 1;
        const TOP = 1 << 2;
        const BOTTOM = 1 << 3;
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BoundsType {
    #[serde(rename = "OBS_BOUNDS_NONE")]
    None,
    #[serde(rename = "OBS_BOUNDS_STRETCH")]
    Stretch,
    #[serde(rename = "OBS_BOUNDS_SCALE_INNER")]
    ScaleInner,
    #[serde(rename = "OBS_BOUNDS_SCALE_OUTER")]
    ScaleOuter,
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_WIDTH")]
    ScaleToWidth,
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_HEIGHT")]
    ScaleToHeight,
    #[serde(rename = "OBS_BOUNDS_MAX_ONLY")]
    MaxOnly,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum MediaAction {
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_NONE")]
    None,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PLAY")]
    Play,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PAUSE")]
    Pause,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_STOP")]
    Stop,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_RESTART")]
    Restart,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_NEXT")]
    Next,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PREVIOUS")]
    Previous,
}
