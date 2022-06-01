//! Requests related to the OBS configuration.

use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetPersistentData")]
    GetPersistentData {
        /// The data realm to select.
        #[serde(rename = "realm")]
        realm: Realm,
        /// The name of the slot to retrieve data from.
        #[serde(rename = "slotName")]
        slot_name: &'a str,
    },
    #[serde(rename = "SetPersistentData")]
    SetPersistentData(SetPersistentData<'a>),
    #[serde(rename = "GetVideoSettings")]
    VideoSettings,
    #[serde(rename = "SetVideoSettings")]
    SetVideoSettings(SetVideoSettings),
    #[serde(rename = "GetStreamServiceSettings")]
    StreamServiceSettings,
    #[serde(rename = "SetStreamServiceSettings")]
    SetStreamServiceSettings {
        /// Type of stream service to apply. Example: `rtmp_common` or `rtmp_custom`.
        #[serde(rename = "streamServiceType")]
        r#type: &'a str,
        /// Settings to apply to the service.
        #[serde(rename = "streamServiceSettings")]
        settings: serde_json::Value,
    },
    #[serde(rename = "GetRecordDirectory")]
    RecordDirectory,
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Config(value)
    }
}

/// Request information for [`crate::client::Config::get_persistent_data`] and
/// [`crate::client::Config::set_persistent_data`] as part of
/// [`SetPersistentData`].
#[derive(Clone, Copy, Serialize)]
pub enum Realm {
    /// Data located in the global settings.
    #[serde(rename = "OBS_WEBSOCKET_DATA_REALM_GLOBAL")]
    Global,
    /// Data bound to the current profile.
    #[serde(rename = "OBS_WEBSOCKET_DATA_REALM_PROFILE")]
    Profile,
}

/// Request information for [`crate::client::Config::set_persistent_data`].
#[derive(Serialize)]
pub struct SetPersistentData<'a> {
    /// The data realm to select.
    #[serde(rename = "realm")]
    pub realm: Realm,
    /// The name of the slot to retrieve data from.
    #[serde(rename = "slotName")]
    pub slot_name: &'a str,
    /// The value to apply to the slot.
    #[serde(rename = "slotValue")]
    pub slot_value: &'a serde_json::Value,
}

/// Request information for [`crate::client::Config::set_video_settings`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct SetVideoSettings {
    /// Numerator of the fractional FPS value.
    #[serde(rename = "fpsNumerator")]
    pub fps_numerator: Option<u32>,
    /// Denominator of the fractional FPS value.
    #[serde(rename = "fpsDenominator")]
    pub fps_denominator: Option<u32>,
    /// Width of the base (canvas) resolution in pixels.
    #[serde(rename = "baseWidth")]
    pub base_width: Option<u32>,
    /// Height of the base (canvas) resolution in pixels.
    #[serde(rename = "baseHeight")]
    pub base_height: Option<u32>,
    /// Width of the output resolution in pixels.
    #[serde(rename = "outputWidth")]
    pub output_width: Option<u32>,
    /// Height of the output resolution in pixels.
    #[serde(rename = "outputHeight")]
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
