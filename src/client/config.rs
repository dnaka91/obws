use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{
    requests::config::{Realm, Request, SetPersistentData, SetProfileParameter, SetVideoSettings},
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
            .send_message(Request::GetPersistentData { realm, slot_name })
            .await
    }

    /// Sets the value of a "slot" from the selected persistent data realm.
    pub async fn set_persistent_data(&self, data: SetPersistentData<'_>) -> Result<()> {
        self.client
            .send_message(Request::SetPersistentData(data))
            .await
    }

    /// Gets an array of all scene collections.
    pub async fn list_scene_collections(&self) -> Result<responses::SceneCollections> {
        self.client.send_message(Request::ListSceneColletions).await
    }

    /// Switches to a scene collection.
    ///
    /// **Note:** This will block until the collection has finished changing.
    pub async fn set_current_scene_collection(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::SetCurrentSceneCollection { name })
            .await
    }

    /// Creates a new scene collection, switching to it in the process.
    ///
    /// **Note:** This will block until the collection has finished changing.
    pub async fn create_scene_collection(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::CreateSceneCollection { name })
            .await
    }

    /// Gets an array of all profiles.
    pub async fn list_profiles(&self) -> Result<responses::Profiles> {
        self.client.send_message(Request::ListProfiles).await
    }

    /// Switches to a profile.
    pub async fn set_current_profile(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::SetCurrentProfile { name })
            .await
    }

    /// Creates a new profile, switching to it in the process.
    pub async fn create_profile(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::CreateProfile { name })
            .await
    }

    /// Removes a profile. If the current profile is chosen, it will change to a different profile
    /// first.
    pub async fn remove_profile(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::RemoveProfile { name })
            .await
    }

    /// Gets a parameter from the current profile's configuration.
    pub async fn get_profile_parameter(
        &self,
        category: &str,
        name: &str,
    ) -> Result<responses::ProfileParameter> {
        self.client
            .send_message(Request::GetProfileParameter { category, name })
            .await
    }

    /// Sets the value of a parameter in the current profile's configuration.
    pub async fn set_profile_parameter(&self, parameter: SetProfileParameter<'_>) -> Result<()> {
        self.client
            .send_message(Request::SetProfileParameter(parameter))
            .await
    }

    /// Gets the current video settings.
    ///
    /// **Note:** To get the true FPS value, divide the FPS numerator by the FPS denominator.
    /// Example: `60000/1001`.
    pub async fn video_settings(&self) -> Result<responses::VideoSettings> {
        self.client.send_message(Request::VideoSettings).await
    }

    /// Sets the current video settings.
    ///
    /// **Note:** Fields must be specified in pairs. For example, you cannot set only
    /// [`SetVideoSettings::base_width`] without needing to specify
    /// [`SetVideoSettings::base_height`].
    pub async fn set_video_settings(&self, settings: SetVideoSettings) -> Result<()> {
        self.client
            .send_message(Request::SetVideoSettings(settings))
            .await
    }

    /// Gets the current stream service settings (stream destination).
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
    pub async fn set_stream_service_settings<T>(&self, r#type: &'a str, settings: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetStreamServiceSettings {
                r#type,
                settings: serde_json::to_value(settings).map_err(Error::SerializeCustomData)?,
            })
            .await
    }

    /// Gets the current directory that the record output is set to.
    pub async fn record_directory(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::RecordDirectory>(Request::RecordDirectory)
            .await
            .map(|rd| rd.record_directory)
    }
}
