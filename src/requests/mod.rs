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
    SetInputSettings(SetInputSettings<'a>),
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
    CreateInput(CreateInput<'a>),
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
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetProfileParameter<'a> {
    pub parameter_category: &'a str,
    pub parameter_name: &'a str,
    pub parameter_value: Option<&'a str>,
}

#[derive(Default, Serialize)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub command: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInputSettings<'a> {
    pub input_name: &'a str,
    pub input_settings: serde_json::Value,
    pub overlay: bool,
}

#[derive(Serialize)]
pub enum Volume {
    #[serde(rename = "inputVolumeMul")]
    Mul(f64),
    #[serde(rename = "inputVolumeDb")]
    Db(f64),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInput<'a> {
    pub scene_name: &'a str,
    pub input_name: &'a str,
    pub input_kind: &'a str,
    pub input_settings: Option<serde_json::Value>,
    pub scene_item_enabled: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSourceScreenshot<'a> {
    pub source_name: &'a str,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_compression_quality: Option<i32>,
    pub image_format: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSourceScreenshot<'a> {
    pub source_name: &'a str,
    pub image_file_path: &'a Path,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_compression_quality: Option<i32>,
    pub image_format: &'a str,
}
