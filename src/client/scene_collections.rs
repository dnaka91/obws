use super::Client;
use crate::{
    requests::scene_collections::Request, responses::scene_collections as responses, Result,
};

/// API functions related to scene collections.
pub struct SceneCollections<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneCollections<'a> {
    /// Gets an array of all scene collections.
    pub async fn list(&self) -> Result<responses::SceneCollections> {
        self.client.send_message(Request::List).await
    }

    /// Get the currently active scene collection name.
    pub async fn current(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::SceneCollections>(Request::List)
            .await
            .map(|sc| sc.current)
    }

    /// Switches to a scene collection.
    ///
    /// **Note:** This will block until the collection has finished changing.
    pub async fn set_current(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::SetCurrent { name }).await
    }

    /// Creates a new scene collection, switching to it in the process.
    ///
    /// **Note:** This will block until the collection has finished changing.
    pub async fn create(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Create { name }).await
    }
}
