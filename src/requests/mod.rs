//! All requests that can be send to the API.

use std::path::Path;

use bitflags::bitflags;
use serde::{ser::SerializeStruct, Serialize};
use serde_repr::Serialize_repr;
use serde_with::skip_serializing_none;
use time::Duration;

use crate::{
    common::{Alignment, BoundsType, MediaAction},
    MonitorType,
};

mod ser;

pub(crate) enum ClientRequest<'a> {
    /// Response to [`Hello`](crate::responses::ServerMessage::Hello) message, should contain
    /// authentication string if authentication is required, along with Pub-sub subscriptions and
    /// other session parameters.
    Identify(Identify),
    /// Sent at any time after initial identification to update the provided session parameters.
    Reidentify(Reidentify),
    /// Client is making a request to obs-websocket. For example get current scene, create source.
    Request(Request<'a>),
    /// Client is making a batch of requests for obs-websocket. Requests are processed serially
    /// (in order) by the server.
    RequestBatch(RequestBatch<'a>),
}

impl<'a> Serialize for ClientRequest<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize_repr)]
        #[repr(u8)]
        enum OpCode {
            /// The message sent by a newly connected client to obs-websocket in response to a
            /// `Hello`.
            Identify = 1,
            /// The message sent by an already-identified client to update identification
            /// parameters.
            Reidentify = 3,
            /// The message sent by a client to obs-websocket to perform a request.
            Request = 6,
            /// The message sent by a client to obs-websocket to perform a batch of requests.
            RequestBatch = 8,
        }

        fn write_state<S>(serializer: S, op: OpCode, d: &impl Serialize) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("ClientRequest", 2)?;
            state.serialize_field("op", &op)?;
            state.serialize_field("d", d)?;
            state.end()
        }

        match self {
            Self::Identify(value) => write_state(serializer, OpCode::Identify, value),
            Self::Reidentify(value) => write_state(serializer, OpCode::Reidentify, value),
            Self::Request(value) => write_state(serializer, OpCode::Request, value),
            Self::RequestBatch(value) => write_state(serializer, OpCode::RequestBatch, value),
        }
    }
}

/// Response to [`Hello`](crate::responses::ServerMessage::Hello) message, should contain
/// authentication string if authentication is required, along with Pub-sub subscriptions and other
/// session parameters.
#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Identify {
    /// Version number that the client would like the obs-websocket server to use.
    pub rpc_version: u32,
    pub authentication: Option<String>,
    /// Bit mask of event subscription items to subscribe to events and event categories at will. By
    /// default, all event categories are subscribed, except for events marked as high volume. High
    /// volume events must be explicitly subscribed to.
    pub event_subscriptions: Option<EventSubscription>,
}

/// Sent at any time after initial identification to update the provided session parameters.
#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Reidentify {
    pub event_subscriptions: Option<EventSubscription>,
}

/// Client is making a request to obs-websocket. For example get current scene, create source.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request<'a> {
    pub request_id: &'a str,
    #[serde(flatten)]
    pub ty: RequestType<'a>,
}

/// Client is making a batch of requests for obs-websocket. Requests are processed serially
/// (in order) by the server.
#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequestBatch<'a> {
    pub request_id: &'a str,
    /// When true, the processing of requests will be halted on first failure. Returns only the
    /// processed requests in
    /// [`RequestBatchResponse`](crate::responses::ServerMessage::RequestBatchResponse).
    pub halt_on_failure: Option<bool>,
    pub requests: &'a [RequestType<'a>],
    pub execution_type: Option<ExecutionType>,
}

