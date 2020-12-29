//! All events that can be received from the API.

use chrono::Duration;
use serde::Deserialize;

use crate::common::{SceneItem, SceneItemTransform};

/// Events are sent when a recognized action occurs within OBS.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Event {
    #[serde(default, deserialize_with = "crate::de::duration")]
    /// Time elapsed between now and stream start (only present if OBS Studio is streaming).
    pub stream_timecode: Option<Duration>,
    /// Time elapsed between now and recording start (only present if OBS Studio is recording).
    #[serde(default, deserialize_with = "crate::de::duration")]
    pub rec_timecode: Option<Duration>,
    /// The type of event.
    #[serde(flatten)]
    pub ty: EventType,
}

/// All possible event types that can occur while the user interacts with OBS.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "update-type")]
pub enum EventType {
    // --------------------------------
    // Scenes
    // --------------------------------
    /// Indicates a scene change.
    #[serde(rename_all = "kebab-case")]
    SwitchScenes {
        /// The new scene.
        scene_name: String,
        /// List of scene items in the new scene.
        sources: Vec<SceneItem>,
    },
    /// The scene list has been modified. Scenes have been added, removed, or renamed.
    ScenesChanged,
    /// Triggered when switching to another scene collection or when renaming the current scene
    /// collection.
    SceneCollectionChanged,
    /// Triggered when a scene collection is created, added, renamed, or removed.
    SceneCollectionListChanged,
    // --------------------------------
    // Transitions
    // --------------------------------
    /// The active transition has been changed.
    #[serde(rename_all = "kebab-case")]
    SwitchTransition {
        /// The name of the new active transition.
        transition_name: String,
    },
    /// The list of available transitions has been modified. Transitions have been added, removed,
    /// or renamed.
    TransitionListChanged,
    /// The active transition duration has been changed.
    #[serde(rename_all = "kebab-case")]
    TransitionDurationChanged {
        /// New transition duration.
        #[serde(deserialize_with = "crate::de::duration_millis")]
        new_duration: Duration,
    },
    /// A transition (other than "cut") has begun.
    #[serde(rename_all = "kebab-case")]
    TransitionBegin {
        /// Transition name.
        name: String,
        /// Transition type.
        #[serde(rename = "type")]
        ty: String,
        /// Transition duration (in milliseconds). Will be -1 for any transition with a fixed
        /// duration, such as a Stinger, due to limitations of the OBS API.
        #[serde(deserialize_with = "crate::de::duration_millis_opt")]
        duration: Option<Duration>,
        /// Source scene of the transition.
        from_scene: String,
        /// Destination scene of the transition.
        to_scene: String,
    },
    /// A transition (other than "cut") has ended. Please note that the `from_scene` field is not
    /// available in TransitionEnd.
    #[serde(rename_all = "kebab-case")]
    TransitionEnd {
        /// Transition name.
        name: String,
        /// Transition type.
        #[serde(rename = "type")]
        ty: String,
        /// Transition duration (in milliseconds).
        #[serde(deserialize_with = "crate::de::duration_millis")]
        duration: Duration,
        /// Destination scene of the transition.
        to_scene: String,
    },
    /// A stinger transition has finished playing its video.
    #[serde(rename_all = "kebab-case")]
    TransitionVideoEnd {
        /// Transition name.
        name: String,
        /// Transition type.
        #[serde(rename = "type")]
        ty: String,
        /// Transition duration (in milliseconds).
        #[serde(deserialize_with = "crate::de::duration_millis")]
        duration: Duration,
        /// Source scene of the transition.
        from_scene: String,
        /// Destination scene of the transition.
        to_scene: String,
    },
    // --------------------------------
    // Profiles
    // --------------------------------
    /// Triggered when switching to another profile or when renaming the current profile.
    ProfileChanged,
    /// Triggered when a profile is created, added, renamed, or removed.
    ProfileListChanged,
    // --------------------------------
    // Streaming
    // --------------------------------
    /// A request to start streaming has been issued.
    #[serde(rename_all = "kebab-case")]
    StreamStarting {
        /// Always false (retrocompatibility).
        #[serde(default)]
        preview_only: bool,
    },
    /// Streaming started successfully.
    StreamStarted,
    /// A request to stop streaming has been issued.
    #[serde(rename_all = "kebab-case")]
    StreamStopping {
        /// Always false (retrocompatibility).
        #[serde(default)]
        preview_only: bool,
    },
    /// Streaming stopped successfully.
    StreamStopped,
    /// Emitted every 2 seconds when stream is active.
    #[serde(rename_all = "kebab-case")]
    StreamStatus {
        /// Current streaming state.
        streaming: bool,
        /// Current recording state.
        recording: bool,
        /// Replay Buffer status.
        replay_buffer_active: bool,
        /// Amount of data per second (in bytes) transmitted by the stream encoder.
        bytes_per_sec: u64,
        /// Amount of data per second (in kilobits) transmitted by the stream encoder.
        kbits_per_sec: u64,
        /// Percentage of dropped frames.
        strain: f64,
        /// Total time (in seconds) since the stream started.
        total_stream_time: u64,
        /// Total number of frames transmitted since the stream started.
        num_total_frames: u64,
        /// Number of frames dropped by the encoder since the stream started.
        num_dropped_frames: u64,
        /// Current framerate.
        fps: f64,
        /// Number of frames rendered.
        render_total_frames: u64,
        /// Number of frames missed due to rendering lag.
        render_missed_frames: u64,
        /// Number of frames outputted.
        output_total_frames: u64,
        /// Number of frames skipped due to encoding lag.
        output_skipped_frames: u64,
        /// Average frame time (in milliseconds).
        average_frame_time: f64,
        /// Current CPU usage (percentage).
        cpu_usage: f64,
        /// Current RAM usage (in megabytes).
        memory_usage: f64,
        /// Free recording disk space (in megabytes).
        free_disk_space: f64,
        /// Always false (retrocompatibility).
        #[serde(default)]
        preview_only: bool,
    },
    // --------------------------------
    // Recording
    // --------------------------------
    /// A request to start recording has been issued.
    RecordingStarting,
    /// Recording started successfully.
    RecordingStarted,
    /// A request to stop recording has been issued.
    RecordingStopping,
    /// Recording stopped successfully.
    RecordingStopped,
    /// Current recording paused.
    RecordingPaused,
    /// Current recording resumed.
    RecordingResumed,
    // --------------------------------
    // Replay Buffer
    // --------------------------------
    /// A request to start the replay buffer has been issued.
    ReplayStarting,
    /// Replay Buffer started successfully.
    ReplayStarted,
    /// A request to stop the replay buffer has been issued.
    ReplayStopping,
    /// Replay Buffer stopped successfully.
    ReplayStopped,
    // --------------------------------
    // Other
    // --------------------------------
    /// OBS is exiting.
    Exiting,
    // --------------------------------
    // General
    // --------------------------------
    /// A custom broadcast message, sent by the server, requested by one of the websocket clients.
    BroadcastCustomMessage {
        /// Identifier provided by the sender.
        realm: String,
        /// User-defined data.
        data: serde_json::Map<String, serde_json::Value>,
    },
    // --------------------------------
    // Sources
    // --------------------------------
    /// A source has been created. A source can be an input, a scene or a transition.
    #[serde(rename_all = "camelCase")]
    SourceCreated {
        /// Source name.
        source_name: String,
        /// Source type. Can be "input", "scene", "transition" or "filter".
        source_type: SourceType,
        /// Source kind.
        source_kind: String,
        /// Source settings.
        source_settings: serde_json::Value,
    },
    /// A source has been destroyed/removed. A source can be an input, a scene or a transition.
    #[serde(rename_all = "camelCase")]
    SourceDestroyed {
        /// Source name.
        source_name: String,
        /// Source type. Can be "input", "scene", "transition" or "filter".
        source_type: SourceType,
        /// Source kind.
        source_kind: String,
    },
    /// The volume of a source has changed.
    #[serde(rename_all = "camelCase")]
    SourceVolumeChanged {
        /// Source name.
        source_name: String,
        /// Source volume.
        volume: f32,
    },
    /// A source has been muted or unmuted.
    #[serde(rename_all = "camelCase")]
    SourceMuteStateChanged {
        /// Source name.
        source_name: String,
        /// Mute status of the source.
        muted: bool,
    },
    /// A source has removed audio.
    #[serde(rename_all = "camelCase")]
    SourceAudioDeactivated {
        /// Source name.
        source_name: String,
    },
    /// A source has added audio.
    #[serde(rename_all = "camelCase")]
    SourceAudioActivated {
        /// Source name.
        source_name: String,
    },
    /// The audio sync offset of a source has changed.
    #[serde(rename_all = "camelCase")]
    SourceAudioSyncOffsetChanged {
        /// Source name.
        source_name: String,
        /// Audio sync offset of the source (in nanoseconds).
        #[serde(deserialize_with = "crate::de::duration_nanos")]
        sync_offset: Duration,
    },
    /// Audio mixer routing changed on a source.
    #[serde(rename_all = "camelCase")]
    SourceAudioMixersChanged {
        /// Source name.
        source_name: String,
        /// Routing status of the source for each audio mixer (array of 6 values).
        mixers: [AudioMixer; 6],
        /// Raw mixer flags (little-endian, one bit per mixer) as an hexadecimal value.
        hex_mixers_value: String,
    },
    /// A source has been renamed.
    #[serde(rename_all = "camelCase")]
    SourceRenamed {
        /// Previous source name.
        previous_name: String,
        /// New source name.
        new_name: String,
        /// Type of source (input, scene, filter, transition).
        source_type: SourceType,
    },
    /// A filter was added to a source.
    #[serde(rename_all = "camelCase")]
    SourceFilterAdded {
        /// Source name.
        source_name: String,
        /// Filter name.
        filter_name: String,
        /// Filter type.
        filter_type: String,
        /// Filter settings.
        filter_settings: serde_json::Value,
    },
    /// A filter was removed from a source.
    #[serde(rename_all = "camelCase")]
    SourceFilterRemoved {
        /// Source name.
        source_name: String,
        /// Filter name.
        filter_name: String,
        /// Filter type.
        filter_type: String,
    },
    /// The visibility/enabled state of a filter changed.
    #[serde(rename_all = "camelCase")]
    SourceFilterVisibilityChanged {
        /// Source name.
        source_name: String,
        /// Filter name.
        filter_name: String,
        /// New filter state.
        filter_enabled: bool,
    },
    /// Filters in a source have been reordered.
    #[serde(rename_all = "camelCase")]
    SourceFiltersReordered {
        /// Source name.
        source_name: String,
        /// Ordered Filters list.
        filters: Vec<SourceFilter>,
    },
    // --------------------------------
    // Scene Items
    // --------------------------------
    /// Scene items within a scene have been reordered.
    #[serde(rename_all = "kebab-case")]
    SourceOrderChanged {
        /// Name of the scene where items have been reordered.
        scene_name: String,
        /// Ordered list of scene items.
        scene_items: Vec<SourceOrderSceneItem>,
    },
    /// A scene item has been added to a scene.
    #[serde(rename_all = "kebab-case")]
    SceneItemAdded {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item added to the scene.
        item_name: String,
        /// Scene item ID.
        item_id: i64,
    },
    /// A scene item has been removed from a scene.
    #[serde(rename_all = "kebab-case")]
    SceneItemRemoved {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item removed from the scene.
        item_name: String,
        /// Scene item ID.
        item_id: i64,
    },
    /// A scene item's visibility has been toggled.
    #[serde(rename_all = "kebab-case")]
    SceneItemVisibilityChanged {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Scene item ID.
        item_id: i64,
        /// New visibility state of the item.
        item_visible: bool,
    },
    /// A scene item's locked status has been toggled.
    #[serde(rename_all = "kebab-case")]
    SceneItemLockChanged {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Scene item ID.
        item_id: i64,
        /// New locked state of the item.
        item_locked: bool,
    },
    /// A scene item's transform has been changed.
    #[serde(rename_all = "kebab-case")]
    SceneItemTransformChanged {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// Scene item ID.
        item_id: i64,
        /// Scene item transform properties.
        transform: SceneItemTransform,
    },
    /// A scene item is selected.
    #[serde(rename_all = "kebab-case")]
    SceneItemSelected {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// ID of the item in the scene.
        item_id: i64,
    },
    /// A scene item is deselected.
    #[serde(rename_all = "kebab-case")]
    SceneItemDeselected {
        /// Name of the scene.
        scene_name: String,
        /// Name of the item in the scene.
        item_name: String,
        /// ID of the item in the scene.
        item_id: i64,
    },
    // --------------------------------
    // Studio Mode
    // --------------------------------
    /// The selected preview scene has changed (only available in Studio Mode).
    #[serde(rename_all = "kebab-case")]
    PreviewSceneChanged {
        /// Name of the scene being previewed.
        scene_name: String,
        /// List of sources composing the scene.
        sources: Vec<SceneItem>,
    },
    /// Studio Mode has been enabled or disabled.
    #[serde(rename_all = "kebab-case")]
    StudioModeSwitched {
        /// The new enabled state of Studio Mode.
        new_state: bool,
    },
    /// Fallback value for any unknown event type.
    #[serde(other)]
    Unknown,
}

