//! Responses related to inputs.

use serde::{Deserialize, Serialize};
use time::Duration;

use crate::common::MonitorType;

/// Response value for [`crate::client::Inputs::get_input_list`].
#[derive(Debug, Deserialize)]
pub(crate) struct Inputs {
    /// Array of inputs.
    #[serde(rename = "inputs")]
    pub inputs: Vec<Input>,
}

/// Response value for [`crate::client::Inputs::list`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Input {
    /// Name of the input source.
    #[serde(rename = "inputName")]
    pub name: String,
    /// Version input kind.
    #[serde(rename = "inputKind")]
    pub kind: String,
    /// Kind of input, without the version part.
    #[serde(rename = "unversionedInputKind")]
    pub unversioned_kind: String,
}

/// Response value for [`crate::client::Inputs::list_kinds`].
#[derive(Debug, Deserialize)]
pub(crate) struct InputKinds {
    /// Array of input kinds.
    #[serde(rename = "inputKinds")]
    pub input_kinds: Vec<String>,
}

/// Response value for [`crate::client::Inputs::specials`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SpecialInputs {
    /// Name of the Desktop Audio input.
    #[serde(rename = "desktop1")]
    pub desktop1: Option<String>,
    /// Name of the Desktop Audio 2 input.
    #[serde(rename = "desktop2")]
    pub desktop2: Option<String>,
    /// Name of the Mic/Auxiliary Audio input.
    #[serde(rename = "mic1")]
    pub mic1: Option<String>,
    /// Name of the Mic/Auxiliary Audio 2 input.
    #[serde(rename = "mic2")]
    pub mic2: Option<String>,
    /// Name of the Mic/Auxiliary Audio 3 input.
    #[serde(rename = "mic3")]
    pub mic3: Option<String>,
    /// Name of the Mic/Auxiliary Audio 4 input.
    #[serde(rename = "mic4")]
    pub mic4: Option<String>,
}

/// Response value for
/// [`crate::client::Inputs::get_input_default_settings`].
#[derive(Debug, Deserialize)]
pub(crate) struct DefaultInputSettings<T> {
    /// Object of default settings for the input kind.
    #[serde(rename = "defaultInputSettings")]
    pub default_input_settings: T,
}

/// Response value for [`crate::client::Inputs::settings`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InputSettings<T> {
    /// Object of settings for the input.
    #[serde(rename = "inputSettings")]
    pub settings: T,
    /// The kind of the input.
    #[serde(rename = "inputKind")]
    pub kind: String,
}

/// Response value for [`crate::client::Inputs::get_input_mute`] and
/// [`crate::client::Inputs::toggle_input_mute`].
#[derive(Debug, Deserialize)]
pub(crate) struct InputMuted {
    /// Whether the input is muted.
    #[serde(rename = "inputMuted")]
    pub muted: bool,
}

/// Response value for [`crate::client::Inputs::volume`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputVolume {
    /// Volume setting in mul.
    #[serde(rename = "inputVolumeMul")]
    pub mul: f32,
    /// Volume setting in dB.
    #[serde(rename = "inputVolumeDb")]
    pub db: f32,
}

/// Response value for [`crate::client::Inputs::audio_balance`].
#[derive(Debug, Deserialize)]
pub(crate) struct AudioBalance {
    #[serde(rename = "inputAudioBalance")]
    pub audio_balance: f32,
}

/// Response value for [`crate::client::Inputs::audio_sync_offset`].
#[derive(Debug, Deserialize)]
pub(crate) struct AudioSyncOffset {
    /// Audio sync offset in milliseconds.
    #[serde(
        rename = "inputAudioSyncOffset",
        with = "crate::serde::duration_millis"
    )]
    pub input_audio_sync_offset: Duration,
}

/// Response value for [`crate::client::Inputs::audio_monitor_type`].
#[derive(Debug, Deserialize)]
pub(crate) struct AudioMonitorType {
    /// Audio monitor type.
    #[serde(rename = "monitorType")]
    pub monitor_type: MonitorType,
}

/// Response value for [`crate::client::Inputs::audio_tracks`].
#[derive(Debug, Deserialize)]
pub(crate) struct AudioTracks {
    /// Object of audio tracks and associated enable states.
    #[serde(rename = "inputAudioTracks", with = "crate::serde::audio_tracks")]
    pub audio_tracks: [bool; 6],
}

/// Response value for [`crate::client::Inputs::get_properties_list_property_items`].
#[derive(Debug, Deserialize)]
pub(crate) struct ListPropertyItems {
    /// Array of items in the list property.
    #[serde(rename = "propertyItems")]
    pub property_items: Vec<ListPropertyItem>,
}

/// Response value for [`crate::client::Inputs::properties_list_property_items`].
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListPropertyItem {
    /// Name of the item.
    #[serde(rename = "itemName")]
    pub name: String,
    /// Whether this item is enabled in the UI.
    #[serde(rename = "itemEnabled")]
    pub enabled: bool,
    /// Content of the item, depending on what it represents.
    #[serde(rename = "itemValue")]
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemId {
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub scene_item_id: i64,
}
