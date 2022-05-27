//! All requests that can be send to the API.

use std::path::Path;

use bitflags::bitflags;
use serde::{ser::SerializeStruct, Serialize};
use serde_repr::Serialize_repr;
use serde_with::skip_serializing_none;
use time::Duration;

use crate::common::{Alignment, BoundsType, MediaAction, MonitorType};

pub mod custom;
mod ser;

pub(crate) enum ClientRequest<'a> {
    /// Response to [`crate::responses::ServerMessage::Hello`] message, should contain
    /// authentication string if authentication is required, along with Pub-sub subscriptions and
    /// other session parameters.
    Identify(Identify),
    /// Sent at any time after initial identification to update the provided session parameters.
    Reidentify(Reidentify),
    /// Client is making a request to obs-websocket. For example get current scene, create source.
    Request(Request<'a>),
    /// Client is making a batch of requests for obs-websocket. Requests are processed serially
    /// (in order) by the server.
    #[allow(dead_code)]
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

/// Response to [`crate::responses::ServerMessage::Hello`] message, should contain
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
    /// [`crate::responses::ServerMessage::RequestBatchResponse`].
    pub halt_on_failure: Option<bool>,
    pub requests: &'a [RequestType<'a>],
    pub execution_type: Option<ExecutionType>,
}