/// Part of [`EventType::SceneCollectionListChanged`].
#[derive(Clone, Debug, Deserialize)]
pub struct SceneCollection {
    /// Scene collection name.
    pub name: String,
}

/// Part of [`EventType::ProfileListChanged`].
#[derive(Clone, Debug, Deserialize)]
pub struct Profile {
    /// Profile name.
    pub name: String,
}

/// Part of [`EventType::SourceAudioMixersChanged`].
#[derive(Clone, Debug, Deserialize)]
pub struct AudioMixer {
    /// Mixer number.
    pub id: i64,
    /// Routing status.
    pub enabled: bool,
}

/// Part of [`EventType::SourceFiltersReordered`].
#[derive(Clone, Debug, Deserialize)]
pub struct SourceFilter {
    /// Filter name.
    pub name: String,
    /// Filter type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Filter visibility status.
    pub enabled: bool,
}

/// Part of [`EventType::SourceOrderChanged`].
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SourceOrderSceneItem {
    /// Item source name.
    pub source_name: String,
    /// Scene item unique ID.
    pub item_id: i64,
}

/// Part of [`EventType::SourceCreated`], [`EventType::SourceDestroyed`] and
/// [`EventType::SourceRenamed`].
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    /// An input source.
    Input,
    /// A scene.
    Scene,
    /// Transition between scenes.
    Transition,
    /// Filter for scene items.
    Filter,
}
