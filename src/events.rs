//! All events that can be received from the API.

use std::{collections::BTreeMap, path::PathBuf};

use chrono::Duration;
use serde::Deserialize;

use crate::{responses::SceneItemTransform, MonitorType};

/// All possible event types that can occur while the user interacts with OBS.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "eventType", content = "eventData")]
pub enum Event {
    // --------------------------------
    // Config
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    CurrentSceneCollectionChanged {
        scene_collection_name: String,
    },
    #[serde(rename_all = "camelCase")]
    SceneCollectionListChanged {
        scene_collections: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    CurrentProfileChanged {
        profile_name: String,
    },
    #[serde(rename_all = "camelCase")]
    ProfileListChanged {
        profiles: Vec<String>,
    },
    // --------------------------------
    // Filters
    // --------------------------------
    // --------------------------------
    // General
    // --------------------------------
    CustomEvent(serde_json::Value),
    ExitStarted,
    #[serde(rename_all = "camelCase")]
    StudioModeStateChanged {
        studio_mode_enabled: bool,
    },
    // --------------------------------
    // Inputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    InputCreated {
        input_name: String,
        input_kind: String,
        unversioned_input_kind: String,
        input_settings: serde_json::Value,
        default_input_settings: serde_json::Value,
    },
    #[serde(rename_all = "camelCase")]
    InputRemoved {
        input_name: String,
    },
    #[serde(rename_all = "camelCase")]
    InputNameChanged {
        old_input_name: String,
        input_name: String,
    },
    #[serde(rename_all = "camelCase")]
    InputActiveStateChanged {
        input_name: String,
        video_active: bool,
    },
    #[serde(rename_all = "camelCase")]
    InputShowStateChanged {
        input_name: String,
        video_showing: bool,
    },
    #[serde(rename_all = "camelCase")]
    InputMuteStateChanged {
        input_name: String,
        input_muted: bool,
    },
    #[serde(rename_all = "camelCase")]
    InputVolumeChanged {
        input_name: String,
        input_volume_mul: f64,
        input_volume_db: f64,
    },
    #[serde(rename_all = "camelCase")]
    InputAudioSyncOffsetChanged {
        input_name: String,
        #[serde(deserialize_with = "crate::de::duration_millis")]
        input_audio_sync_offset: Duration,
    },
    #[serde(rename_all = "camelCase")]
    InputAudioTracksChanged {
        input_name: String,
        input_audio_tracks: BTreeMap<String, bool>,
    },
    #[serde(rename_all = "camelCase")]
    InputAudioMonitorTypeChanged {
        input_name: String,
        monitor_type: MonitorType,
    },
    // --------------------------------
    // Media Inputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    MediaInputPlaybackStarted {
        input_name: String,
    },
    #[serde(rename_all = "camelCase")]
    MediaInputPlaybackEnded {
        input_name: String,
    },
    #[serde(rename_all = "camelCase")]
    MediaInputActionTriggered {
        input_name: String,
        media_action: MediaAction,
    },
    // --------------------------------
    // Outputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    StreamStateChanged {
        output_active: bool,
        output_state: OutputState,
    },
    #[serde(rename_all = "camelCase")]
    RecordStateChanged {
        output_active: bool,
        output_state: OutputState,
    },
    #[serde(rename_all = "camelCase")]
    ReplayBufferStateChanged {
        output_active: bool,
        output_state: OutputState,
    },
    #[serde(rename_all = "camelCase")]
    VirtualcamStateChanged {
        output_active: bool,
        output_state: OutputState,
    },
    #[serde(rename_all = "camelCase")]
    ReplayBufferSaved {
        saved_replay_path: PathBuf,
    },
    // --------------------------------
    // Scene Items
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    SceneItemCreated {
        scene_name: String,
        input_name: String,
        scene_item_id: u64,
        scene_item_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    SceneItemRemoved {
        scene_name: String,
        input_name: String,
        scene_item_id: u64,
        scene_item_index: u32,
    },
    #[serde(rename_all = "camelCase")]
    SceneItemReindexed {
        scene_name: String,
        scene_items: Vec<BasicSceneItem>,
    },
    #[serde(rename_all = "camelCase")]
    SceneItemEnableStateChanged {
        scene_name: String,
        scene_item_id: u64,
        scene_item_enabled: bool,
    },
    #[serde(rename_all = "camelCase")]
    SceneItemLockStateChanged {
        scene_name: String,
        scene_item_id: u64,
        scene_item_locked: bool,
    },
    #[serde(rename_all = "camelCase")]
    SceneItemTransformChanged {
        scene_name: String,
        scene_item_id: u64,
        scene_item_transform: SceneItemTransform,
    },
    // --------------------------------
    // Scenes
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    SceneCreated {
        scene_name: String,
        is_group: bool,
    },
    #[serde(rename_all = "camelCase")]
    SceneRemoved {
        scene_name: String,
        is_group: bool,
    },
    #[serde(rename_all = "camelCase")]
    SceneNameChanged {
        old_scene_name: String,
        scene_name: String,
    },
    #[serde(rename_all = "camelCase")]
    CurrentSceneChanged {
        scene_name: String,
    },
    #[serde(rename_all = "camelCase")]
    CurrentPreviewSceneChanged {
        scene_name: String,
    },
    #[serde(rename_all = "camelCase")]
    SceneListChanged {
        scenes: Vec<Scene>,
    },
    // --------------------------------
    // Transitions
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    TransitionCreated {
        transition_name: String,
        transition_kind: String,
        transition_fixed: bool,
    },
    #[serde(rename_all = "camelCase")]
    TransitionRemoved {
        transition_name: String,
    },
    #[serde(rename_all = "camelCase")]
    TransitionNameChanged {
        old_transition_name: String,
        transition_name: String,
    },
    // --------------------------------
    // Custom
    // --------------------------------
    /// Web-socket server is stopping.
    ServerStopping,
    /// Web-socket server has stopped.
    ServerStopped,
    /// Fallback value for any unknown event type.
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum MediaAction {
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PAUSE")]
    Pause,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PLAY")]
    Play,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_RESTART")]
    Restart,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_STOP")]
    Stop,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_NEXT")]
    Next,
    #[serde(rename = "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_PREVIOUS")]
    Previous,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum OutputState {
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STARTING")]
    Starting,
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STARTED")]
    Started,
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STOPPING")]
    Stopping,
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STOPPED")]
    Stopped,
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_PAUSED")]
    Paused,
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_RESUMED")]
    Resumed,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicSceneItem {
    scene_item_id: u64,
    scene_item_index: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    scene_name: String,
    is_group: bool,
}
