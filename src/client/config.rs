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
    ///
    /// - `realm`: The data realm to select.
    /// - `slot_name`: The name of the slot to retrieve data from.
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
    pub async fn get_scene_collection_list(&self) -> Result<responses::SceneCollections> {
        self.client
            .send_message(RequestType::GetSceneCollectionList)
            .await
    }

    /// Switches to a scene collection.
    ///
    /// **Note:** This will block until the collection has finished changing.
    ///
    /// - `scene_collection_name`: Name of the scene collection to switch to.
    pub async fn set_current_scene_collection(&self, scene_collection_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentSceneCollection {
                scene_collection_name,
            })
            .await
    }

    /// Creates a new scene collection, switching to it in the process.
    ///
    /// **Note:** This will block until the collection has finished changing.
    ///
    /// - `scene_collection_name`: Name for the new scene collection.
    pub async fn create_scene_collection(&self, scene_collection_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateSceneCollection {
                scene_collection_name,
            })
            .await
    }

    /// Gets an array of all profiles.
    pub async fn get_profile_list(&self) -> Result<responses::Profiles> {
        self.client.send_message(RequestType::GetProfileList).await
    }

    /// Switches to a profile.
    ///
    /// - `profile_name`: Name of the profile to switch to.
    pub async fn set_current_profile(&self, profile_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProfile { profile_name })
            .await
    }

    /// Creates a new profile, switching to it in the process.
    ///
    /// - `profile_name`: Name for the new profile.
    pub async fn create_profile(&self, profile_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateProfile { profile_name })
            .await
    }

    /// Removes a profile. If the current profile is chosen, it will change to a different profile
    /// first.
    ///
    /// - `profile_name`: Name of the profile to remove.
    pub async fn remove_profile(&self, profile_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveProfile { profile_name })
            .await
    }

    /// Gets a parameter from the current profile's configuration.
    ///
    /// - `parameter_category`: Category of the parameter to get.
    /// - `parameter_name`: Name of the parameter to get.
    pub async fn get_profile_parameter(
        &self,
        parameter_category: &str,
        parameter_name: &str,
    ) -> Result<responses::ProfileParameter> {
        self.client
            .send_message(RequestType::GetProfileParameter {
                parameter_category,
                parameter_name,
            })
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
    pub async fn get_video_settings(&self) -> Result<responses::VideoSettings> {
        self.client
            .send_message(RequestType::GetVideoSettings)
            .await
    }

    /// Sets the current video settings.
    ///
    /// **Note:** Fields must be specified in pairs. For example, you cannot set only [`base_width`]
    /// without needing to specify [`base_height`].
    ///
    /// [`base_width`]: SetVideoSettings::base_width
    /// [`base_height`]: SetVideoSettings::base_height
    pub async fn set_video_settings(&self, settings: SetVideoSettings) -> Result<()> {
        self.client
            .send_message(RequestType::SetVideoSettings(settings))
            .await
    }

    /// Gets the current stream service settings (stream destination).
    pub async fn get_stream_service_settings<T>(
        &self,
    ) -> Result<responses::StreamServiceSettings<T>>
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
    ///
    /// - `stream_service_type`: Type of stream service to apply. Example: `rtmp_common` or
    ///   `rtmp_custom`.
    /// - `stream_service_settings`: Settings to apply to the service.
    pub async fn set_stream_service_settings<T>(
        &self,
        stream_service_type: &'a str,
        stream_service_settings: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetStreamServiceSettings {
                stream_service_type,
                stream_service_settings: serde_json::to_value(stream_service_settings)
                    .map_err(Error::SerializeCustomData)?,
            })
            .await
    }

    /// Gets the current directory that the record output is set to.
    pub async fn get_record_directory(&self) -> Result<String> {
        self.client
            .send_message::<responses::RecordDirectory>(RequestType::GetRecordDirectory)
            .await
            .map(|rd| rd.record_directory)
    }
}