bitflags! {
    #[derive(Serialize)]
    #[serde(transparent)]
    pub struct EventSubscription: u32 {
        /// Subscription value used to disable all events.
        const NONE = 0;
        /// Subscription value to receive events in the `General` category.
        const GENERAL = 1 << 0;
        /// Subscription value to receive events in the `Config` category.
        const CONFIG = 1 << 1;
        /// Subscription value to receive events in the `Scenes` category.
        const SCENES = 1 << 2;
        /// Subscription value to receive events in the `Inputs` category.
        const INPUTS = 1 << 3;
        /// Subscription value to receive events in the `Transitions` category.
        const TRANSITIONS = 1 << 4;
        /// Subscription value to receive events in the `Filters` category.
        const FILTERS = 1 << 5;
        /// Subscription value to receive events in the `Outputs` category.
        const OUTPUTS = 1 << 6;
        /// Subscription value to receive events in the `SceneItems` category.
        const SCENE_ITEMS = 1 << 7;
        /// Subscription value to receive events in the `MediaInputs` category.
        const MEDIA_INPUTS = 1 << 8;
        /// Subscription value to receive the [`VendorEvent`] event.
        ///
        /// [`VendorEvent`]: crate::events::Event::VendorEvent
        const VENDORS = 1 << 9;

        /// Helper to receive all non-high-volume events.
        const ALL = Self::GENERAL.bits
            | Self::CONFIG.bits
            | Self::SCENES.bits
            | Self::INPUTS.bits
            | Self::TRANSITIONS.bits
            | Self::FILTERS.bits
            | Self::OUTPUTS.bits
            | Self::SCENE_ITEMS.bits
            | Self::MEDIA_INPUTS.bits
            | Self::VENDORS.bits;

        /// Subscription value to receive the [`InputVolumeMeters`] high-volume event.
        ///
        /// [`InputVolumeMeters`]: crate::events::Event::InputVolumeMeters
        const INPUT_VOLUME_METERS = 1 << 16;
        /// Subscription value to receive the [`InputActiveStateChanged`] high-volume event.
        ///
        /// [`InputActiveStateChanged`]: crate::events::Event::InputActiveStateChanged
        const INPUT_ACTIVE_STATE_CHANGED = 1 << 17;
        /// Subscription value to receive the [`InputShowStateChanged`] high-volume event.
        ///
        /// [`InputShowStateChanged`]: crate::events::Event::InputShowStateChanged
        const INPUT_SHOW_STATE_CHANGED = 1 << 18;
        /// Subscription value to receive the [`SceneItemTransformChanged`] high-volume event.
        ///
        /// [`SceneItemTransformChanged`]: crate::events::Event::SceneItemTransformChanged
        const SCENE_ITEM_TRANSFORM_CHANGED = 1 << 19;

    }
}

