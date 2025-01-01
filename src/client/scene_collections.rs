use super::Client;
use crate::{
    error::Result, requests::scene_collections::Request, responses::scene_collections as responses,
};

/// API functions related to scene collections.
pub struct SceneCollections<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneCollections<'a> {
    /// Gets an array of all scene collections.
    #[doc(alias = "GetSceneCollectionList")]
    pub async fn list(&self) -> Result<responses::SceneCollections> {
        self.client.send_message(Request::List).await
    }

    /// Get the currently active scene collection name.
    #[doc(alias = "GetSceneCollectionList")]
    pub async fn current(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::SceneCollections>(Request::List)
            .await
            .map(|sc| sc.current)
    }

    /// Switches to a scene collection.
    ///
    /// **Note:** This will block until the collection has finished changing.
    #[doc(alias = "SetCurrentSceneCollection")]
    pub async fn set_current(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::SetCurrent { name }).await
    }

    /// Creates a new scene collection, switching to it in the process.
    ///
    /// **Note:** This will block until the collection has finished changing.
    #[doc(alias = "CreateSceneCollection")]
    pub async fn create(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Create { name }).await
    }
}
