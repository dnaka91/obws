use super::Client;
use crate::{
    requests::{RequestType, SetProfileParameter},
    responses::{ProfileParameter, Profiles, SceneCollections},
    Result,
};

/// API functions related to OBS configuration.
pub struct Config<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Config<'a> {
    pub async fn get_scene_collection_list(&self) -> Result<SceneCollections> {
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

    pub async fn get_profile_list(&self) -> Result<Profiles> {
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
    ) -> Result<ProfileParameter> {
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
}
