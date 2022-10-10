//! Requests related to inputs.

use serde::Serialize;
use serde_with::skip_serializing_none;
use time::Duration;

use crate::common::MonitorType;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetInputList")]
    List {
        /// Restrict the array to only inputs of the specified kind.
        #[serde(rename = "inputKind", skip_serializing_if = "Option::is_none")]
        kind: Option<&'a str>,
    },
    #[serde(rename = "GetInputKindList")]
    ListKinds {
        /// Return all kinds as unversioned or with version suffixes (if available).
        #[serde(rename = "unversioned")]
        unversioned: bool,
    },
    #[serde(rename = "GetSpecialInputs")]
    Specials,
    #[serde(rename = "GetInputDefaultSettings")]
    DefaultSettings {
        /// Input kind to get the default settings for.
        #[serde(rename = "inputKind")]
        kind: &'a str,
    },
    #[serde(rename = "GetInputSettings")]
    Settings {
        /// Name of the input to get the settings of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputSettings")]
    SetSettings(SetSettingsInternal<'a>),
    #[serde(rename = "GetInputMute")]
    Muted {
        /// Name of input to get the mute state of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputMute")]
    SetMuted {
        /// Name of the input to set the mute state of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Whether to mute the input.
        #[serde(rename = "inputMuted")]
        muted: bool,
    },
    #[serde(rename = "ToggleInputMute")]
    ToggleMute {
        /// Name of the input to toggle the mute state of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "GetInputVolume")]
    Volume {
        /// Name of the input to get the volume of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputVolume")]
    SetVolume {
        /// Name of the input to set the volume of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Volume settings in either mul or dB.
        #[serde(rename = "volume", flatten)]
        volume: Volume,
    },
    #[serde(rename = "SetInputName")]
    SetName {
        /// Current input name.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// New name for the input.
        #[serde(rename = "newInputName")]
        new: &'a str,
    },
    #[serde(rename = "CreateInput")]
    Create(CreateInputInternal<'a>),
    #[serde(rename = "RemoveInput")]
    Remove {
        /// Name of the input to remove.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "GetInputAudioBalance")]
    AudioBalance {
        /// Name of the input to get the audio balance of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputAudioBalance")]
    SetAudioBalance {
        /// Name of the input to set the audio balance of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// New audio balance value. Must be in range of `0.0..=1.0`.
        #[serde(rename = "inputAudioBalance")]
        balance: f32,
    },
    #[serde(rename = "GetInputAudioSyncOffset")]
    AudioSyncOffset {
        /// Name of the input to get the audio sync offset of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputAudioSyncOffset")]
    SetAudioSyncOffset {
        /// Name of the input to set the audio sync offset of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// New audio sync offset in milliseconds.
        #[serde(
            rename = "inputAudioSyncOffset",
            with = "crate::serde::duration_millis"
        )]
        offset: Duration,
    },
    #[serde(rename = "GetInputAudioMonitorType")]
    AudioMonitorType {
        /// Name of the input to get the audio monitor type of.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputAudioMonitorType")]
    SetAudioMonitorType {
        /// Name of the input to set the audio monitor type of.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Audio monitor type.
        #[serde(rename = "monitorType")]
        monitor_type: MonitorType,
    },
    #[serde(rename = "GetInputAudioTracks")]
    AudioTracks {
        /// Name of the input.
        #[serde(rename = "inputName")]
        name: &'a str,
    },
    #[serde(rename = "SetInputAudioTracks")]
    SetAudioTracks {
        /// Name of the input.
        #[serde(rename = "inputName")]
        name: &'a str,
        /// Track settings to apply.
        #[serde(
            rename = "inputAudioTracks",
            with = "crate::serde::audio_tracks::option"
        )]
        tracks: [Option<bool>; 6],
    },
    #[serde(rename = "GetInputPropertiesListPropertyItems")]
    PropertiesListPropertyItems {
        /// Name of the input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// Name of the list property to get the items of.
        #[serde(rename = "propertyName")]
        property: &'a str,
    },
    #[serde(rename = "PressInputPropertiesButton")]
    PressPropertiesButton {
        /// Name of the input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// Name of the button property to press.
        #[serde(rename = "propertyName")]
        property: &'a str,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Inputs(value)
    }
}

/// Request information for [`crate::client::Inputs::set_settings`].
pub struct SetSettings<'a, T> {
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
pub(crate) struct SetSettingsInternal<'a> {
    /// Name of the input to set the settings of.
    #[serde(rename = "inputName")]
    pub input: &'a str,
    /// Object of settings to apply.
    #[serde(rename = "inputSettings")]
    pub settings: serde_json::Value,
    /// Apply settings on top of existing ones or reset the input to its defaults, then apply
    /// settings.
    #[serde(rename = "overlay")]
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Inputs::set_volume`].
#[derive(Serialize)]
#[non_exhaustive]
pub enum Volume {
    /// Volume setting in mul.
    #[serde(rename = "inputVolumeMul")]
    Mul(f32),
    /// Volume setting in dB.
    #[serde(rename = "inputVolumeDb")]
    Db(f32),
}

/// Request information for [`crate::client::Inputs::create`].
pub struct Create<'a, T> {
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
