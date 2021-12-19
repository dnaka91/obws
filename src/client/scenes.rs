use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to scenes.
pub struct Scenes<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Gets an array of all scenes in OBS.
    pub async fn get_scene_list(&self) -> Result<responses::Scenes> {
        self.client.send_message(RequestType::GetSceneList).await
    }

    /// Gets the current program scene.
    pub async fn get_current_program_scene(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentProgramScene>(RequestType::GetCurrentProgramScene)
            .await
            .map(|cps| cps.current_program_scene_name)
    }

    /// Sets the current program scene.
    ///
    /// - `scene_name`: Scene to set as the current program scene.
    pub async fn set_current_program_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProgramScene { scene_name })
            .await
    }

    /// Gets the current preview scene.
    ///
    /// Only available when studio mode is enabled.
    pub async fn get_current_preview_scene(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentPreviewScene>(RequestType::GetCurrentPreviewScene)
            .await
            .map(|cps| cps.current_preview_scene_name)
    }

    /// Sets the current preview scene.
    ///
    /// Only available when studio mode is enabled.
    ///
    /// - `scene_name`: Scene to set as the current preview scene.
    pub async fn set_current_preview_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentPreviewScene { scene_name })
            .await
    }

    /// Sets the name of a scene (rename).
    ///
    /// - `scene_name`: Name of the scene to be renamed.
    /// - `new_scene_name`: New name for the scene.
    pub async fn set_scene_name(&self, scene_name: &str, new_scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneName {
                scene_name,
                new_scene_name,
            })
            .await
    }

    /// Creates a new scene in OBS.
    ///
    /// - `scene_name`: Name for the new scene.
    pub async fn create_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateScene { scene_name })
            .await
    }

    /// Removes a scene from OBS.
    ///
    /// - `scene_name`: Name of the scene to remove.
    pub async fn remove_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveScene { scene_name })
            .await
    }
}
