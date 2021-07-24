use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{
    requests::{RequestType, SetProfileParameter, SetVideoSettings},
    responses, Error, Result,
};

/// API functions related to OBS configuration.
pub struct Config<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Config<'a> {
    pub async fn get_scene_collection_list(&self) -> Result<responses::SceneCollections> {
        self.client
            .send_message(RequestType::GetSceneCollectionList)
            .await
    }

    pub async fn set_current_scene_collection(&self, scene_collection_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentSceneCollection {
                scene_collection_name,
            })
            .await
    }

    pub async fn get_profile_list(&self) -> Result<responses::Profiles> {
        self.client.send_message(RequestType::GetProfileList).await
    }

    pub async fn set_current_profile(&self, profile_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProfile { profile_name })
            .await
    }

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

    pub async fn set_profile_parameter(&self, parameter: SetProfileParameter<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetProfileParameter(parameter))
            .await
    }

    pub async fn get_video_settings(&self) -> Result<responses::VideoSettings> {
        self.client
            .send_message(RequestType::GetVideoSettings)
            .await
    }

    pub async fn set_video_settings(&self, settings: SetVideoSettings) -> Result<()> {
        self.client
            .send_message(RequestType::SetVideoSettings(settings))
            .await
    }

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
}
