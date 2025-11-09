//! All events that can be received from the API.

use std::{
    collections::BTreeMap,
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll},
};

use bitflags::bitflags;
use futures_util::{Stream, StreamExt, stream::Fuse};
use serde::{Deserialize, Serialize};
use time::Duration;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
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
        /// Bitflag value for the capabilities that an input supports.
        #[serde(rename = "inputKindCaps", default)]
        caps: OutputFlags,
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
    /// The record output has started writing to a new file. For example, when a file split
    /// happens.
    RecordFileChanged {
        /// File name that the output has begun writing to.
        #[serde(rename = "newOutputPath")]
        path: String,
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

/// These flags determine what type of data the source outputs and expects.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(from = "u32", into = "u32")]
pub struct OutputFlags(u32);

bitflags! {
    impl OutputFlags: u32 {
        /// Source has video.
        ///
        /// Unless SOURCE_ASYNC_VIDEO is specified, the source must include the video_render
        /// callback in the source definition structure.
        const VIDEO = 1 << 0;
        /// Source has audio.
        ///
        /// Use the obs_source_output_audio function to pass raw audio data, which will be
        /// automatically converted and uploaded.  If used with SOURCE_ASYNC_VIDEO, audio will
        /// automatically be synced up to the video output.
        const AUDIO = 1 << 1;
        /// Async video flag (use [`Self::ASYNC_VIDEO`]).
        const ASYNC = 1 << 2;
        /// Source passes raw video data via RAM.
        ///
        /// Use the obs_source_output_video function to pass raw video data, which will be
        /// automatically uploaded at the specified timestamp.
        ///
        /// If this flag is specified, it is not necessary to include the video_render callback.
        /// However, if you wish to use that function as well, you must call obs_source_getframe to
        /// get the current frame data, and obs_source_releaseframe to release the data when
        /// complete.
        const ASYNC_VIDEO = Self::ASYNC.bits() | Self::VIDEO.bits();
        /// Source uses custom drawing, rather than a default effect.
        ///
        /// If this flag is specified, the video_render callback will pass a NULL effect, and
        /// effect-based filters will not use direct rendering.
        const CUSTOM_DRAW = 1 << 3;
        /// Source supports interaction.
        ///
        /// When this is used, the source will receive interaction events if they provide the
        /// necessary callbacks in the source definition structure.
        const INTERACTION = 1 << 5;
        /// Source composites sub-sources
        ///
        /// When used specifies that the source composites one or more sub-sources. Sources that
        /// render sub-sources must implement the audio_render callback in order to perform custom
        /// mixing of sub-sources.
        ///
        /// This capability flag is always set for transitions.
        const COMPOSITE = 1 << 6;
        /// Source should not be fully duplicated
        ///
        /// When this is used, specifies that the source should not be fully duplicated, and should
        /// prefer to duplicate via holding references rather than full duplication.
        const DO_NOT_DUPLICATE = 1 << 7;
        /// Source is deprecated and should not be used.
        const DEPRECATED = 1 << 8;
        /// Source cannot have its audio monitored
        ///
        /// Specifies that this source may cause a feedback loop if audio is monitored with a device
        /// selected as desktop audio.
        ///
        /// This is used primarily with desktop audio capture sources.
        const DO_NOT_SELF_MONITOR = 1 << 9;
        /// Source type is currently disabled and should not be shown to the user.
        const CAP_DISABLED = 1 << 10;
        /// Source type is obsolete (has been updated with new defaults/properties/etc).
        const CAP_OBSOLETE = Self::CAP_DISABLED.bits();
        /// Source should enable monitoring by default.  Monitoring should be set by the frontend if
        /// this flag is set.
        const MONITOR_BY_DEFAULT = 1 << 11;
        /// Used internally for audio submixing.
        const SUBMIX = 1 << 12;
        /// Source type can be controlled by media controls.
        const CONTROLLABLE_MEDIA = 1 << 13;
        /// Source type provides cea708 data.
        const CEA_708 = 1 << 14;
        /// Source understands SRGB rendering.
        const SRGB = 1 << 15;
        /// Source type prefers not to have its properties shown on creation (prefers to rely on
        /// defaults first).
        const CAP_DONT_SHOW_PROPERTIES = 1 << 16;
        /// Source requires a canvas to operate.
        const REQUIRES_CANVAS = 1 << 17;
    }
}

impl From<OutputFlags> for u32 {
    fn from(value: OutputFlags) -> Self {
        value.bits()
    }
}

impl From<u32> for OutputFlags {
    fn from(value: u32) -> Self {
        Self::from_bits_truncate(value)
    }
}

/// Event stream returned by [`Client::events`](crate::Client::events).
pub struct EventStream {
    inner: Fuse<BroadcastStream<Event>>,
}

impl EventStream {
    pub(crate) fn new(receiver: broadcast::Receiver<Event>) -> Self {
        Self {
            inner: BroadcastStream::new(receiver).fuse(),
        }
    }
}

impl Stream for EventStream {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut()
            .inner
            .poll_next_unpin(cx)
            .map(|v| v.and_then(Result::ok))
    }
}
