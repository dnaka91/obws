//! All requests that can be send to the API.

use bitflags::bitflags;
use serde::{ser::SerializeStruct, Serialize};
use serde_repr::Serialize_repr;
use serde_with::skip_serializing_none;

pub mod config;
pub mod custom;
pub mod filters;
pub mod general;
pub mod hotkeys;
pub mod inputs;
pub(crate) mod media_inputs;
pub(crate) mod outputs;
pub mod profiles;
pub(crate) mod recording;
pub(crate) mod replay_buffer;
pub(crate) mod scene_collections;
pub mod scene_items;
pub mod scenes;
pub mod sources;
pub(crate) mod streaming;
pub(crate) mod transitions;
pub mod ui;
pub(crate) mod virtual_cam;

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

pub(crate) enum RequestType<'a> {
    Config(self::config::Request<'a>),
    Filters(self::filters::Request<'a>),
    General(self::general::Request<'a>),
    Hotkeys(self::hotkeys::Request<'a>),
    Inputs(self::inputs::Request<'a>),
    MediaInputs(self::media_inputs::Request<'a>),
    Outputs(self::outputs::Request<'a>),
    Profiles(self::profiles::Request<'a>),
    Recording(self::recording::Request),
    ReplayBuffer(self::replay_buffer::Request),
    SceneCollections(self::scene_collections::Request<'a>),
    SceneItems(self::scene_items::Request<'a>),
    Scenes(self::scenes::Request<'a>),
    Sources(self::sources::Request<'a>),
    Streaming(self::streaming::Request<'a>),
    Transitions(self::transitions::Request<'a>),
    Ui(self::ui::Request<'a>),
    VirtualCam(self::virtual_cam::Request),
}

impl<'a> Serialize for RequestType<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Config(req) => req.serialize(serializer),
            Self::Filters(req) => req.serialize(serializer),
            Self::General(req) => req.serialize(serializer),
            Self::Hotkeys(req) => req.serialize(serializer),
            Self::Inputs(req) => req.serialize(serializer),
            Self::MediaInputs(req) => req.serialize(serializer),
            Self::Outputs(req) => req.serialize(serializer),
            Self::Profiles(req) => req.serialize(serializer),
            Self::Recording(req) => req.serialize(serializer),
            Self::ReplayBuffer(req) => req.serialize(serializer),
            Self::SceneCollections(req) => req.serialize(serializer),
            Self::SceneItems(req) => req.serialize(serializer),
            Self::Scenes(req) => req.serialize(serializer),
            Self::Sources(req) => req.serialize(serializer),
            Self::Streaming(req) => req.serialize(serializer),
            Self::Transitions(req) => req.serialize(serializer),
            Self::Ui(req) => req.serialize(serializer),
            Self::VirtualCam(req) => req.serialize(serializer),
        }
    }
}
