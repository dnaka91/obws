use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{
    requests::{Realm, RequestType, SetPersistentData, SetProfileParameter, SetVideoSettings},
    responses, Error, Result,
};

/// API functions related to OBS configuration.
pub struct Config<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Config<'a> {
    /// Gets the value of a "slot" from the selected persistent data realm.
    pub async fn get_persistent_data(
        &self,
        realm: Realm,
        slot_name: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .send_message(RequestType::GetPersistentData { realm, slot_name })
            .await
    }

    /// Sets the value of a "slot" from the selected persistent data realm.
    pub async fn set_persistent_data(&self, data: SetPersistentData<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetPersistentData(data))
            .await
    }

    /// Gets an array of all scene collections.
    pub async fn list_scene_collections(&self) -> Result<responses::SceneCollections> {
        self.client
            .send_message(RequestType::GetSceneCollectionList)
            .await
    }

    /// Switches to a scene collection.
    ///
    /// **Note:** This will block until the collection has finished changing.
    pub async fn set_current_scene_collection(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentSceneCollection { name })
            .await
    }

    /// Creates a new scene collection, switching to it in the process.
    ///
    /// **Note:** This will block until the collection has finished changing.
    pub async fn create_scene_collection(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateSceneCollection { name })
            .await
    }

    /// Gets an array of all profiles.
    pub async fn list_profiles(&self) -> Result<responses::Profiles> {
        self.client.send_message(RequestType::GetProfileList).await
    }

    /// Switches to a profile.
    pub async fn set_current_profile(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProfile { name })
            .await
    }

    /// Creates a new profile, switching to it in the process.
    pub async fn create_profile(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateProfile { name })
            .await
    }

    /// Removes a profile. If the current profile is chosen, it will change to a different profile
    /// first.
    pub async fn remove_profile(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveProfile { name })
            .await
    }

    /// Gets a parameter from the current profile's configuration.
    pub async fn get_profile_parameter(
        &self,
        category: &str,
        name: &str,
    ) -> Result<responses::ProfileParameter> {
        self.client
            .send_message(RequestType::GetProfileParameter { category, name })
            .await
    }

    /// Sets the value of a parameter in the current profile's configuration.
    pub async fn set_profile_parameter(&self, parameter: SetProfileParameter<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetProfileParameter(parameter))
            .await
    }

    /// Gets the current video settings.
    ///
    /// **Note:** To get the true FPS value, divide the FPS numerator by the FPS denominator.
    /// Example: `60000/1001`.
    pub async fn video_settings(&self) -> Result<responses::VideoSettings> {
        self.client
            .send_message(RequestType::GetVideoSettings)
            .await
    }

    /// Sets the current video settings.
    ///
    /// **Note:** Fields must be specified in pairs. For example, you cannot set only
    /// [`SetVideoSettings::base_width`] without needing to specify
    /// [`SetVideoSettings::base_height`].
    pub async fn set_video_settings(&self, settings: SetVideoSettings) -> Result<()> {
        self.client
            .send_message(RequestType::SetVideoSettings(settings))
            .await
    }

    /// Gets the current stream service settings (stream destination).
    pub async fn stream_service_settings<T>(&self) -> Result<responses::StreamServiceSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message(RequestType::GetStreamServiceSettings)
            .await
    }

    /// Sets the current stream service settings (stream destination).
    ///
    /// **Note:** Simple RTMP settings can be set with type `rtmp_custom` and the settings fields
    /// `server` and `key`.
    pub async fn set_stream_service_settings<T>(&self, r#type: &'a str, settings: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetStreamServiceSettings {
                r#type,
                settings: serde_json::to_value(settings).map_err(Error::SerializeCustomData)?,
            })
            .await
    }

    /// Gets the current directory that the record output is set to.
    pub async fn record_directory(&self) -> Result<String> {
        self.client
            .send_message::<responses::RecordDirectory>(RequestType::GetRecordDirectory)
            .await
            .map(|rd| rd.record_directory)
    }
}
