//! All requests that can be send to the API.

use std::path::Path;

use serde::{ser::SerializeStruct, Serialize};
use serde_with::skip_serializing_none;

pub(crate) enum ClientRequest<'a> {
    Identify(Identify),
    Reidentify(Reidentify),
    Request(Request<'a>),
    RequestBatch(RequestBatch<'a>),
}

impl<'a> Serialize for ClientRequest<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        fn write_state<S>(serializer: S, op: u8, d: &impl Serialize) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut state = serializer.serialize_struct("ClientRequest", 2)?;
            state.serialize_field("op", &op)?;
            state.serialize_field("d", d)?;
            state.end()
        }

        match self {
            Self::Identify(value) => write_state(serializer, 1, value),
            Self::Reidentify(value) => write_state(serializer, 3, value),
            Self::Request(value) => write_state(serializer, 6, value),
            Self::RequestBatch(value) => write_state(serializer, 8, value),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Identify {
    pub rpc_version: u32,
    pub authentication: Option<String>,
    pub ignore_invalid_messages: bool,
    pub ignore_non_fatal_request_checks: bool,
    pub event_subscriptions: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Reidentify {
    pub ignore_invalid_messages: bool,
    pub ignore_non_fatal_request_checks: bool,
    pub event_subscriptions: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request<'a> {
    pub request_id: &'a str,
    #[serde(flatten)]
    pub ty: RequestType<'a>,
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RequestBatch<'a> {
    pub request_id: &'a str,
    pub halt_on_failure: Option<bool>,
    pub requests: &'a [RequestType<'a>],
}

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum RequestType<'a> {
    // --------------------------------
    // Config
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetPersistentData {
        realm: Realm,
        slot_name: &'a str,
    },
    SetPersistentData(SetPersistentData<'a>),
    GetSceneCollectionList,
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneCollection {
        scene_collection_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateSceneCollection {
        scene_collection_name: &'a str,
    },
    GetProfileList,
    #[serde(rename_all = "camelCase")]
    SetCurrentProfile {
        profile_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateProfile {
        profile_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    RemoveProfile {
        profile_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetProfileParameter {
        parameter_category: &'a str,
        parameter_name: &'a str,
    },
    SetProfileParameter(SetProfileParameter<'a>),
    GetVideoSettings,
    SetVideoSettings(SetVideoSettings),
    GetStreamServiceSettings,
    #[serde(rename_all = "camelCase")]
    SetStreamServiceSettings {
        stream_service_type: &'a str,
        stream_service_settings: serde_json::Value,
    },
    // --------------------------------
    // General
    // --------------------------------
    GetVersion,
    #[serde(rename_all = "camelCase")]
    BroadcastCustomEvent {
        event_data: serde_json::Value,
    },
    GetStats,
    GetHotkeyList,
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyByName {
        hotkey_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyByKeySequence {
        key_id: &'a str,
        key_modifiers: KeyModifiers,
    },
    GetStudioModeEnabled,
    #[serde(rename_all = "camelCase")]
    SetStudioModeEnabled {
        studio_mode_enabled: bool,
    },
    // TODO: Sleep
    // --------------------------------
    // Inputs
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetInputList {
        input_kind: Option<&'a str>,
    },
    GetInputKindList {
        unversioned: bool,
    },
    #[serde(rename_all = "camelCase")]
    GetInputDefaultSettings {
        input_kind: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetInputSettings {
        input_name: &'a str,
    },
    SetInputSettings(SetInputSettingsInternal<'a>),
    #[serde(rename_all = "camelCase")]
    GetInputMute {
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetInputMute {
        input_name: &'a str,
        input_muted: bool,
    },
    #[serde(rename_all = "camelCase")]
    ToggleInputMute {
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetInputVolume {
        input_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetInputVolume {
        input_name: &'a str,
        #[serde(flatten)]
        input_volume: Volume,
    },
    #[serde(rename_all = "camelCase")]
    SetInputName {
        input_name: &'a str,
        new_input_name: &'a str,
    },
    CreateInput(CreateInputInternal<'a>),
    // --------------------------------
    // Scenes
    // --------------------------------
    GetSceneList,
    GetCurrentProgramScene,
    #[serde(rename_all = "camelCase")]
    SetCurrentProgramScene {
        scene_name: &'a str,
    },
    GetCurrentPreviewScene,
    #[serde(rename_all = "camelCase")]
    SetCurrentPreviewScene {
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetSceneName {
        scene_name: &'a str,
        new_scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    CreateScene {
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    RemoveScene {
        scene_name: &'a str,
    },
    // --------------------------------
    // Sources
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetSourceActive {
        source_name: &'a str,
    },
    GetSourceScreenshot(GetSourceScreenshot<'a>),
    SaveSourceScreenshot(SaveSourceScreenshot<'a>),
    // --------------------------------
    // Streaming
    // --------------------------------
    GetStreamStatus,
    StartStream,
    StopStream,
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
    pub realm: Realm,
    pub slot_name: &'a str,
    pub slot_value: &'a serde_json::Value,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetProfileParameter<'a> {
    pub parameter_category: &'a str,
    pub parameter_name: &'a str,
    pub parameter_value: Option<&'a str>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVideoSettings {
    pub fps_numerator: Option<u32>,
    pub fps_denominator: Option<u32>,
    pub base_width: Option<u32>,
    pub base_height: Option<u32>,
    pub output_width: Option<u32>,
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

#[derive(Default, Serialize)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub command: bool,
}

pub struct SetInputSettings<'a, T> {
    pub input_name: &'a str,
    pub input_settings: &'a T,
    pub overlay: bool,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetInputSettingsInternal<'a> {
    pub input_name: &'a str,
    pub input_settings: serde_json::Value,
    pub overlay: bool,
}

#[derive(Serialize)]
pub enum Volume {
    #[serde(rename = "inputVolumeMul")]
    Mul(f32),
    #[serde(rename = "inputVolumeDb")]
    Db(f32),
}

pub struct CreateInput<'a, T> {
    pub scene_name: &'a str,
    pub input_name: &'a str,
    pub input_kind: &'a str,
    pub input_settings: Option<T>,
    pub scene_item_enabled: Option<bool>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateInputInternal<'a> {
    pub scene_name: &'a str,
    pub input_name: &'a str,
    pub input_kind: &'a str,
    pub input_settings: Option<serde_json::Value>,
    pub scene_item_enabled: Option<bool>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceScreenshot<'a> {
    pub source_name: &'a str,
    pub image_format: &'a str,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_compression_quality: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSourceScreenshot<'a> {
    pub source_name: &'a str,
    pub image_format: &'a str,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_compression_quality: Option<i32>,
    pub image_file_path: &'a Path,
}
