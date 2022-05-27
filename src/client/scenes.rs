use super::Client;
use crate::{
    requests::{RequestType, SetSceneSceneTransitionOverride},
    responses, Result,
};

/// API functions related to scenes.
pub struct Scenes<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Gets an array of all scenes in OBS.
    pub async fn list(&self) -> Result<responses::Scenes> {
        self.client.send_message(RequestType::GetSceneList).await
    }

    /// Gets an array of all groups in OBS.
    ///
    /// Groups in OBS are actually scenes, but renamed and modified. In obs-websocket, we treat them
    /// as scenes where we can.
    pub async fn list_groups(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<responses::Groups>(RequestType::GetGroupList)
            .await
            .map(|g| g.groups)
    }

    /// Gets the current program scene.
    pub async fn current_program_scene(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentProgramScene>(RequestType::GetCurrentProgramScene)
            .await
            .map(|cps| cps.current_program_scene_name)
    }

    /// Sets the current program scene.
    pub async fn set_current_program_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProgramScene { scene_name })
            .await
    }

    /// Gets the current preview scene.
    ///
    /// Only available when studio mode is enabled.
    pub async fn current_preview_scene(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentPreviewScene>(RequestType::GetCurrentPreviewScene)
            .await
            .map(|cps| cps.current_preview_scene_name)
    }

    /// Sets the current preview scene.
    ///
    /// Only available when studio mode is enabled.
    pub async fn set_current_preview_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentPreviewScene { scene_name })
            .await
    }

    /// Sets the name of a scene (rename).
    pub async fn set_name(&self, name: &str, new: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneName { name, new })
            .await
    }

    /// Creates a new scene in OBS.
    pub async fn create(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateScene { name })
            .await
    }

    /// Removes a scene from OBS.
    pub async fn remove(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveScene { name })
            .await
    }

    /// Gets the scene transition overridden for a scene.
    pub async fn transition_override(
        &self,
        scene_name: &str,
    ) -> Result<responses::SceneTransitionOverride> {
        self.client
            .send_message(RequestType::GetSceneSceneTransitionOverride { scene_name })
            .await
    }

    /// Sets the scene transition overridden for a scene.
    pub async fn set_transition_override(
        &self,
        transition_override: SetSceneSceneTransitionOverride<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneSceneTransitionOverride(
                transition_override,
            ))
            .await
    }
}
