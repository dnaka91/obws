use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to scenes.
pub struct Scenes<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Scenes<'a> {
    pub async fn get_scene_list(&self) -> Result<responses::Scenes> {
        self.client.send_message(RequestType::GetSceneList).await
    }

    pub async fn get_current_program_scene(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentProgramScene>(RequestType::GetCurrentProgramScene)
            .await
            .map(|cps| cps.current_program_scene_name)
    }

    pub async fn set_current_program_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProgramScene { scene_name })
            .await
    }

    pub async fn get_current_preview_scene(&self) -> Result<Option<String>> {
        self.client
            .send_message::<responses::CurrentPreviewScene>(RequestType::GetCurrentPreviewScene)
            .await
            .map(|cps| cps.current_preview_scene_name)
    }

    pub async fn set_current_preview_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentPreviewScene { scene_name })
            .await
    }

    pub async fn set_scene_name(&self, scene_name: &str, new_scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneName {
                scene_name,
                new_scene_name,
            })
            .await
    }

    pub async fn create_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateScene { scene_name })
            .await
    }

    pub async fn remove_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveScene { scene_name })
            .await
    }
}