bitflags! {
    /// Bit flags for possible event subscriptions, that can be enabled when connecting to the OBS
    /// instance.
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
        /// Subscription value to receive events in the `Ui` category.
        const UI = 1 << 10;

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
            | Self::VENDORS.bits
            | Self::UI.bits;

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

#[allow(dead_code)]
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
    SetCurrentSceneCollection {
        /// Name of the scene collection to switch to.
        #[serde(rename = "sceneCollectionName")]
        name: &'a str,
    },
    CreateSceneCollection {
        /// Name for the new scene collection.
        #[serde(rename = "sceneCollectionName")]
        name: &'a str,
    },
    GetProfileList,
    SetCurrentProfile {
        /// Name of the profile to switch to.
        #[serde(rename = "profileName")]
        name: &'a str,
    },
    CreateProfile {
        /// Name for the new profile.
        #[serde(rename = "profileName")]
        name: &'a str,
    },
    RemoveProfile {
        /// Name of the profile to remove.
        #[serde(rename = "profileName")]
        name: &'a str,
    },
    GetProfileParameter {
        /// Category of the parameter to get.
        #[serde(rename = "parameterCategory")]
        category: &'a str,
        /// Name of the parameter to get.
        #[serde(rename = "parameterName")]
        name: &'a str,
    },
    SetProfileParameter(SetProfileParameter<'a>),
    GetVideoSettings,
    SetVideoSettings(SetVideoSettings),
    GetStreamServiceSettings,
    #[serde(rename_all = "camelCase")]
    SetStreamServiceSettings {
        /// Type of stream service to apply. Example: `rtmp_common` or `rtmp_custom`.
        #[serde(rename = "streamServiceType")]
        r#type: &'a str,
        /// Settings to apply to the service.
        #[serde(rename = "streamServiceSettings")]
        settings: serde_json::Value,
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
    RemoveSourceFilter {
        /// Name of the source the filter is on.
        #[serde(rename = "sourceName")]
        source: &'a str,
        /// Name of the filter to remove.
        #[serde(rename = "filterName")]
        filter: &'a str,
    },
    SetSourceFilterName(SetSourceFilterName<'a>),
    GetSourceFilter {
        /// Name of the source.
        #[serde(rename = "sourceName")]
        source: &'a str,
        /// Name of the filter.
        #[serde(rename = "filterName")]
        filter: &'a str,
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
        #[serde(rename = "hotkeyName")]
        name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyByKeySequence {
        /// The OBS key ID to use.
        #[serde(rename = "keyId")]
        id: &'a str,
        /// Object containing key modifiers to apply.
        #[serde(rename = "keyModifiers")]
        modifiers: KeyModifiers,
    },
    // TODO: Sleep
    // --------------------------------
    // Inputs
    // --------------------------------
    GetInputList {
        /// Restrict the array to only inputs of the specified kind.
        #[serde(rename = "inputKind", skip_serializing_if = "Option::is_none")]
        kind: Option<&'a str>,
    },
    GetInputKindList {
        /// Return all kinds as unversioned or with version suffixes (if available).
        unversioned: bool,
    },
    GetInputDefaultSettings {
        /// Input kind to get the default settings for.
        #[serde(rename = "inputKind")]
        kind: &'a str,
    },
    GetInputSettings {
        /// Name of the input to get the settings of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    SetInputSettings(SetInputSettingsInternal<'a>),
    GetInputMute {
        /// Name of input to get the mute state of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    SetInputMute {
        /// Name of the input to set the mute state of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Whether to mute the input.
        #[serde(rename = "inputMuted")]
        muted: bool,
    },
    ToggleInputMute {
        /// Name of the input to toggle the mute state of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    GetInputVolume {
        /// Name of the input to get the volume of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    SetInputVolume {
        /// Name of the input to set the volume of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Volume settings in either mul or dB.
        #[serde(flatten)]
        volume: Volume,
    },
    SetInputName {
        /// Current input name.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// New name for the input.
        #[serde(rename = "newInputName")]
        new: &'a str,
    },
    CreateInput(CreateInputInternal<'a>),
    RemoveInput {
        /// Name of the input to remove.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    GetInputAudioSyncOffset {
        /// Name of the input to get the audio sync offset of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    SetInputAudioSyncOffset {
        /// Name of the input to set the audio sync offset of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// New audio sync offset in milliseconds.
        #[serde(
            rename = "inputAudioSyncOffset",
            serialize_with = "ser::duration_millis"
        )]
        offset: Duration,
    },
    GetInputAudioMonitorType {
        /// Name of the input to get the audio monitor type of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    SetInputAudioMonitorType {
        /// Name of the input to set the audio monitor type of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Audio monitor type.
        #[serde(rename = "monitorType")]
        monitor_type: MonitorType,
    },
    GetInputPropertiesListPropertyItems {
        /// Name of the input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// Name of the list property to get the items of.
        #[serde(rename = "propertyName")]
        property: &'a str,
    },
    PressInputPropertiesButton {
        /// Name of the input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// Name of the button property to press.
        #[serde(rename = "propertyName")]
        property: &'a str,
    },
    // --------------------------------
    // Media inputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetMediaInputStatus {
        /// Name of the media input.
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetMediaInputCursor {
        /// Name of the media input.
        input_name: &'a str,
        /// New cursor position to set.
        #[serde(serialize_with = "ser::duration_millis")]
        media_cursor: Duration,
    },
    #[serde(rename_all = "camelCase")]
    OffsetMediaInputCursor {
        /// Name of the media input.
        input_name: &'a str,
        /// Value to offset the current cursor position by.
        #[serde(serialize_with = "ser::duration_millis")]
        media_cursor_offset: Duration,
    },
    #[serde(rename_all = "camelCase")]
    TriggerMediaInputAction {
        /// Name of the media input.
        input_name: &'a str,
        /// Identifier of the media action.
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
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetGroupSceneItemList {
        /// Name of the group to get the items of.
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    GetSceneItemId(GetSceneItemId<'a>),
    CreateSceneItem(CreateSceneItem<'a>),
    #[serde(rename_all = "camelCase")]
    RemoveSceneItem {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    DuplicateSceneItem(DuplicateSceneItem<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemTransform {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    SetSceneItemTransform(SetSceneItemTransform<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemEnabled {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    SetSceneItemEnabled(SetSceneItemEnabled<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemLocked {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    SetSceneItemLocked(SetSceneItemLocked<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemIndex {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename_all = "camelCase")]
    SetSceneItemIndex(SetSceneItemIndex<'a>),
    #[serde(rename_all = "camelCase")]
    GetSceneItemPrivateSettings {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename_all = "camelCase")]
    SetSceneItemPrivateSettings(SetSceneItemPrivateSettingsInternal<'a>),
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
        #[serde(rename = "sceneName")]
        name: &'a str,
        /// New name for the scene.
        #[serde(rename = "newSceneName")]
        new: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateScene {
        /// Name for the new scene.
        #[serde(rename = "sceneName")]
        name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    RemoveScene {
        /// Name of the scene to remove.
        #[serde(rename = "sceneName")]
        name: &'a str,
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
        #[serde(rename = "sourceName")]
        name: &'a str,
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
    SetCurrentSceneTransition {
        /// Name of the transition to make active.
        #[serde(rename = "transitionName")]
        name: &'a str,
    },
    SetCurrentSceneTransitionDuration {
        /// Duration in milliseconds.
        #[serde(rename = "transitionDuration", serialize_with = "ser::duration_millis")]
        duration: Duration,
    },
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneTransitionSettings {
        /// Settings object to apply to the transition.
        #[serde(rename = "transitionSettings")]
        settings: serde_json::Value,
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
    SetStudioModeEnabled {
        /// Enable or disable the studio mode.
        #[serde(rename = "studioModeEnabled")]
        enabled: bool,
    },
    OpenInputPropertiesDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    OpenInputFiltersDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    OpenInputInteractDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    GetMonitorList,
}

/// Request information for [`crate::client::Config::get_persistent_data`] and
/// [`crate::client::Config::set_persistent_data`] as part of
/// [`SetPersistentData`].
#[derive(Clone, Copy, Serialize)]
pub enum Realm {
    /// Data located in the global settings.
    #[serde(rename = "OBS_WEBSOCKET_DATA_REALM_GLOBAL")]
    Global,
    /// Data bound to the current profile.
    #[serde(rename = "OBS_WEBSOCKET_DATA_REALM_PROFILE")]
    Profile,
}

/// Request information for [`crate::client::Config::set_persistent_data`].
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

/// Request information for [`crate::client::Config::set_profile_parameter`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct SetProfileParameter<'a> {
    /// Category of the parameter to set.
    #[serde(rename = "parameterCategory")]
    pub category: &'a str,
    /// Name of the parameter to set.
    #[serde(rename = "parameterName")]
    pub name: &'a str,
    /// Value of the parameter to set. Use [`None`] to delete.
    #[serde(rename = "parameterValue")]
    pub value: Option<&'a str>,
}

/// Request information for [`crate::client::Config::set_video_settings`].
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

/// Request information for [`crate::client::Filters::create`].
pub struct CreateSourceFilter<'a, T> {
    /// Name of the source to add the filter to.
    pub source: &'a str,
    /// Name of the new filter to be created.
    pub filter: &'a str,
    /// The kind of filter to be created.
    pub kind: &'a str,
    /// Settings object to initialize the filter with.
    pub settings: Option<T>,
}

/// Request information for [`crate::client::Filters::create_source_filter`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub(crate) struct CreateSourceFilterInternal<'a> {
    /// Name of the source to add the filter to.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Name of the new filter to be created.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// The kind of filter to be created.
    #[serde(rename = "filterKind")]
    pub kind: &'a str,
    /// Settings object to initialize the filter with.
    #[serde(rename = "filterSettings")]
    pub settings: Option<serde_json::Value>,
}

/// Request information for [`crate::client::Filters::set_name`].
#[derive(Default, Serialize)]
pub struct SetSourceFilterName<'a> {
    /// Name of the source the filter is on.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Current name of the filter.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// New name for the filter.
    #[serde(rename = "newFilterName")]
    pub new_name: &'a str,
}

/// Request information for [`crate::client::Filters::set_index`].
#[derive(Default, Serialize)]
pub struct SetSourceFilterIndex<'a> {
    /// Name of the source the filter is on.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Name of the filter.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// New index position of the filter.
    #[serde(rename = "filterIndex")]
    pub index: u32,
}

/// Request information for [`crate::client::Filters::set_settings`].
pub struct SetSourceFilterSettings<'a, T> {
    /// Name of the source the filter is on.
    pub source: &'a str,
    /// Name of the filter to set the settings of.
    pub filter: &'a str,
    /// Object of settings to apply.
    pub settings: T,
    /// Whether to overlay over the current settings or replace them.
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Filters::set_settings`].
#[derive(Default, Serialize)]
pub(crate) struct SetSourceFilterSettingsInternal<'a> {
    /// Name of the source the filter is on.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Name of the filter to set the settings of.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// Object of settings to apply.
    #[serde(rename = "filterSettings")]
    pub settings: serde_json::Value,
    /// Whether to overlay over the current settings or replace them.
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Filters::set_enabled`].
#[derive(Default, Serialize)]
pub struct SetSourceFilterEnabled<'a> {
    /// Name of the source the filter is on.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Name of the filter.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// New enable state of the filter.
    #[serde(rename = "filterEnabled")]
    pub enabled: bool,
}

/// Request information for [`crate::client::General::call_vendor_request`].
pub struct CallVendorRequest<'a, T> {
    /// Name of the vendor to use.
    pub vendor_name: &'a str,
    /// The request type to call.
    pub request_type: &'a str,
    /// Object containing appropriate request data.
    pub request_data: &'a T,
}

/// Request information for [`crate::client::General::call_vendor_request`].
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

/// Request information for
/// [`crate::client::General::trigger_hotkey_by_key_sequence`].
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

/// Request information for [`crate::client::Inputs::set_settings`].
pub struct SetInputSettings<'a, T> {
    /// Name of the input to set the settings of.
    pub input: &'a str,
    /// Object of settings to apply.
    pub settings: &'a T,
    /// Apply settings on top of existing ones or reset the input to its defaults, then apply
    /// settings.
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Inputs::set_input_settings`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub(crate) struct SetInputSettingsInternal<'a> {
    /// Name of the input to set the settings of.
    #[serde(rename = "inputName")]
    pub input: &'a str,
    /// Object of settings to apply.
    #[serde(rename = "inputSettings")]
    pub settings: serde_json::Value,
    /// Apply settings on top of existing ones or reset the input to its defaults, then apply
    /// settings.
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Inputs::set_volume`].
#[derive(Serialize)]
pub enum Volume {
    /// Volume setting in mul.
    #[serde(rename = "inputVolumeMul")]
    Mul(f32),
    /// Volume setting in dB.
    #[serde(rename = "inputVolumeDb")]
    Db(f32),
}

/// Request information for [`crate::client::Inputs::create`].
pub struct CreateInput<'a, T> {
    /// Name of the scene to add the input to as a scene item.
    pub scene: &'a str,
    /// Name of the new input to created.
    pub input: &'a str,
    /// The kind of input to be created.
    pub kind: &'a str,
    /// Settings object to initialize the input with.
    pub settings: Option<T>,
    /// Whether to set the created scene item to enabled or disabled.
    pub enabled: Option<bool>,
}

/// Request information for [`crate::client::Inputs::create_input`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub(crate) struct CreateInputInternal<'a> {
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    #[serde(rename = "inputName")]
    pub input: &'a str,
    #[serde(rename = "inputKind")]
    pub kind: &'a str,
    #[serde(rename = "inputSettings")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: Option<bool>,
}

/// Request information for [`crate::client::SceneItems::id`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSceneItemId<'a> {
    /// Name of the scene or group to search in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Name of the source to find.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Number of matches to skip during search.
    ///
    /// `>= 0` means first forward. `-1` means last (top) item.
    pub search_offset: Option<i32>,
}

/// Request information for [`crate::client::SceneItems::create`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSceneItem<'a> {
    /// Name of the scene to create the new item in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Name of the source to add to the scene.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Enable state to apply to the scene item on creation.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: Option<bool>,
}

/// Request information for [`crate::client::SceneItems::duplicate`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateSceneItem<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Name of the scene to create the duplicated item in.
    #[serde(rename = "destinationSceneName")]
    pub destination: Option<&'a str>,
}

/// Request information for [`crate::client::SceneItems::set_transform`].
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemTransform<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Object containing scene item transform info to update.
    #[serde(rename = "sceneItemTransform")]
    pub transform: SceneItemTransform,
}

/// Request information for [`crate::client::SceneItems::set_transform`] as part of
/// [`SetSceneItemTransform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    #[serde(flatten)]
    pub position: Option<Position>,
    /// The clockwise rotation of the scene item in degrees around the point of alignment.
    pub rotation: Option<f32>,
    #[serde(flatten)]
    pub scale: Option<Scale>,
    /// The point on the source that the item is manipulated from.
    pub alignment: Option<Alignment>,
    #[serde(flatten)]
    pub bounds: Option<Bounds>,
    #[serde(flatten)]
    pub crop: Option<Crop>,
}

impl From<crate::responses::SceneItemTransform> for SceneItemTransform {
    fn from(t: crate::responses::SceneItemTransform) -> Self {
        Self {
            position: Some(Position {
                x: Some(t.position_x),
                y: Some(t.position_y),
            }),
            rotation: Some(t.rotation),
            scale: Some(Scale {
                x: Some(t.scale_x),
                y: Some(t.scale_y),
            }),
            alignment: Some(t.alignment),
            bounds: Some(Bounds {
                r#type: Some(t.bounds_type),
                alignment: Some(t.bounds_alignment),
                width: Some(t.bounds_width),
                height: Some(t.bounds_height),
            }),
            crop: Some(Crop {
                left: Some(t.crop_left),
                right: Some(t.crop_right),
                top: Some(t.crop_top),
                bottom: Some(t.crop_bottom),
            }),
        }
    }
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct Position {
    /// The x position of the source from the left.
    #[serde(rename = "positionX")]
    pub x: Option<f32>,
    /// The y position of the source from the top.
    #[serde(rename = "positionY")]
    pub y: Option<f32>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct Scale {
    /// The x-scale factor of the source.
    #[serde(rename = "scaleX")]
    pub x: Option<f32>,
    /// The y-scale factor of the source.
    #[serde(rename = "scaleY")]
    pub y: Option<f32>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct Bounds {
    /// Type of bounding box.
    #[serde(rename = "boundsType")]
    pub r#type: Option<BoundsType>,
    /// Alignment of the bounding box.
    #[serde(rename = "boundsAlignment")]
    pub alignment: Option<Alignment>,
    /// Width of the bounding box.
    #[serde(rename = "boundsWidth")]
    pub width: Option<f32>,
    /// Height of the bounding box.
    #[serde(rename = "boundsHeight")]
    pub height: Option<f32>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct Crop {
    /// The number of pixels cropped off the left of the source before scaling.
    #[serde(rename = "cropLeft")]
    pub left: Option<u32>,
    /// The number of pixels cropped off the right of the source before scaling.
    #[serde(rename = "cropRight")]
    pub right: Option<u32>,
    /// The number of pixels cropped off the top of the source before scaling.
    #[serde(rename = "cropTop")]
    pub top: Option<u32>,
    /// The number of pixels cropped off the bottom of the source before scaling.
    #[serde(rename = "cropBottom")]
    pub bottom: Option<u32>,
}

/// Request information for [`crate::client::SceneItems::set_enabled`].
#[derive(Default, Serialize)]
pub struct SetSceneItemEnabled<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New enable state of the scene item.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: bool,
}

/// Request information for [`crate::client::SceneItems::set_locked`].
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemLocked<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New lock state of the scene item.
    #[serde(rename = "sceneItemLocked")]
    pub locked: bool,
}

/// Request information for [`crate::client::SceneItems::set_index`].
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneItemIndex<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New index position of the scene item.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
}

/// Request information for [`crate::client::SceneItems::set_private_settings`].
pub struct SetSceneItemPrivateSettings<'a, T> {
    /// Name of the scene the item is in.
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    pub item_id: i64,
    /// Object of settings to apply.
    pub settings: &'a T,
}

/// Request information for
/// [`crate::client::SceneItems::set_scene_item_private_settings`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetSceneItemPrivateSettingsInternal<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Object of settings to apply.
    #[serde(rename = "sceneItemSettings")]
    pub settings: serde_json::Value,
}

/// Request information for [`crate::client::Scenes::set_transition_override`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneSceneTransitionOverride<'a> {
    /// Name of the scene.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Name of the scene transition to use as override.
    #[serde(rename = "transitionName")]
    pub transition: Option<&'a str>,
    /// Duration to use for any overridden transition.
    #[serde(
        rename = "transitionDuration",
        serialize_with = "ser::duration_millis_opt"
    )]
    pub duration: Option<Duration>,
}

