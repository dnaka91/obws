use serde::{Serialize, de::DeserializeOwned};

use super::Client;
use crate::{
    error::Result,
    requests::config::{Realm, Request, SetPersistentData, SetVideoSettings},
    responses::config as responses,
};

/// API functions related to OBS configuration.
pub struct Config<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Config<'a> {
    /// Gets the value of a "slot" from the selected persistent data realm.
    #[doc(alias = "GetPersistentData")]
    pub async fn get_persistent_data(
        &self,
        realm: Realm,
        slot_name: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .send_message(Request::GetPersistentData { realm, slot_name })
            .await
    }

    /// Sets the value of a "slot" from the selected persistent data realm.
    #[doc(alias = "SetPersistentData")]
    pub async fn set_persistent_data(&self, data: SetPersistentData<'_>) -> Result<()> {
        self.client
            .send_message(Request::SetPersistentData(data))
            .await
    }

    /// Gets the current video settings.
    ///
    /// **Note:** To get the true FPS value, divide the FPS numerator by the FPS denominator.
    /// Example: `60000/1001`.
    #[doc(alias = "GetVideoSettings")]
    pub async fn video_settings(&self) -> Result<responses::VideoSettings> {
        self.client.send_message(Request::VideoSettings).await
    }

    /// Sets the current video settings.
    ///
    /// **Note:** Fields must be specified in pairs. For example, you cannot set only
    /// [`SetVideoSettings::base_width`] without needing to specify
    /// [`SetVideoSettings::base_height`].
    #[doc(alias = "SetVideoSettings")]
    pub async fn set_video_settings(&self, settings: SetVideoSettings) -> Result<()> {
        self.client
            .send_message(Request::SetVideoSettings(settings))
            .await
    }

    /// Gets the current stream service settings (stream destination).
    #[doc(alias = "GetStreamServiceSettings")]
    pub async fn stream_service_settings<T>(&self) -> Result<responses::StreamServiceSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message(Request::StreamServiceSettings)
            .await
    }

    /// Sets the current stream service settings (stream destination).
    ///
    /// **Note:** Simple RTMP settings can be set with type `rtmp_custom` and the settings fields
    /// `server` and `key`.
    #[doc(alias = "SetStreamServiceSettings")]
    pub async fn set_stream_service_settings<T>(&self, r#type: &'a str, settings: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetStreamServiceSettings {
                r#type,
                settings: serde_json::to_value(settings)
                    .map_err(crate::error::SerializeCustomDataError)?,
            })
            .await
    }

    /// Gets the current directory that the record output is set to.
    #[doc(alias = "GetRecordDirectory")]
    pub async fn record_directory(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::RecordDirectory>(Request::RecordDirectory)
            .await
            .map(|rd| rd.record_directory)
    }

    /// Sets the current directory that the record output writes files to.
    #[doc(alias = "SetRecordDirectory")]
    pub async fn set_record_directory(&self, directory: &'a str) -> Result<()> {
        self.client
            .send_message(Request::SetRecordDirectory { directory })
            .await
    }
}
