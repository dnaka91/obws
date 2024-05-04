//! All events that can be received from the API.

use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

use crate::{
    common::{MediaAction, MonitorType},
    responses::{
        filters::SourceFilter,
        ids::{SceneId, TransitionId},
        inputs::InputId,
        scene_items::SceneItemTransform,
        sources::SourceId,
    },
};

/// All possible event types that can occur while the user interacts with OBS.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "eventType", content = "eventData")]
#[non_exhaustive]
pub enum Event {
    // --------------------------------
    // Config
    // --------------------------------
    /// The current scene collection has begun changing.
    ///
    /// **Note:** We recommend using this event to trigger a pause of all polling requests, as
    /// performing any requests during a scene collection change is considered undefined behavior
    /// and can cause crashes!
    CurrentSceneCollectionChanging {
        /// Name of the current scene collection.
        #[serde(rename = "sceneCollectionName")]
        name: String,
    },
    /// The current scene collection has changed.
    ///
    /// **Note:** If polling has been paused during [`CurrentSceneCollectionChanging`], this is the
    /// indicator to restart polling.
    ///
    /// [`CurrentSceneCollectionChanging`]: Event::CurrentSceneCollectionChanging
    CurrentSceneCollectionChanged {
        /// Name of the new scene collection.
        #[serde(rename = "sceneCollectionName")]
        name: String,
    },
    /// The scene collection list has changed.
    SceneCollectionListChanged {
        /// Updated list of scene collections.
        #[serde(rename = "sceneCollections")]
        collections: Vec<String>,
    },
    /// The current profile has begun changing.
    CurrentProfileChanging {
        /// Name of the current profile.
        #[serde(rename = "profileName")]
        name: String,
    },
    /// The current profile has changed.
    CurrentProfileChanged {
        /// Name of the new profile.
        #[serde(rename = "profileName")]
        name: String,
    },
    /// The profile list has changed.
    ProfileListChanged {
        /// Updated list of profiles.
        #[serde(rename = "profiles")]
        profiles: Vec<String>,
    },
    // --------------------------------
    // Filters
    // --------------------------------
    /// A filter has been added to a source.
    SourceFilterCreated {
        /// Name of the source the filter was added to.
        #[serde(rename = "sourceName")]
        source: String,
        /// Name of the filter.
        #[serde(rename = "filterName")]
        filter: String,
        /// The kind of the filter.
        #[serde(rename = "filterKind")]
        kind: String,
        /// Index position of the filter.
        #[serde(rename = "filterIndex")]
        index: u32,
        /// The settings configured to the filter when it was created.
        #[serde(rename = "filterSettings")]
        settings: serde_json::Value,
        /// The default settings for the filter.
        #[serde(rename = "defaultFilterSettings")]
        default_settings: serde_json::Value,
    },
    /// A filter has been removed from a source.
    SourceFilterRemoved {
        /// Name of the source the filter was on.
        #[serde(rename = "sourceName")]
        source: String,
        /// Name of the filter.
        #[serde(rename = "filterName")]
        filter: String,
    },
    /// A source's filter list has been re-indexed.
    SourceFilterListReindexed {
        /// Name of the source.
        #[serde(rename = "sourceName")]
        source: String,
        /// Array of filter objects.
        filters: Vec<SourceFilter>,
    },
    /// A source filter's enable state has changed.
    SourceFilterEnableStateChanged {
        /// Name of the source the filter is on.
        #[serde(rename = "sourceName")]
        source: String,
        /// Name of the filter.
        #[serde(rename = "filterName")]
        filter: String,
        /// Whether the filter is enabled.
        #[serde(rename = "filterEnabled")]
        enabled: bool,
    },
    /// The name of a source filter has changed.
    SourceFilterNameChanged {
        /// The source the filter is on.
        #[serde(rename = "sourceName")]
        source: String,
        /// Old name of the filter.
        #[serde(rename = "oldFilterName")]
        old_name: String,
        /// New name of the filter.
        #[serde(rename = "filterName")]
        new_name: String,
    },
    /// A source filter's settings have changed (been updated).
    SourceFilterSettingsChanged {
        /// Name of the source the filter is on.
        #[serde(rename = "sourceName")]
        source: String,
        /// Name of the filter.
        #[serde(rename = "filterName")]
        filter: String,
        /// New settings object of the filter.
        #[serde(rename = "filterSettings")]
        settings: serde_json::Value,
    },
    // --------------------------------
    // General
    // --------------------------------
    /// A custom event that was triggered by
    /// [`crate::client::General::broadcast_custom_event`].
    ///
    /// The content can be any valid JSON object.
    CustomEvent(serde_json::Value),
    /// OBS has begun the shutdown process.
    ExitStarted,
    /// An event has been emitted from a vendor.
    ///
    /// A vendor is a unique name registered by a third-party plugin or script, which allows for
    /// custom requests and events to be added to obs-websocket. If a plugin or script implements
    /// vendor requests or events, documentation is expected to be provided with them.
    VendorEvent {
        /// Name of the vendor emitting the event.
        #[serde(rename = "vendorName")]
        vendor_name: String,
        /// Vendor-provided event type definition.
        #[serde(rename = "eventType")]
        event_type: String,
        /// Vendor-provided event data. `{}` if event does not provide any data.
        #[serde(rename = "eventData")]
        event_data: serde_json::Value,
    },
    // --------------------------------
    // Inputs
    // --------------------------------
    /// An input has been created.
    InputCreated {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// The kind of the input.
        #[serde(rename = "inputKind")]
        kind: String,
        /// The unversioned kind of input (aka no `_v2` stuff).
        #[serde(rename = "unversionedInputKind")]
        unversioned_kind: String,
        /// The settings configured to the input when it was created.
        #[serde(rename = "inputSettings")]
        settings: serde_json::Value,
        /// The default settings for the input.
        #[serde(rename = "defaultInputSettings")]
        default_settings: serde_json::Value,
    },
    /// An input has been removed.
    InputRemoved {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
    },
    /// The name of an input has changed.
    InputNameChanged {
        /// UUID of the input.
        #[serde(rename = "inputUuid")]
        uuid: Uuid,
        /// Old name of the input.
        #[serde(rename = "oldInputName")]
        old_name: String,
        /// New name of the input.
        #[serde(rename = "inputName")]
        new_name: String,
    },
    /// An input's settings have changed (been updated).
    ///
    /// Note: On some inputs, changing values in the properties dialog will cause an immediate
    /// update. Pressing the _Cancel_ button will revert the settings, resulting in another event
    /// being fired.
    InputSettingsChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// New settings object of the input.
        #[serde(rename = "inputSettings")]
        settings: serde_json::Value,
    },
    /// An input's active state has changed.
    ///
    /// When an input is active, it means it's being shown by the program feed.
    InputActiveStateChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// Whether the input is active.
        #[serde(rename = "videoActive")]
        active: bool,
    },
    /// An input's show state has changed.
    ///
    /// When an input is showing, it means it's being shown by the preview or a dialog.
    InputShowStateChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// Whether the input is showing.
        #[serde(rename = "videoShowing")]
        showing: bool,
    },
    /// An input's mute state has changed.
    InputMuteStateChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// Whether the input is muted.
        #[serde(rename = "inputMuted")]
        muted: bool,
    },
    /// An input's volume level has changed.
    InputVolumeChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// New volume level multiplier.
        #[serde(rename = "inputVolumeMul")]
        mul: f64,
        /// New volume level in `dB`.
        #[serde(rename = "inputVolumeDb")]
        db: f64,
    },
    /// The audio balance value of an input has changed.
    InputAudioBalanceChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// New audio balance value of the input.
        #[serde(rename = "inputAudioBalance")]
        audio_balance: f64,
    },
    /// The sync offset of an input has changed.
    InputAudioSyncOffsetChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// New sync offset in milliseconds.
        #[serde(
            rename = "inputAudioSyncOffset",
            with = "crate::serde::duration_millis"
        )]
        offset: Duration,
    },
    /// The audio tracks of an input have changed.
    InputAudioTracksChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// Object of audio tracks along with their associated enable states.
        #[serde(rename = "inputAudioTracks")]
        tracks: BTreeMap<String, bool>,
    },
    /// The monitor type of an input has changed.
    InputAudioMonitorTypeChanged {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// New monitor type of the input.
        #[serde(rename = "monitorType")]
        monitor_type: MonitorType,
    },
    /// A high-volume event providing volume levels of all active inputs every 50 milliseconds.
    InputVolumeMeters {
        /// Array of active inputs with their associated volume levels.
        #[serde(rename = "inputs")]
        inputs: Vec<InputVolumeMeter>,
    },
    // --------------------------------
    // Media Inputs
    // --------------------------------
    /// A media input has started playing.
    MediaInputPlaybackStarted {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
    },
    /// A media input has finished playing.
    MediaInputPlaybackEnded {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
    },
    /// An action has been performed on an input.
    MediaInputActionTriggered {
        /// Identifier of the input.
        #[serde(flatten)]
        id: InputId,
        /// Action performed on the input.
        #[serde(rename = "mediaAction")]
        media_action: MediaAction,
    },
    // --------------------------------
    // Outputs
    // --------------------------------
    /// The state of the stream output has changed.
    StreamStateChanged {
        /// Whether the output is active.
        #[serde(rename = "outputActive")]
        active: bool,
        /// The specific state of the output.
        #[serde(rename = "outputState")]
        state: OutputState,
    },
    /// The state of the record output has changed.
    RecordStateChanged {
        /// Whether the output is active.
        #[serde(rename = "outputActive")]
        active: bool,
        /// The specific state of the output.
        #[serde(rename = "outputState")]
        state: OutputState,
        /// File name for the saved recording, if record stopped.
        #[serde(rename = "outputPath")]
        path: Option<String>,
    },
    /// The state of the replay buffer output has changed.
    ReplayBufferStateChanged {
        /// Whether the output is active.
        #[serde(rename = "outputActive")]
        active: bool,
        /// The specific state of the output.
        #[serde(rename = "outputState")]
        state: OutputState,
    },
    /// The state of the virtual cam output has changed.
    VirtualcamStateChanged {
        /// Whether the output is active.
        #[serde(rename = "outputActive")]
        active: bool,
        /// The specific state of the output.
        #[serde(rename = "outputState")]
        state: OutputState,
    },
    /// The replay buffer has been saved.
    ReplayBufferSaved {
        /// Path of the saved replay file.
        #[serde(rename = "savedReplayPath")]
        path: PathBuf,
    },
    // --------------------------------
    // Scene Items
    // --------------------------------
    /// A scene item has been created.
    SceneItemCreated {
        /// Identifier of the scene the item was added to.
        #[serde(flatten)]
        scene: SceneId,
        /// Identifier of the underlying source (input/scene).
        #[serde(flatten)]
        source: SourceId,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: u64,
        /// Index position of the item.
        #[serde(rename = "sceneItemIndex")]
        index: u32,
    },
    /// A scene item has been removed.
    ///
    /// This event is not emitted when the scene the item is in is removed.
    SceneItemRemoved {
        /// Identifier of the scene the item was removed from.
        #[serde(flatten)]
        scene: SceneId,
        /// Identifier of the underlying source (input/scene).
        #[serde(flatten)]
        source: SourceId,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: u64,
    },
    /// A scene's item list has been re-indexed.
    SceneItemListReindexed {
        /// Identifier of the scene.
        #[serde(flatten)]
        scene: SceneId,
        /// Array of scene item objects.
        #[serde(rename = "sceneItems")]
        items: Vec<BasicSceneItem>,
    },
    /// A scene item's enable state has changed.
    SceneItemEnableStateChanged {
        /// Identifier of the scene the item is in.
        #[serde(flatten)]
        scene: SceneId,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: u64,
        /// Whether the scene item is enabled (visible).
        #[serde(rename = "sceneItemEnabled")]
        enabled: bool,
    },
    /// A scene item's lock state has changed.
    SceneItemLockStateChanged {
        /// Identifier of the scene the item is in.
        #[serde(flatten)]
        scene: SceneId,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: u64,
        /// Whether the scene item is locked.
        #[serde(rename = "sceneItemLocked")]
        locked: bool,
    },
    /// A scene item has been selected in the UI.
    SceneItemSelected {
        /// Identifier of the scene the item is in.
        #[serde(flatten)]
        scene: SceneId,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: u64,
    },
    /// The transform/crop of a scene item has changed.
    SceneItemTransformChanged {
        /// Identifier of the scene the item is in.
        #[serde(flatten)]
        scene: SceneId,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: u64,
        /// New transform/crop info of the scene item.
        #[serde(rename = "sceneItemTransform")]
        transform: SceneItemTransform,
    },
    // --------------------------------
    // Scenes
    // --------------------------------
    /// A new scene has been created.
    SceneCreated {
        /// Identifier of the new scene.
        #[serde(flatten)]
        id: SceneId,
        /// Whether the new scene is a group.
        #[serde(rename = "isGroup")]
        is_group: bool,
    },
    /// A scene has been removed.
    SceneRemoved {
        /// Identifier of the removed scene.
        #[serde(flatten)]
        id: SceneId,
        /// Whether the scene was a group.
        #[serde(rename = "isGroup")]
        is_group: bool,
    },
    /// The name of a scene has changed.
    SceneNameChanged {
        /// UUID of the scene.
        #[serde(rename = "sceneUuid")]
        uuid: Uuid,
        /// Old name of the scene.
        #[serde(rename = "oldSceneName")]
        old_name: String,
        /// New name of the scene.
        #[serde(rename = "sceneName")]
        new_name: String,
    },
    /// The current program scene has changed.
    CurrentProgramSceneChanged {
        /// Identifier of the scene that was switched to.
        #[serde(flatten)]
        id: SceneId,
    },
    /// The current preview scene has changed.
    CurrentPreviewSceneChanged {
        /// Identifier of the scene that was switched to.
        #[serde(flatten)]
        id: SceneId,
    },
    /// The list of scenes has changed.
    SceneListChanged {
        /// Updated array of scenes.
        scenes: Vec<Scene>,
    },
    // --------------------------------
    // Transitions
    // --------------------------------
    /// The current scene transition has changed.
    CurrentSceneTransitionChanged {
        /// Identifier of the new transition.
        #[serde(flatten)]
        id: TransitionId,
    },
    /// The current scene transition duration has changed.
    CurrentSceneTransitionDurationChanged {
        /// Transition duration in milliseconds.
        #[serde(rename = "transitionDuration", with = "crate::serde::duration_millis")]
        duration: Duration,
    },
    /// A scene transition has started.
    SceneTransitionStarted {
        /// Scene transition identifier.
        #[serde(flatten)]
        id: TransitionId,
    },
    /// A scene transition has completed fully.
    ///
    /// **Note:** Does not appear to trigger when the transition is interrupted by the user.
    SceneTransitionEnded {
        /// Scene transition identifier.
        #[serde(flatten)]
        id: TransitionId,
    },
    /// A scene transition's video has completed fully.
    ///
    /// Useful for stinger transitions to tell when the video *actually* ends.
    /// [`Self::SceneTransitionEnded`] only signifies the cut point, not the completion of
    /// transition playback.
    ///
    /// **Note:** Appears to be called by every transition, regardless of relevance.
    SceneTransitionVideoEnded {
        /// Scene transition identifier.
        #[serde(flatten)]
        id: TransitionId,
    },
    // --------------------------------
    // UI
    // --------------------------------
    /// Studio mode has been enabled or disabled.
    StudioModeStateChanged {
        /// Whether the studio mode is enabled.
        #[serde(rename = "studioModeEnabled")]
        enabled: bool,
    },
    /// A screenshot has been saved.
    ///
    /// **Note**: Triggered for the screenshot feature available in `Settings -> Hotkeys ->
    /// Screenshot Output` ONLY.
    ScreenshotSaved {
        /// Path of the saved image file.
        #[serde(rename = "savedScreenshotPath")]
        path: String,
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

/// Volume meter information for a single input, describing the current volume level.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputVolumeMeter {
    /// Name of this input.
    #[serde(rename = "inputName")]
    pub name: String,
    /// List of volume levels, in **Mul**.
    #[serde(rename = "inputLevelsMul")]
    pub levels: Vec<[f32; 3]>,
}