/// Request information for [`crate::client::Sources::take_screenshot`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceScreenshot<'a> {
    /// Name of the source to take a screenshot of.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Image compression format to use. Use [`crate::client::General::version`] to get compatible
    /// image formats.
    #[serde(rename = "imageFormat")]
    pub format: &'a str,
    /// Width to scale the screenshot to.
    #[serde(rename = "imageWidth")]
    pub width: Option<u32>,
    /// Height to scale the screenshot to.
    #[serde(rename = "imageHeight")]
    pub height: Option<u32>,
    /// Compression quality to use. 0 for high compression, 100 for uncompressed. -1 to use
    /// "default".
    #[serde(rename = "imageCompressionQuality")]
    pub compression_quality: Option<i32>,
}

/// Request information for [`crate::client::Sources::save_screenshot`].
#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSourceScreenshot<'a> {
    /// Name of the source to take a screenshot of.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Image compression format to use. Use [`crate::client::General::version`] to get compatible
    /// image formats.
    #[serde(rename = "imageFormat")]
    pub format: &'a str,
    /// Width to scale the screenshot to.
    #[serde(rename = "imageWidth")]
    pub width: Option<u32>,
    /// Height to scale the screenshot to.
    #[serde(rename = "imageHeight")]
    pub height: Option<u32>,
    /// Compression quality to use. 0 for high compression, 100 for uncompressed. -1 to use
    /// "default".
    #[serde(rename = "imageCompressionQuality")]
    pub compression_quality: Option<i32>,
    /// Path to save the screenshot file to. For example `C:\Users\user\Desktop\screenshot.png`.
    #[serde(rename = "imageFilePath")]
    pub file_path: &'a Path,
}
