//! All responses that can be received from the API.

pub mod config;
pub mod filters;
pub mod general;
pub(crate) mod hotkeys;
pub mod inputs;
pub mod media_inputs;
pub mod profiles;
pub mod recording;
pub(crate) mod replay_buffer;
pub mod scene_collections;
pub mod scene_items;
pub mod scenes;
pub mod sources;
pub mod streaming;
pub mod transitions;
pub mod ui;
pub(crate) mod virtual_cam;

use serde::{de, Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

#[derive(Debug)]
pub(crate) enum ServerMessage {
    /// First message sent from the server immediately on client connection. Contains authentication
    /// information if authentication is required. Also contains RPC version for version
    /// negotiation.
    Hello(Hello),
    /// The identify request was received and validated, and the connection is now ready for normal
    /// operation.
    Identified(Identified),
    /// An event coming from OBS has occurred. For example scene switched, source muted.
    #[cfg(feature = "events")]
    Event(crate::events::Event),
    #[cfg(not(feature = "events"))]
    Event,
    /// `obs-websocket` is responding to a request coming from a client.
    RequestResponse(RequestResponse),
    /// `obs-websocket` is responding to a request batch coming from the client.
    RequestBatchResponse(RequestBatchResponse),
}

impl<'de> Deserialize<'de> for ServerMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawServerMessage {
            #[serde(rename = "op")]
            op_code: OpCode,
            #[serde(rename = "d")]
            data: serde_json::Value,
        }

        #[derive(Deserialize_repr)]
        #[repr(u8)]
        enum OpCode {
            /// The initial message sent by obs-websocket to newly connected clients.
            Hello = 0,
            /// The response sent by obs-websocket to a client after it has successfully identified
            /// with obs-websocket.
            Identified = 2,
            /// The message sent by obs-websocket containing an event payload.
            Event = 5,
            /// The message sent by obs-websocket in response to a particular request from a client.
            RequestResponse = 7,
            /// The message sent by obs-websocket in response to a particular batch of requests from
            /// a client.
            RequestBatchResponse = 9,
        }

        let raw = RawServerMessage::deserialize(deserializer)?;

        Ok(match raw.op_code {
            OpCode::Hello => {
                ServerMessage::Hello(serde_json::from_value(raw.data).map_err(de::Error::custom)?)
            }
            OpCode::Identified => ServerMessage::Identified(
                serde_json::from_value(raw.data).map_err(de::Error::custom)?,
            ),
            OpCode::Event => {
                #[cfg(feature = "events")]
                {
                    ServerMessage::Event(
                        serde_json::from_value(raw.data).map_err(de::Error::custom)?,
                    )
                }
                #[cfg(not(feature = "events"))]
                {
                    ServerMessage::Event
                }
            }
            OpCode::RequestResponse => ServerMessage::RequestResponse(
                serde_json::from_value(raw.data).map_err(de::Error::custom)?,
            ),
            OpCode::RequestBatchResponse => ServerMessage::RequestBatchResponse(
                serde_json::from_value(raw.data).map_err(de::Error::custom)?,
            ),
        })
    }
}

/// First message sent from the server immediately on client connection. Contains authentication
/// information if authentication is required. Also contains RPC version for version negotiation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Hello {
    #[allow(dead_code)]
    pub obs_web_socket_version: semver::Version,
    /// version number which gets incremented on each **breaking change** to the obs-websocket
    /// protocol. Its usage in this context is to provide the current RPC version that the server
    /// would like to use.
    pub rpc_version: u32,
    pub authentication: Option<Authentication>,
}

/// The identify request was received and validated, and the connection is now ready for normal
/// operation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Identified {
    /// The RPC (remote procedure call) version to be used.
    pub negotiated_rpc_version: u32,
}

