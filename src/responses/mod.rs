//! All responses that can be received from the API.

pub use semver::Version as SemVerVersion;
use serde::{de, Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use time::Duration;

use crate::{
    common::{Alignment, BoundsType},
    MonitorType,
};

#[derive(Debug)]
pub(crate) enum ServerMessage {
    /// First message sent from the server immediately on client connection. Contains authentication
    /// information if it is required. Also contains RPC (remote procedure call) version for
    /// version negotiation.
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
            Hello = 0,
            Identified = 2,
            Event = 5,
            RequestResponse = 7,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Hello {
    pub obs_web_socket_version: SemVerVersion,
    /// Version number which gets incremented on each **breaking change** to the obs-websocket
    /// protocol.
    pub rpc_version: u32,
    pub authentication: Option<Authentication>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Identified {
    /// The RPC (remote procedure call) version to be used.
    pub negotiated_rpc_version: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequestResponse {
    pub request_id: String,
    pub request_status: Status,
    #[serde(default)]
    pub response_data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequestBatchResponse {
    pub request_id: String,
    pub results: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Authentication {
    pub challenge: String,
    pub salt: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Status {
    pub result: bool,
    pub code: StatusCode,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
pub enum StatusCode {
    Unknown = 0,
    /// For internal use to signify a successful parameter check.
    NoError = 10,
    Success = 100,
    /// The `requestType` field is missing from the request data.
    MissingRequestType = 203,
    /// The request type is invalid or does not exist.
    UnknownRequestType = 204,
    /// Generic error code (comment required).
    GenericError = 205,
    /// The request batch execution type is not supported.
    UnsupportedRequestBatchExecutionType = 206,
    /// A required request parameter is missing.
    MissingRequestParameter = 300,
    /// The request does not have a valid `requestData` object.
    MissingRequestData = 301,
    /// Generic invalid request parameter message (comment required).
    InvalidRequestParameter = 400,
    /// A request parameter has the wrong data type.
    InvalidRequestParameterType = 401,
    /// A request parameter (float or int) is out of valid range.
    RequestParameterOutOfRange = 402,
    /// A request parameter (string or array) is empty and cannot be.
    RequestParameterEmpty = 403,
    /// There are too many request parameters (for example a request takes two optional values,
    /// where only one is allowed at a time).
    TooManyRequestParameters = 404,
    /// An output is running and cannot be in order to perform the request (generic).
    OutputRunning = 500,
    /// An output is not running and should be.
    OutputNotRunning = 501,
    /// An output is paused and should not be.
    OutputPaused = 502,
    /// An output is disabled and should not be.
    OutputDisabled = 503,
    /// Studio mode is active and cannot be.
    StudioModeActive = 504,
    /// Studio mode is not active and should be.
    StudioModeNotActive = 505,
    /// An output is not paused and should be
    OutputNotPaused = 506,
    /// The resource was not found.
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
    /// Creating the resource failed.
    ResourceCreationFailed = 700,
    /// Performing an action on the resource failed.
    ResourceActionFailed = 701,
    /// Processing the request failed unexpectedly (comment required).
    RequestProcessingFailed = 702,
    /// The combination of request parameters cannot be used to perform an action.
    CannotAct = 703,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneCollections {
    pub current_scene_collection_name: String,
    pub scene_collections: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profiles {
    pub current_profile_name: String,
    pub profiles: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileParameter {
    pub parameter_value: Option<String>,
    pub default_parameter_value: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoSettings {
    pub fps_numerator: u32,
    pub fps_denominator: u32,
    pub base_width: u32,
    pub base_height: u32,
    pub output_width: u32,
    pub output_height: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamServiceSettings<T> {
    pub stream_service_type: String,
    pub stream_service_settings: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub obs_version: SemVerVersion,
    pub obs_web_socket_version: SemVerVersion,
    pub rpc_version: u32,
    pub available_requests: Vec<String>,
    pub supported_image_formats: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub web_socket_session_incoming_messages: u64,
    pub web_socket_session_outgoing_messages: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Hotkeys {
    pub hotkeys: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StudioModeEnabled {
    pub studio_mode_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Inputs {
    pub inputs: Vec<Input>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub input_name: String,
    pub input_kind: String,
    pub unversioned_input_kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InputKinds {
    pub input_kinds: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DefaultInputSettings<T> {
    pub default_input_settings: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputSettings<T> {
    pub input_settings: T,
    pub input_kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InputMuted {
    pub input_muted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputVolume {
    pub input_volume_mul: f32,
    pub input_volume_db: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaStatus {
    pub media_state: MediaState,
    #[serde(deserialize_with = "crate::de::duration_millis_opt")]
    pub media_duration: Option<Duration>,
    #[serde(deserialize_with = "crate::de::duration_millis_opt")]
    pub media_cursor: Option<Duration>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum MediaState {
    #[serde(rename = "OBS_MEDIA_STATE_NONE")]
    None,
    #[serde(rename = "OBS_MEDIA_STATE_PLAYING")]
    Playing,
    #[serde(rename = "OBS_MEDIA_STATE_OPENING")]
    Opening,
    #[serde(rename = "OBS_MEDIA_STATE_BUFFERING")]
    Buffering,
    #[serde(rename = "OBS_MEDIA_STATE_PAUSED")]
    Paused,
    #[serde(rename = "OBS_MEDIA_STATE_STOPPED")]
    Stopped,
    #[serde(rename = "OBS_MEDIA_STATE_ENDED")]
    Ended,
    #[serde(rename = "OBS_MEDIA_STATE_ERROR")]
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordStatus {
    pub output_active: bool,
    pub output_paused: bool,
    #[serde(deserialize_with = "crate::de::duration_timecode")]
    pub output_timecode: Duration,
    #[serde(deserialize_with = "crate::de::duration_nanos")]
    pub output_duration: Duration,
    pub output_bytes: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputActive {
    pub output_active: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputPaused {
    pub output_paused: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecordDirectory {
    pub record_directory: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemId {
    pub scene_item_id: i64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    pub source_width: u32,
    pub source_height: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub width: u32,
    pub height: u32,
    pub alignment: Alignment,
    pub bounds_type: BoundsType,
    pub bounds_alignment: Alignment,
    pub bounds_width: u32,
    pub bounds_height: u32,
    pub crop_left: u32,
    pub crop_right: u32,
    pub crop_top: u32,
    pub crop_bottom: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemEnabled {
    pub scene_item_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemLocked {
    pub scene_item_locked: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemIndex {
    pub scene_item_index: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioSyncOffset {
    #[serde(deserialize_with = "crate::de::duration_nanos")]
    pub input_audio_sync_offset: Duration,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioMonitorType {
    pub monitor_type: MonitorType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListPropertyItems {
    pub property_items: Vec<ListPropertyItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPropertyItem {
    pub item_name: String,
    pub item_enabled: bool,
    pub item_value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemList {
    pub scene_items: Vec<SceneItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItem {
    scene_item_id: i64,
    scene_item_index: u32,
    source_name: String,
    source_type: SourceType,
    input_kind: Option<String>,
    is_group: Option<bool>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum SourceType {
    #[serde(rename = "OBS_SOURCE_TYPE_INPUT")]
    Input,
    #[serde(rename = "OBS_SOURCE_TYPE_FILTER")]
    Filter,
    #[serde(rename = "OBS_SOURCE_TYPE_TRANSITION")]
    Transition,
    #[serde(rename = "OBS_SOURCE_TYPE_SCENE")]
    Scene,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scenes {
    pub current_program_scene_name: Option<String>,
    pub current_preview_scene_name: Option<String>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    pub scene_name: String,
    pub is_group: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrentProgramScene {
    pub current_program_scene_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrentPreviewScene {
    pub current_preview_scene_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceActive {
    pub video_active: bool,
    pub video_showing: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImageData {
    pub image_data: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamStatus {
    pub output_active: bool,
    pub output_reconnecting: bool,
    #[serde(deserialize_with = "crate::de::duration_timecode")]
    pub output_timecode: Duration,
    #[serde(deserialize_with = "crate::de::duration_nanos")]
    pub output_duration: Duration,
    pub output_bytes: u64,
    pub output_skipped_frames: u32,
    pub output_total_frames: u32,
}
