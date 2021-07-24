//! All requests that can be send to the API.

use std::path::Path;

use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(tag = "messageType")]
pub(crate) enum ClientRequest<'a> {
    #[serde(rename_all = "camelCase")]
    Identify {
        rpc_version: u32,
        authentication: Option<String>,
        ignore_invalid_messages: bool,
        ignore_non_fatal_request_checks: bool,
        event_subscriptions: Option<u32>,
    },
    #[serde(rename_all = "camelCase")]
    Reidentify {
        ignore_invalid_messages: bool,
        ignore_non_fatal_request_checks: bool,
        event_subscriptions: Option<u32>,
    },
    #[serde(rename_all = "camelCase")]
    Request {
        request_id: &'a str,
        #[serde(flatten)]
        ty: RequestType<'a>,
    },
    #[serde(rename_all = "camelCase")]
    RequestBatch {
        request_id: &'a str,
        halt_on_failure: Option<bool>,
        requests: &'a [RequestType<'a>],
    },
}

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum RequestType<'a> {
    // --------------------------------
    // Config
    // --------------------------------
    GetSceneCollectionList,
    #[serde(rename_all = "camelCase")]
    SetCurrentSceneCollection {
        scene_collection_name: &'a str,
    },
    GetProfileList,
    #[serde(rename_all = "camelCase")]
    SetCurrentProfile {
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