/// The output state describes the current status of any output (like recording, virtual-cam, ...).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub enum OutputState {
    /// A request to start the output has been issued.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STARTING")]
    Starting,
    /// Output started successfully.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STARTED")]
    Started,
    /// A request to stop the output has been issued.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STOPPING")]
    Stopping,
    /// Output stopped successfully.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_STOPPED")]
    Stopped,
    /// Output disconnected and is reconnecting.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_RECONNECTING")]
    Reconnecting,
    /// Output reconnected successfully.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_RECONNECTED")]
    Reconnected,
    /// Current output paused.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_PAUSED")]
    Paused,
    /// Current output resumed.
    #[serde(rename = "OBS_WEBSOCKET_OUTPUT_RESUMED")]
    Resumed,
    /// Fallback for any unknown event type.
    #[serde(other)]
    Unknown,
}

/// A basic scene item, only describing identifier and position.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BasicSceneItem {
    /// Identifier of this scene item.
    #[serde(rename = "sceneItemId")]
    pub id: u64,
    /// Positional index within the owning scene.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
}

/// The scene describes basic details about a single scene setup in OBS.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Scene {
    /// Name of this scene.
    #[serde(rename = "sceneName")]
    pub name: String,
    /// Positional index in the scene list.
    #[serde(rename = "sceneIndex")]
    pub index: usize,
}
