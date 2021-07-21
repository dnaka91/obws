use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to scene collections.
pub struct SceneCollections<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneCollections<'a> {
    /// Change the active scene collection.
    ///
    /// - `sc_name`: Name of the desired scene collection.
    pub async fn set_current_scene_collection(&self, sc_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentSceneCollection { sc_name })
            .await
    }

    /// Get the name of the current scene collection.
    pub async fn get_current_scene_collection(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentSceneCollection>(
                RequestType::GetCurrentSceneCollection,
            )
            .await
            .map(|csc| csc.sc_name)
    }

    /// List available scene collections.
    pub async fn list_scene_collections(&self) -> Result<Vec<responses::SceneCollection>> {
        self.client
            .send_message::<responses::SceneCollections>(RequestType::ListSceneCollections)
            .await
            .map(|sc| sc.scene_collections)
    }
}
