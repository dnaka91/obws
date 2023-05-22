use super::Client;
use crate::{
    requests::scenes::{Request, SetTransitionOverride},
    responses::scenes as responses,
    Result,
};

/// API functions related to scenes.
pub struct Scenes<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Gets an array of all scenes in OBS.
    #[doc(alias = "GetSceneList")]
    pub async fn list(&self) -> Result<responses::Scenes> {
        self.client.send_message(Request::List).await
    }

    /// Gets an array of all groups in OBS.
    ///
    /// Groups in OBS are actually scenes, but renamed and modified. In obs-websocket, we treat them
    /// as scenes where we can.
    #[doc(alias = "GetGroupList")]
    pub async fn list_groups(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<_, responses::Groups>(Request::ListGroups)
            .await
            .map(|g| g.groups)
    }

    /// Gets the current program scene.
    #[doc(alias = "GetCurrentProgramScene")]
    pub async fn current_program_scene(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::CurrentProgramScene>(Request::CurrentProgramScene)
            .await
            .map(|cps| cps.current_program_scene_name)
    }

    /// Sets the current program scene.
    #[doc(alias = "SetCurrentProgramScene")]
    pub async fn set_current_program_scene(&self, scene: &str) -> Result<()> {
        self.client
            .send_message(Request::SetCurrentProgramScene { scene })
            .await
    }

    /// Gets the current preview scene.
    ///
    /// Only available when studio mode is enabled.
    #[doc(alias = "GetCurrentPreviewScene")]
    pub async fn current_preview_scene(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::CurrentPreviewScene>(Request::CurrentPreviewScene)
            .await
            .map(|cps| cps.current_preview_scene_name)
    }

    /// Sets the current preview scene.
    ///
    /// Only available when studio mode is enabled.
    #[doc(alias = "SetCurrentPreviewScene")]
    pub async fn set_current_preview_scene(&self, scene: &str) -> Result<()> {
        self.client
            .send_message(Request::SetCurrentPreviewScene { scene })
            .await
    }

    /// Sets the name of a scene (rename).
    #[doc(alias = "SetSceneName")]
    pub async fn set_name(&self, scene: &str, new_name: &str) -> Result<()> {
        self.client
            .send_message(Request::SetName { scene, new_name })
            .await
    }

    /// Creates a new scene in OBS.
    #[doc(alias = "CreateScene")]
    pub async fn create(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Create { name }).await
    }

    /// Removes a scene from OBS.
    #[doc(alias = "RemoveScene")]
    pub async fn remove(&self, scene: &str) -> Result<()> {
        self.client.send_message(Request::Remove { scene }).await
    }

    /// Gets the scene transition overridden for a scene.
    #[doc(alias = "GetSceneSceneTransitionOverride")]
    pub async fn transition_override(
        &self,
        scene: &str,
    ) -> Result<responses::SceneTransitionOverride> {
        self.client
            .send_message(Request::TransitionOverride { scene })
            .await
    }

    /// Sets the scene transition overridden for a scene.
    #[doc(alias = "SetSceneSceneTransitionOverride")]
    pub async fn set_transition_override(
        &self,
        transition_override: SetTransitionOverride<'_>,
    ) -> Result<()> {
        self.client
            .send_message(Request::SetTransitionOverride(transition_override))
            .await
    }
}
