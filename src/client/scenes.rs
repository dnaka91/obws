use super::Client;
use crate::requests::{RequestType, Scene, SceneTransitionOverride};
use crate::responses;
use crate::Result;

/// API functions related to scenes.
pub struct Scenes<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Scenes<'a> {
    /// Switch to the specified scene.
    ///
    /// - `scene_name`: Name of the scene to switch to.
    pub async fn set_current_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentScene { scene_name })
            .await
    }

    /// Get the current scene's name and source items.
    pub async fn get_current_scene(&self) -> Result<responses::CurrentScene> {
        self.client.send_message(RequestType::GetCurrentScene).await
    }

    /// Get a list of scenes in the currently active profile.
    pub async fn get_scene_list(&self) -> Result<responses::SceneList> {
        self.client.send_message(RequestType::GetSceneList).await
    }

    /// Create a new scene scene.
    ///
    /// - `scene_name`: Name of the scene to create.
    pub async fn create_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::CreateScene { scene_name })
            .await
    }

    /// Changes the order of scene items in the requested scene.
    ///
    /// - `scene`: Name of the scene to reorder (defaults to current).
    /// - `items`: Ordered list of objects with name and/or id specified. Id preferred due to
    ///   uniqueness per scene
    pub async fn reorder_scene_items(
        &self,
        scene: Option<&str>,
        items: &[Scene<'_>],
    ) -> Result<()> {
        self.client
            .send_message(RequestType::ReorderSceneItems { scene, items })
            .await
    }

    /// Set a scene to use a specific transition override.
    pub async fn set_scene_transition_override(
        &self,
        scene_transition: SceneTransitionOverride<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneTransitionOverride(scene_transition))
            .await
    }

    /// Remove any transition override on a scene.
    ///
    /// - `scene_name`: Name of the scene to remove the override from.
    pub async fn remove_scene_transition_override(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveSceneTransitionOverride { scene_name })
            .await
    }

    /// Get the current scene transition override.
    ///
    /// - `scene_name`: Name of the scene to get the override for.
    pub async fn get_scene_transition_override(
        &self,
        scene_name: &str,
    ) -> Result<responses::SceneTransitionOverride> {
        self.client
            .send_message(RequestType::GetSceneTransitionOverride { scene_name })
            .await
    }
}