#[derive(Serialize_repr)]
#[repr(i8)]
pub(crate) enum ExecutionType {
    /// Not a request batch.
    None = -1,
    /// A request batch which processes all requests serially, as fast as possible.
    SerialRealtime = 0,
    /// A request batch type which processes all requests serially, in sync with the graphics
    /// thread. Designed to provide high accuracy for animations.
    SerialFrame = 1,
    /// A request batch type which processes all requests using all available threads in the thread
    /// pool.
    Parallel = 2,
}

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum RequestType<'a> {
    // --------------------------------
    // Config
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetPersistentData {
        /// The data realm to select.
        realm: Realm,
        /// The name of the slot to retrieve data from.
        slot_name: &'a str,
    },
    SetPersistentData(SetPersistentData<'a>),
    GetSceneCollectionList,
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneCollection {
        /// Name of the scene collection to switch to.
        scene_collection_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateSceneCollection {
        /// Name for the new scene collection.
        scene_collection_name: &'a str,
    },
    GetProfileList,
    #[serde(rename_all = "camelCase")]
    SetCurrentProfile {
        /// Name of the profile to switch to.
        profile_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateProfile {
        /// Name for the new profile.
        profile_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    RemoveProfile {
        /// Name of the profile to remove.
        profile_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetProfileParameter {
        /// Category of the parameter to get.
        parameter_category: &'a str,
        /// Name of the parameter to get.
        parameter_name: &'a str,
    },
    SetProfileParameter(SetProfileParameter<'a>),
    GetVideoSettings,
    SetVideoSettings(SetVideoSettings),
    GetStreamServiceSettings,
    #[serde(rename_all = "camelCase")]
    SetStreamServiceSettings {
        /// Type of stream service to apply. Example: `rtmp_common` or `rtmp_custom`.
        stream_service_type: &'a str,
        /// Settings to apply to the service.
        stream_service_settings: serde_json::Value,
    },
    GetRecordDirectory,
    // --------------------------------
    // Filters
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetSourceFilterList {
        /// Name of the source.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceFilterDefaultSettings {
        /// Filter kind to get the default settings for.
        filter_kind: &'a str,
    },
    CreateSourceFilter(CreateSourceFilterInternal<'a>),
    #[serde(rename_all = "camelCase")]
    RemoveSourceFilter {
        /// Name of the source the filter is on.
        source_name: &'a str,
        /// Name of the filter to remove.
        filter_name: &'a str,
    },
    SetSourceFilterName(SetSourceFilterName<'a>),
    #[serde(rename_all = "camelCase")]
    GetSourceFilter {
        /// Name of the source.
        source_name: &'a str,
        /// Name of the filter.
        filter_name: &'a str,
    },
    SetSourceFilterIndex(SetSourceFilterIndex<'a>),
    SetSourceFilterSettings(SetSourceFilterSettingsInternal<'a>),
    SetSourceFilterEnabled(SetSourceFilterEnabled<'a>),
    // --------------------------------
    // General
    // --------------------------------
    GetVersion,
    #[serde(rename_all = "camelCase")]
    BroadcastCustomEvent {
        /// Data payload to emit to all receivers.
        event_data: serde_json::Value,
    },
    CallVendorRequest(CallVendorRequestInternal<'a>),
    GetStats,
    GetHotkeyList,
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyByName {
        /// Name of the hotkey to trigger.
        hotkey_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyByKeySequence {
        /// The OBS key ID to use.
        key_id: &'a str,
        /// Object containing key modifiers to apply.
        key_modifiers: KeyModifiers,
    },
    // TODO: Sleep
    // --------------------------------
    // Inputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetInputList {
        /// Restrict the array to only inputs of the specified kind.
        #[serde(skip_serializing_if = "Option::is_none")]
        input_kind: Option<&'a str>,
    },
    GetInputKindList {
        /// Return all kinds as unversioned or with version suffixes (if available).
        unversioned: bool,
    },
    #[serde(rename_all = "camelCase")]
    GetInputDefaultSettings {
        /// Input kind to get the default settings for.
        input_kind: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetInputSettings {
        /// Name of the input to get the settings of.
        input_name: &'a str,
    },
    SetInputSettings(SetInputSettingsInternal<'a>),
    #[serde(rename_all = "camelCase")]
    GetInputMute {
        /// Name of input to get the mute state of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetInputMute {
        /// Name of the input to set the mute state of.
        input_name: &'a str,
        /// Whether to mute the input.
        input_muted: bool,
    },
    #[serde(rename_all = "camelCase")]
    ToggleInputMute {
        /// Name of the input to toggle the mute state of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetInputVolume {
        /// Name of the input to get the volume of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetInputVolume {
        /// Name of the input to set the volume of.
        input_name: &'a str,
        /// Volume settings in either mul or dB.
        #[serde(flatten)]
        input_volume: Volume,
    },
    #[serde(rename_all = "camelCase")]
    SetInputName {
        /// Current input name.
        input_name: &'a str,
        /// New name for the input.
        new_input_name: &'a str,
    },
    CreateInput(CreateInputInternal<'a>),
    #[serde(rename_all = "camelCase")]
    RemoveInput {
        /// Name of the input to remove.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetInputAudioSyncOffset {
        /// Name of the input to get the audio sync offset of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetInputAudioSyncOffset {
        /// Name of the input to set the audio sync offset of.
        input_name: &'a str,
        /// New audio sync offset in milliseconds.
        #[serde(serialize_with = "ser::duration_millis")]
        input_audio_sync_offset: Duration,
    },
    #[serde(rename_all = "camelCase")]
    GetInputAudioMonitorType {
        /// Name of the input to get the audio monitor type of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetInputAudioMonitorType {
        /// Name of the input to set the audio monitor type of.
        input_name: &'a str,
        /// Audio monitor type.
        monitor_type: MonitorType,
    },
    #[serde(rename_all = "camelCase")]
    GetInputPropertiesListPropertyItems {
        /// Name of the input.
        input_name: &'a str,
        /// Name of the list property to get the items of.
        property_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    PressInputPropertiesButton {
        /// Name of the input.
        input_name: &'a str,
        /// Name of the button property to press.
        property_name: &'a str,
    },
    // --------------------------------
    // Media inputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetMediaInputStatus {
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetMediaInputCursor {
        input_name: &'a str,
        #[serde(serialize_with = "ser::duration_millis")]
        media_cursor: Duration,
    },
    #[serde(rename_all = "camelCase")]
    OffsetMediaInputCursor {
        input_name: &'a str,
        #[serde(serialize_with = "ser::duration_millis")]
        media_cursor_offset: Duration,
    },
    #[serde(rename_all = "camelCase")]
    TriggerMediaInputAction {
        input_name: &'a str,
        media_action: MediaAction,
    },
    // --------------------------------
    // Outputs
    // --------------------------------
    GetVirtualCamStatus,
    ToggleVirtualCam,
    StartVirtualCam,
    StopVirtualCam,
    GetReplayBufferStatus,
    ToggleReplayBuffer,
    StartReplayBuffer,
    StopReplayBuffer,
    SaveReplayBuffer,
    GetLastReplayBufferReplay,
    // --------------------------------
    // Recording
    // --------------------------------
    GetRecordStatus,
    ToggleRecord,
    StartRecord,
    StopRecord,
    ToggleRecordPause,
    PauseRecord,
    ResumeRecord,
    // --------------------------------
    // Scene items
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetSceneItemList {
        /// Name of the scene to get the items of.
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetGroupSceneItemList {
        /// Name of the group to get the items of.
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSceneItemId {
        /// Name of the scene or group to search in.
        scene_name: &'a str,
        /// Name of the source to find.
        source_name: &'a str,
    },
    CreateSceneItem(CreateSceneItem<'a>),
    #[serde(rename_all = "camelCase")]
    RemoveSceneItem {
        /// Name of the scene the item is in.
        scene_name: &'a str,
        /// Numeric ID of the scene item.
        scene_item_id: i64,
    },
    DuplicateSceneItem(DuplicateSceneItem<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemTransform {
        /// Name of the scene the item is in.
        scene_name: &'a str,
        /// Numeric ID of the scene item.
        scene_item_id: i64,
    },
    SetSceneItemTransform(SetSceneItemTransform<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemEnabled {
        /// Name of the scene the item is in.
        scene_name: &'a str,
        /// Numeric ID of the scene item.
        scene_item_id: i64,
    },
    SetSceneItemEnabled(SetSceneItemEnabled<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemLocked {
        /// Name of the scene the item is in.
        scene_name: &'a str,
        /// Numeric ID of the scene item.
        scene_item_id: i64,
    },
    SetSceneItemLocked(SetSceneItemLocked<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemIndex {
        /// Name of the scene the item is in.
        scene_name: &'a str,
        /// Numeric ID of the scene item.
        scene_item_id: i64,
    },
    #[serde(rename_all = "camelCase")]
    SetSceneItemIndex(SetSceneItemIndex<'a>),
    // --------------------------------
    // Scenes
    // --------------------------------
    GetSceneList,
    GetGroupList,
    GetCurrentProgramScene,
    #[serde(rename_all = "camelCase")]
    SetCurrentProgramScene {
        /// Scene to set as the current program scene.
        scene_name: &'a str,
    },
    GetCurrentPreviewScene,
    #[serde(rename_all = "camelCase")]
    SetCurrentPreviewScene {
        /// Scene to set as the current preview scene.
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetSceneName {
        /// Name of the scene to be renamed.
        scene_name: &'a str,
        /// New name for the scene.
        new_scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateScene {
        /// Name for the new scene.
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    RemoveScene {
        /// Name of the scene to remove.
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSceneSceneTransitionOverride {
        /// Name of the scene.
        scene_name: &'a str,
    },
    SetSceneSceneTransitionOverride(SetSceneSceneTransitionOverride<'a>),
    // --------------------------------
    // Sources
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetSourceActive {
        /// Name of the source to get the active state of.
        source_name: &'a str,
    },
    GetSourceScreenshot(GetSourceScreenshot<'a>),
    SaveSourceScreenshot(SaveSourceScreenshot<'a>),
    // --------------------------------
    // Streaming
    // --------------------------------
    GetStreamStatus,
    ToggleStream,
    StartStream,
    StopStream,
    #[serde(rename_all = "camelCase")]
    SendStreamCaption {
        /// Caption text.
        caption_text: &'a str,
    },
    // --------------------------------
    // Transitions
    // --------------------------------
    GetTransitionKindList,
    GetSceneTransitionList,
    GetCurrentSceneTransition,
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneTransition {
        /// Name of the transition to make active.
        transition_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneTransitionDuration {
        /// Duration in milliseconds.
        #[serde(serialize_with = "ser::duration_millis")]
        transition_duration: Duration,
    },
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneTransitionSettings {
        /// Settings object to apply to the transition.
        transition_settings: serde_json::Value,
        /// Whether to overlay over the current settings or replace them.
        #[serde(skip_serializing_if = "Option::is_none")]
        overlay: Option<bool>,
    },
    GetCurrentSceneTransitionCursor,
    TriggerStudioModeTransition,
    #[serde(rename_all = "camelCase", rename = "SetTBarPosition")]
    SetTbarPosition {
        /// New position.
        position: f32,
        /// Whether to release the T-Bar. Only set `false` if you know that you will be sending
        /// another position update.
        #[serde(skip_serializing_if = "Option::is_none")]
        release: Option<bool>,
    },
    // --------------------------------
    // UI
    // --------------------------------
    GetStudioModeEnabled,
    #[serde(rename_all = "camelCase")]
    SetStudioModeEnabled {
        /// Enable or disable the studio mode.
        studio_mode_enabled: bool,
    },
    #[serde(rename_all = "camelCase")]
    OpenInputPropertiesDialog {
        /// Name of the input to open the dialog of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    OpenInputFiltersDialog {
        /// Name of the input to open the dialog of.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    OpenInputInteractDialog {
        /// Name of the input to open the dialog of.
        input_name: &'a str,
    },
}

#[derive(Clone, Copy, Serialize)]
pub enum Realm {
    #[serde(rename = "OBS_WEBSOCKET_DATA_REALM_GLOBAL")]
    Global,
    #[serde(rename = "OBS_WEBSOCKET_DATA_REALM_PROFILE")]
    Profile,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPersistentData<'a> {
    /// The data realm to select.
    pub realm: Realm,
    /// The name of the slot to retrieve data from.
    pub slot_name: &'a str,
    /// The value to apply to the slot.
    pub slot_value: &'a serde_json::Value,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetProfileParameter<'a> {
    /// Category of the parameter to set.
    pub parameter_category: &'a str,
    /// Name of the parameter to set.
    pub parameter_name: &'a str,
    /// Value of the parameter to set. Use [`None`] to delete.
    pub parameter_value: Option<&'a str>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVideoSettings {
    /// Numerator of the fractional FPS value.
    pub fps_numerator: Option<u32>,
    /// Denominator of the fractional FPS value.
    pub fps_denominator: Option<u32>,
    /// Width of the base (canvas) resolution in pixels.
    pub base_width: Option<u32>,
    /// Height of the base (canvas) resolution in pixels.
    pub base_height: Option<u32>,
    /// Width of the output resolution in pixels.
    pub output_width: Option<u32>,
    /// Height of the output resolution in pixels.
    pub output_height: Option<u32>,
}

impl From<crate::responses::VideoSettings> for SetVideoSettings {
    fn from(v: crate::responses::VideoSettings) -> Self {
        Self {
            fps_numerator: Some(v.fps_numerator),
            fps_denominator: Some(v.fps_denominator),
            base_width: Some(v.base_width),
            base_height: Some(v.base_height),
            output_width: Some(v.output_width),
            output_height: Some(v.output_height),
        }
    }
}

pub struct CreateSourceFilter<'a, T> {
    /// Name of the source to add the filter to.
    pub source_name: &'a str,
    /// Name of the new filter to be created.
    pub filter_name: &'a str,
    /// The kind of filter to be created.
    pub filter_kind: &'a str,
    /// Settings object to initialize the filter with.
    pub filter_settings: Option<T>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateSourceFilterInternal<'a> {
    /// Name of the source to add the filter to.
    pub source_name: &'a str,
    /// Name of the new filter to be created.
    pub filter_name: &'a str,
    /// The kind of filter to be created.
    pub filter_kind: &'a str,
    /// Settings object to initialize the filter with.
    pub filter_settings: Option<serde_json::Value>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceFilterName<'a> {
    /// Name of the source the filter is on.
    pub source_name: &'a str,
    /// Current name of the filter.
    pub filter_name: &'a str,
    /// New name for the filter.
    pub new_filter_name: &'a str,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceFilterIndex<'a> {
    /// Name of the source the filter is on.
    pub source_name: &'a str,
    /// Name of the filter.
    pub filter_name: &'a str,
    /// New index position of the filter.
    pub filter_index: u32,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceFilterSettings<'a, T> {
    /// Name of the source the filter is on.
    pub source_name: &'a str,
    /// Name of the filter to set the settings of.
    pub filter_name: &'a str,
    /// Object of settings to apply.
    pub filter_settings: T,
    /// Whether to overlay over the current settings or replace them.
    pub overlay: Option<bool>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetSourceFilterSettingsInternal<'a> {
    /// Name of the source the filter is on.
    pub source_name: &'a str,
    /// Name of the filter to set the settings of.
    pub filter_name: &'a str,
    /// Object of settings to apply.
    pub filter_settings: serde_json::Value,
    /// Whether to overlay over the current settings or replace them.
    pub overlay: Option<bool>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSourceFilterEnabled<'a> {
    /// Name of the source the filter is on.
    pub source_name: &'a str,
    /// Name of the filter.
    pub filter_name: &'a str,
    /// New enable state of the filter.
    pub filter_enabled: bool,
}

pub struct CallVendorRequest<'a, T> {
    /// Name of the vendor to use.
    pub vendor_name: &'a str,
    /// The request type to call.
    pub request_type: &'a str,
    /// Object containing appropriate request data.
    pub request_data: &'a T,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CallVendorRequestInternal<'a> {
    /// Name of the vendor to use.
    pub vendor_name: &'a str,
    /// The request type to call.
    pub request_type: &'a str,
    /// Object containing appropriate request data.
    pub request_data: serde_json::Value,
}

#[derive(Default, Serialize)]
pub struct KeyModifiers {
    /// Press Shift.
    pub shift: bool,
    /// Press CTRL.
    pub control: bool,
    /// Press ALT.
    pub alt: bool,
    /// Press CMD (Mac).
    pub command: bool,
}

pub struct SetInputSettings<'a, T> {
    /// Name of the input to set the settings of.
    pub input_name: &'a str,
    /// Object of settings to apply.
    pub input_settings: &'a T,
    /// Apply settings on top of existing ones or reset the input to its defaults, then apply
    /// settings.
    pub overlay: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetInputSettingsInternal<'a> {
    /// Name of the input to set the settings of.
    pub input_name: &'a str,
    /// Object of settings to apply.
    pub input_settings: serde_json::Value,
    /// Apply settings on top of existing ones or reset the input to its defaults, then apply
    /// settings.
    pub overlay: Option<bool>,
}

#[derive(Serialize)]
pub enum Volume {
    /// Volume setting in mul.
    #[serde(rename = "inputVolumeMul")]
    Mul(f32),
    /// Volume setting in dB.
    #[serde(rename = "inputVolumeDb")]
    Db(f32),
}

pub struct CreateInput<'a, T> {
    /// Name of the scene to add the input to as a scene item.
    pub scene_name: &'a str,
    /// Name of the new input to created.
    pub input_name: &'a str,
    /// The kind of input to be created.
    pub input_kind: &'a str,
    /// Settings object to initialize the input with.
    pub input_settings: Option<T>,
    /// Whether to set the created scene item to enabled or disabled.
    pub scene_item_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateInputInternal<'a> {
    pub scene_name: &'a str,
    pub input_name: &'a str,
    pub input_kind: &'a str,
    pub input_settings: Option<serde_json::Value>,
    pub scene_item_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSceneItem<'a> {
    /// Name of the scene to create the new item in.
    pub scene_name: &'a str,
    /// Name of the source to add to the scene.
    pub source_name: &'a str,
    /// Enable state to apply to the scene item on creation.
    pub scene_item_enabled: Option<bool>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateSceneItem<'a> {
    /// Name of the scene the item is in.
    pub scene_name: &'a str,
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
    /// Name of the scene to create the duplicated item in.
    pub destination_scene_name: Option<&'a str>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemTransform<'a> {
    /// Name of the scene the item is in.
    pub scene_name: &'a str,
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
    /// Object containing scene item transform info to update.
    pub scene_item_transform: SceneItemTransform,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub rotation: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub alignment: Option<Alignment>,
    pub bounds_type: Option<BoundsType>,
    pub bounds_alignment: Option<Alignment>,
    pub bounds_width: Option<f32>,
    pub bounds_height: Option<f32>,
    pub crop_left: Option<u32>,
    pub crop_right: Option<u32>,
    pub crop_top: Option<u32>,
    pub crop_bottom: Option<u32>,
}

impl From<crate::responses::SceneItemTransform> for SceneItemTransform {
    fn from(t: crate::responses::SceneItemTransform) -> Self {
        Self {
            position_x: Some(t.position_x),
            position_y: Some(t.position_y),
            rotation: Some(t.rotation),
            scale_x: Some(t.scale_x),
            scale_y: Some(t.scale_y),
            alignment: Some(t.alignment),
            bounds_type: Some(t.bounds_type),
            bounds_alignment: Some(t.bounds_alignment),
            bounds_width: Some(t.bounds_width),
            bounds_height: Some(t.bounds_height),
            crop_left: Some(t.crop_left),
            crop_right: Some(t.crop_right),
            crop_top: Some(t.crop_top),
            crop_bottom: Some(t.crop_bottom),
        }
    }
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemEnabled<'a> {
    /// Name of the scene the item is in.
    pub scene_name: &'a str,
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
    /// New enable state of the scene item.
    pub scene_item_enabled: bool,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemLocked<'a> {
    /// Name of the scene the item is in.
    pub scene_name: &'a str,
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
    /// New lock state of the scene item.
    pub scene_item_locked: bool,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemIndex<'a> {
    /// Name of the scene the item is in.
    pub scene_name: &'a str,
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
    /// New index position of the scene item.
    pub scene_item_index: u32,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneSceneTransitionOverride<'a> {
    /// Name of the scene.
    pub scene_name: &'a str,
    /// Name of the scene transition to use as override.
    pub transition_name: Option<&'a str>,
    /// Duration to use for any overridden transition.
    #[serde(serialize_with = "ser::duration_millis_opt")]
    pub transition_duration: Option<Duration>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceScreenshot<'a> {
    /// Name of the source to take a screenshot of.
    pub source_name: &'a str,
    /// Image compression format to use. Use [`get_version`] to get compatible image formats.
    ///
    /// [`get_version`]: crate::client::General::get_version
    pub image_format: &'a str,
    /// Width to scale the screenshot to.
    pub image_width: Option<u32>,
    /// Height to scale the screenshot to.
    pub image_height: Option<u32>,
    /// Compression quality to use. 0 for high compression, 100 for uncompressed. -1 to use
    /// "default".
    pub image_compression_quality: Option<i32>,
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSourceScreenshot<'a> {
    /// Name of the source to take a screenshot of.
    pub source_name: &'a str,
    /// Image compression format to use. Use [`get_version`] to get compatible image formats.
    ///
    /// [`get_version`]: crate::client::General::get_version
    pub image_format: &'a str,
    /// Width to scale the screenshot to.
    pub image_width: Option<u32>,
    /// Height to scale the screenshot to.
    pub image_height: Option<u32>,
    /// Compression quality to use. 0 for high compression, 100 for uncompressed. -1 to use
    /// "default".
    pub image_compression_quality: Option<i32>,
    /// Path to save the screenshot file to. For example `C:\Users\user\Desktop\screenshot.png`.
    pub image_file_path: &'a Path,
}
