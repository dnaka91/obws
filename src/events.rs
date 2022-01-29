//! All events that can be received from the API.

use std::{collections::BTreeMap, path::PathBuf};

use serde::Deserialize;
use time::Duration;

use crate::{common::MediaAction, responses::SceneItemTransform, MonitorType};

/// All possible event types that can occur while the user interacts with OBS.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "eventType", content = "eventData")]
pub enum Event {
    // --------------------------------
    // Config
    // --------------------------------
    /// The current scene collection has begun changing.
    ///
    /// **Note:** We recommend using this event to trigger a pause of all polling requests, as
    /// performing any requests during a scene collection change is considered undefined behavior
    /// and can cause crashes!
    #[serde(rename_all = "camelCase")]
    CurrentSceneCollectionChanging {
        /// Name of the current scene collection.
        scene_collection_name: String,
    },
    /// The current scene collection has changed.
    ///
    /// **Note:** If polling has been paused during [`CurrentSceneCollectionChanging`], this is the
    /// indicator to restart polling.
    ///
    /// [`CurrentSceneCollectionChanging`]: Event::CurrentSceneCollectionChanging
    #[serde(rename_all = "camelCase")]
    CurrentSceneCollectionChanged {
        /// Name of the new scene collection.
        scene_collection_name: String,
    },
    /// The scene collection list has changed.
    #[serde(rename_all = "camelCase")]
    SceneCollectionListChanged {
        /// Updated list of scene collections.
        scene_collections: Vec<String>,
    },
    /// The current profile has begun changing.
    #[serde(rename_all = "camelCase")]
    CurrentProfileChanging {
        /// Name of the current profile.
        profile_name: String,
    },
    /// The current profile has changed.
    #[serde(rename_all = "camelCase")]
    CurrentProfileChanged {
        /// Name of the new profile.
        profile_name: String,
    },
    /// The profile list has changed.
    #[serde(rename_all = "camelCase")]
    ProfileListChanged {
        /// Updated list of profiles.
        profiles: Vec<String>,
    },
    // --------------------------------
    // Filters
    // --------------------------------
    // --------------------------------
    // General
    // --------------------------------
    CustomEvent(serde_json::Value),
    /// OBS has begun the shutdown process.
    ExitStarted,
    /// An event has been emitted from a vendor.
    ///
    /// A vendor is a unique name registered by a third-party plugin or script, which allows for
    /// custom requests and events to be added to obs-websocket. If a plugin or script implements
    /// vendor requests or events, documentation is expected to be provided with them.
    #[serde(rename_all = "camelCase")]
    VendorEvent {
        /// Name of the vendor emitting the event.
        vendor_name: String,
        /// Vendor-provided event type definition.
        event_type: String,
        /// Vendor-provided event data. `{}` if event does not provide any data.
        event_data: serde_json::Value,
    },
    // --------------------------------
    // Inputs
    // --------------------------------
    /// An input has been created.
    #[serde(rename_all = "camelCase")]
    InputCreated {
        /// Name of the input.
        input_name: String,
        /// The kind of the input.
        input_kind: String,
        /// The unversioned kind of input (aka no `_v2` stuff).
        unversioned_input_kind: String,
        /// The settings configured to the input when it was created.
        input_settings: serde_json::Value,
        /// The default settings for the input.
        default_input_settings: serde_json::Value,
    },
    /// An input has been removed.
    #[serde(rename_all = "camelCase")]
    InputRemoved {
        /// Name of the input.
        input_name: String,
    },
    /// The name of an input has changed.
    #[serde(rename_all = "camelCase")]
    InputNameChanged {
        /// Old name of the input.
        old_input_name: String,
        /// New name of the input.
        input_name: String,
    },
    /// An input's active state has changed.
    ///
    /// When an input is active, it means it's being shown by the program feed.
    #[serde(rename_all = "camelCase")]
    InputActiveStateChanged {
        /// Name of the input.
        input_name: String,
        /// Whether the input is active.
        video_active: bool,
    },
    /// An input's show state has changed.
    ///
    /// When an input is showing, it means it's being shown by the preview or a dialog.
    #[serde(rename_all = "camelCase")]
    InputShowStateChanged {
        /// Name of the input.
        input_name: String,
        /// Whether the input is showing.
        video_showing: bool,
    },
    /// An input's mute state has changed.
    #[serde(rename_all = "camelCase")]
    InputMuteStateChanged {
        /// Name of the input.
        input_name: String,
        /// Whether the input is muted.
        input_muted: bool,
    },
    /// An input's volume level has changed.
    #[serde(rename_all = "camelCase")]
    InputVolumeChanged {
        /// Name of the input.
        input_name: String,
        /// New volume level in `multimap`.
        input_volume_mul: f64,
        /// New volume level in `dB`.
        input_volume_db: f64,
    },
    /// The audio balance value of an input has changed.
    #[serde(rename_all = "camelCase")]
    InputAudioBalanceChanged {
        /// Name of the affected input.
        input_name: String,
        /// New audio balance value of the input.
        input_audio_balance: f64,
    },
    /// The sync offset of an input has changed.
    #[serde(rename_all = "camelCase")]
    InputAudioSyncOffsetChanged {
        /// Name of the input.
        input_name: String,
        /// New sync offset in milliseconds.
        #[serde(deserialize_with = "crate::de::duration_millis")]
        input_audio_sync_offset: Duration,
    },
    /// The audio tracks of an input have changed.
    #[serde(rename_all = "camelCase")]
    InputAudioTracksChanged {
        /// Name of the input.
        input_name: String,
        /// Object of audio tracks along with their associated enable states.
        input_audio_tracks: BTreeMap<String, bool>,
    },
    /// The monitor type of an input has changed.
    #[serde(rename_all = "camelCase")]
    InputAudioMonitorTypeChanged {
        /// Name of the input.
        input_name: String,
        /// New monitor type of the input.
        monitor_type: MonitorType,
    },
    /// A high-volume event providing volume levels of all active inputs every 50 milliseconds.
    #[serde(rename_all = "camelCase")]
    InputVolumeMeters {
        /// Array of active inputs with their associated volume levels.
        inputs: Vec<InputVolumeMeter>,
    },
    // --------------------------------
    // Media Inputs
    // --------------------------------
    /// A media input has started playing.
    #[serde(rename_all = "camelCase")]
    MediaInputPlaybackStarted {
        /// Name of the input.
        input_name: String,
    },
    /// A media input has finished playing.
    #[serde(rename_all = "camelCase")]
    MediaInputPlaybackEnded {
        /// Name of the input.
        input_name: String,
    },
    /// An action has been performed on an input.
    #[serde(rename_all = "camelCase")]
    MediaInputActionTriggered {
        /// Name of the input.
        input_name: String,
        /// Action performed on the input.
        media_action: MediaAction,
    },
    // --------------------------------
    // Outputs
    // --------------------------------
    /// The state of the stream output has changed.
    #[serde(rename_all = "camelCase")]
    StreamStateChanged {
        /// Whether the output is active.
        output_active: bool,
        /// The specific state of the output.
        output_state: OutputState,
    },
    /// The state of the record output has changed.
    #[serde(rename_all = "camelCase")]
    RecordStateChanged {
        /// Whether the output is active.
        output_active: bool,
        /// The specific state of the output.
        output_state: OutputState,
    },
    /// The state of the replay buffer output has changed.
    #[serde(rename_all = "camelCase")]
    ReplayBufferStateChanged {
        /// Whether the output is active.
        output_active: bool,
        /// The specific state of the output.
        output_state: OutputState,
    },
    /// The state of the virtual cam output has changed.
    #[serde(rename_all = "camelCase")]
    VirtualcamStateChanged {
        /// Whether the output is active.
        output_active: bool,
        /// The specific state of the output.
        output_state: OutputState,
    },
    /// The replay buffer has been saved.
    #[serde(rename_all = "camelCase")]
    ReplayBufferSaved {
        /// Path of the saved replay file.
        saved_replay_path: PathBuf,
    },
    // --------------------------------
    // Scene Items
    // --------------------------------
    /// A scene item has been created.
    #[serde(rename_all = "camelCase")]
    SceneItemCreated {
        /// Name of the scene the item was added to.
        scene_name: String,
        /// Name of the underlying source (input/scene).
        source_name: String,
        /// Numeric ID of the scene item.
        scene_item_id: u64,
        /// Index position of the item.
        scene_item_index: u32,
    },
    /// A scene item has been removed.
    ///
    /// This event is not emitted when the scene the item is in is removed.
    #[serde(rename_all = "camelCase")]
    SceneItemRemoved {
        /// Name of the scene the item was removed from.
        scene_name: String,
        /// Name of the underlying source (input/scene).
        input_name: String,
        /// Numeric ID of the scene item.
        scene_item_id: u64,
    },
    /// A scene's item list has been re-indexed.
    #[serde(rename_all = "camelCase")]
    SceneItemListReindexed {
        /// Name of the scene.
        scene_name: String,
        /// Array of scene item objects.
        scene_items: Vec<BasicSceneItem>,
    },
    /// A scene item's enable state has changed.
    #[serde(rename_all = "camelCase")]
    SceneItemEnableStateChanged {
        /// Name of the scene the item is in.
        scene_name: String,
        /// Numeric ID of the scene item.
        scene_item_id: u64,
        /// Whether the scene item is enabled (visible).
        scene_item_enabled: bool,
    },
    /// A scene item's lock state has changed.
    #[serde(rename_all = "camelCase")]
    SceneItemLockStateChanged {
        /// Name of the scene the item is in.
        scene_name: String,
        /// Numeric ID of the scene item.
        scene_item_id: u64,
        /// Whether the scene item is locked.
        scene_item_locked: bool,
    },
    /// A scene item has been selected in the UI.
    #[serde(rename_all = "camelCase")]
    SceneItemSelected {
        /// Name of the scene the item is in.
        scene_name: String,
        /// Numeric ID of the scene item.
        scene_item_id: u64,
    },
    /// The transform/crop of a scene item has changed.
    #[serde(rename_all = "camelCase")]
    SceneItemTransformChanged {
        /// The name of the scene the item is in.
        scene_name: String,
        /// Numeric ID of the scene item.
        scene_item_id: u64,
        /// New transform/crop info of the scene item.
        scene_item_transform: SceneItemTransform,
    },
    // --------------------------------
    // Scenes
    // --------------------------------
    /// A new scene has been created.
    #[serde(rename_all = "camelCase")]
    SceneCreated {
        /// Name of the new scene.
        scene_name: String,
        /// Whether the new scene is a group.
        is_group: bool,
    },
    /// A scene has been removed.
    #[serde(rename_all = "camelCase")]
    SceneRemoved {
        /// Name of the removed scene.
        scene_name: String,
        /// Whether the scene was a group.
        is_group: bool,
    },
    /// The name of a scene has changed.
    #[serde(rename_all = "camelCase")]
    SceneNameChanged {
        /// Old name of the scene.
        old_scene_name: String,
        /// New name of the scene.
        scene_name: String,
    },
    /// The current program scene has changed.
    #[serde(rename_all = "camelCase")]
    CurrentProgramSceneChanged {
        /// Name of the scene that was switched to.
        scene_name: String,
    },
    /// The current preview scene has changed.
    #[serde(rename_all = "camelCase")]
    CurrentPreviewSceneChanged {
        /// Name of the scene that was switched to.
        scene_name: String,
    },
    /// The list of scenes has changed.
    #[serde(rename_all = "camelCase")]
    SceneListChanged {
        /// Updated array of scenes.
        scenes: Vec<Scene>,
    },
    // --------------------------------
    // Transitions
    // --------------------------------
    /// The current scene transition has changed.
    #[serde(rename_all = "camelCase")]
    CurrentSceneTransitionChanged {
        /// Name of the new transition.
        transition_name: String,
    },
    /// The current scene transition duration has changed.
    #[serde(rename_all = "camelCase")]
    CurrentSceneTransitionDurationChanged {
        /// Transition duration in milliseconds.
        #[serde(deserialize_with = "crate::de::duration_millis")]
        transition_duration: Duration,
    },
    // --------------------------------
    // UI
    // --------------------------------
    /// Studio mode has been enabled or disabled.
    #[serde(rename_all = "camelCase")]
    StudioModeStateChanged {
        studio_mode_enabled: bool,
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputVolumeMeter {
    pub input_name: String,
    pub input_levels_mul: Vec<[f32; 3]>,
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
    scene_index: usize,
}