/// `obs-websocket` is responding to a request coming from a client.
#[derive(Debug, Deserialize)]
pub(crate) struct RequestResponse {
    #[allow(dead_code)]
    #[serde(rename = "requestType")]
    pub r#type: String,
    #[serde(rename = "requestId")]
    pub id: String,
    #[serde(rename = "requestStatus")]
    pub status: Status,
    #[serde(rename = "responseData", default)]
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RequestBatchResponse {
    #[allow(dead_code)]
    #[serde(rename = "requestId")]
    pub id: String,
    #[allow(dead_code)]
    pub results: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Authentication {
    pub challenge: String,
    pub salt: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Status {
    /// Is true if the request resulted in [`StatusCode::Success`]. False if otherwise.
    pub result: bool,
    pub code: StatusCode,
    /// May be provided by the server on errors to offer further details on why a request failed.
    pub comment: Option<String>,
}

/// The status code gives information about the result of a request. It gives further insight into
/// what went wrong, if a request failed.
#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
pub enum StatusCode {
    /// Unknown status, should never be used.
    Unknown = 0,

    /// For internal use to signify a successful field check.
    NoError = 10,

    /// The request has succeeded.
    Success = 100,

    /// The `requestType` field is missing from the request data.
    MissingRequestType = 203,
    /// The request type is invalid or does not exist.
    UnknownRequestType = 204,
    /// Generic error code.
    ///
    /// **Note:** A comment is required to be provided by obs-websocket.
    GenericError = 205,
    /// The request batch execution type is not supported.
    UnsupportedRequestBatchExecutionType = 206,

    /// A required request field is missing.
    MissingRequestField = 300,
    /// The request does not have a valid `requestData` object.
    MissingRequestData = 301,

    /// Generic invalid request field message.
    ///
    /// **Note:** A comment is required to be provided by obs-websocket.
    InvalidRequestField = 400,
    /// A request field has the wrong data type.
    InvalidRequestFieldType = 401,
    /// A request field (number) is outside the allowed range.
    RequestFieldOutOfRange = 402,
    /// A request field (string or array) is empty and cannot be.
    RequestFieldEmpty = 403,
    /// There are too many request fields (For example a request takes two optional fields, where
    /// only one is allowed at a time).
    TooManyRequestFields = 404,

    /// An output is running and cannot be in order to perform the request.
    OutputRunning = 500,
    /// An output is not running and should be.
    OutputNotRunning = 501,
    /// An output is paused and should not be.
    OutputPaused = 502,
    /// An output is not paused and should be.
    OutputNotPaused = 503,
    /// An output is disabled and should not be.
    OutputDisabled = 504,
    /// Studio mode is active and cannot be.
    StudioModeActive = 505,
    /// Studio mode is not active and should be.
    StudioModeNotActive = 506,

    /// The resource was not found.
    ///
    /// **Note:** Resources are any kind of object in obs-websocket, like inputs, profiles, outputs,
    /// etc.
    ResourceNotFound = 600,
    /// The resource already exists.
    ResourceAlreadyExists = 601,
    /// The type of resource found is invalid.
    InvalidResourceType = 602,
    /// There are not enough instances of the resource in order to perform the request.
    NotEnoughResources = 603,
    /// The state of the resource is invalid. For example, if the resource is blocked from being
    /// accessed.
    InvalidResourceState = 604,
    /// The specified input (obs_source_t-OBS_SOURCE_TYPE_INPUT) had the wrong kind.
    InvalidInputKind = 605,
    /// The resource does not support being configured.
    ///
    /// This is particularly relevant to transitions, where they do not always have changeable
    /// settings.
    ResourceNotConfigurable = 606,
    /// The specified filter had the wrong kind.
    InvalidFilterKind = 607,

    /// Creating the resource failed.
    ResourceCreationFailed = 700,
    /// Performing an action on the resource failed.
    ResourceActionFailed = 701,
    /// Processing the request failed unexpectedly.
    ///
    /// **Note:** A comment is required to be provided by obs-websocket.
    RequestProcessingFailed = 702,
    /// The combination of request fields cannot be used to perform an action.
    CannotAct = 703,
}
