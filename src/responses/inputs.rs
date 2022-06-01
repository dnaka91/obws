//! Responses related to inputs.

use serde::Deserialize;
use time::Duration;

use crate::common::MonitorType;

/// Response value for [`crate::client::Inputs::get_input_list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Inputs {
    /// Array of inputs.
    pub inputs: Vec<Input>,
}

/// Response value for [`crate::client::Inputs::list`].
#[derive(Debug, Deserialize)]
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

/// Response value for [`crate::client::Inputs::get_input_kind_list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InputKinds {
    /// Array of input kinds.
    pub input_kinds: Vec<String>,
}

/// Response value for
/// [`crate::client::Inputs::get_input_default_settings`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DefaultInputSettings<T> {
    /// Object of default settings for the input kind.
    pub default_input_settings: T,
}

/// Response value for [`crate::client::Inputs::settings`].
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
pub struct InputVolume {
    /// Volume setting in mul.
    #[serde(rename = "inputVolumeMul")]
    pub mul: f32,
    /// Volume setting in dB.
    #[serde(rename = "inputVolumeDb")]
    pub db: f32,
}

/// Response value for [`crate::client::Inputs::get_audio_sync_offset`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioSyncOffset {
    /// Audio sync offset in milliseconds.
    #[serde(with = "crate::serde::duration_millis")]
    pub input_audio_sync_offset: Duration,
}

/// Response value for [`crate::client::Inputs::get_audio_monitor_type`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioMonitorType {
    /// Audio monitor type.
    pub monitor_type: MonitorType,
}

/// Response value for [`crate::client::Inputs::get_properties_list_property_items`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListPropertyItems {
    /// Array of items in the list property.
    pub property_items: Vec<ListPropertyItem>,
}

/// Response value for [`crate::client::Inputs::properties_list_property_items`].
#[derive(Debug, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemId {
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
}
