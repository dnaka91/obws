//! All responses that can be received from the API.

use chrono::Duration;
pub use semver::Version as SemVerVersion;
use serde::{de, Deserialize, Deserializer};
use serde_repr::Deserialize_repr;

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
    /// The request type is invalid (does not exist).
    UnknownRequestType = 204,
    /// Generic error code (comment is expected to be provided).
    GenericError = 205,
    /// A required request parameter is missing.
    MissingRequestParameter = 300,
    /// The request does not have a valid `requestData` object.
    MissingRequestData = 301,
    /// Generic invalid request parameter message.
    InvalidRequestParameter = 400,
    /// A request parameter has the wrong data type.
    InvalidRequestParameterDataType = 401,
    /// A request parameter (float or int) is out of valid range.
    RequestParameterOutOfRange = 402,
    /// A request parameter (string or array) is empty and cannot be.
    RequestParameterEmpty = 403,
    /// An output is running and cannot be in order to perform the request (generic).
    OutputRunning = 500,
    /// An output is not running and should be.
    OutputNotRunning = 501,
    /// Stream is running and cannot be.
    StreamRunning = 502,
    /// Stream is not running and should be.
    StreamNotRunning = 503,
    /// Record is running and cannot be.
    RecordRunning = 504,
    /// Record is not running and should be.
    RecordNotRunning = 505,
    /// Record is paused and cannot be.
    RecordPaused = 506,
    /// Replay buffer is running and cannot be.
    ReplayBufferRunning = 507,
    /// Replay buffer is not running and should be.
    ReplayBufferNotRunning = 508,
    /// Replay buffer is disabled and cannot be.
    ReplayBufferDisabled = 509,
    /// Studio mode is active and cannot be.
    StudioModeActive = 510,
    /// Studio mode is not active and should be.
    StudioModeNotActive = 511,
    /// Virtual-cam is running and cannot be.
    VirtualCamRunning = 512,
    /// Virtual-cam is not running and should be.
    VirtualCamNotRunning = 513,
    /// The specified source (`obs_source_t`) was of the invalid type (for example input instead of
    /// scene).
    InvalidSourceType = 600,
    /// The specified source (`obs_source_t`) was not found (generic for input, filter, transition,
    /// scene).
    SourceNotFound = 601,
    /// The specified source (`obs_source_t`) already exists. Applicable to inputs, filters,
    /// transitions, scenes.
    SourceAlreadyExists = 602,
    /// The specified input (`obs_source_t-OBS_SOURCE_TYPE_FILTER`) was not found.
    InputNotFound = 603,
    /// The specified input (`obs_source_t-OBS_SOURCE_TYPE_INPUT`) had the wrong kind.
    InvalidInputKind = 604,
    /// The specified filter (`obs_source_t-OBS_SOURCE_TYPE_FILTER`) was not found.
    FilterNotFound = 605,
    /// The specified transition (`obs_source_t-OBS_SOURCE_TYPE_TRANSITION`) was not found.
    TransitionNotFound = 606,
    /// The specified transition (`obs_source_t-OBS_SOURCE_TYPE_TRANSITION`) does not support
    /// setting its position (transition is of fixed type).
    TransitionDurationFixed = 607,
    /// The specified scene (`obs_source_t-OBS_SOURCE_TYPE_SCENE`), (`obs_scene_t`) was not found.
    SceneNotFound = 608,
    /// The specified scene item (`obs_sceneitem_t`) was not found.
    SceneItemNotFound = 609,
    /// The specified scene collection was not found.
    SceneCollectionNotFound = 610,
    /// The specified profile was not found.
    ProfileNotFound = 611,
    /// The specified output (`obs_output_t`) was not found.
    OutputNotFound = 612,
    /// The specified encoder (`obs_encoder_t`) was not found.
    EncoderNotFound = 613,
    /// The specified service (`obs_service_t`) was not found.
    ServiceNotFound = 614,
    /// The specified hotkey was not found.
    HotkeyNotFound = 615,
    /// The specified directory was not found.
    DirectoryNotFound = 616,
    /// The specified configuration item (`config_t`) was not found. Could be section or parameter
    /// name.
    ConfigParameterNotFound = 617,
    /// The specified property (`obs_properties_t`) was not found.
    PropertyNotFound = 618,
    /// The specified key (`OBS_KEY_*`) was not found.
    KeyNotFound = 619,
    /// The specified data realm (`OBS_WEBSOCKET_DATA_REALM_*`) was not found.
    DataRealmNotFound = 620,
    /// The scene collection already exists.
    SceneCollectionAlreadyExists = 621,
    /// There are not enough scene collections to perform the action.
    NotEnoughSceneCollections = 622,
    /// The profile already exists.
    ProfileAlreadyExists = 623,
    /// There are not enough profiles to perform the action.
    NotEnoughProfiles = 624,
    /// There are not enough scenes to perform the action.
    NotEnoughScenes = 625,
    /// Processing the request failed unexpectedly.
    RequestProcessingFailed = 700,
    /// Starting the Output failed.
    OutputStartFailed = 701,
    /// Duplicating the scene item failed.
    SceneItemDuplicationFailed = 702,
    /// Rendering the screenshot failed.
    ScreenshotRenderFailed = 703,
    /// Encoding the screenshot failed.
    ScreenshotEncodeFailed = 704,
    /// Saving the screenshot failed.
    ScreenshotSaveFailed = 705,
    /// Creating the directory failed.
    DirectoryCreationFailed = 706,
    /// The combination of request parameters cannot be used to perform an action.
    CannotAct = 707,
    /// Creation of a new stream service failed.
    StreamServiceCreationFailed = 708,
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
pub(crate) struct SceneItemId {
    pub scene_item_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scenes {
    pub current_program_scene_name: String,
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
