//! Responses related to scene collections.

use serde::Deserialize;

/// Response value for [`crate::client::SceneCollections::list`].
#[derive(Debug, Deserialize)]
pub struct SceneCollections {
    /// The name of the current scene collection.
    #[serde(rename = "currentSceneCollectionName")]
    pub current: String,
    /// Array of all available scene collections.
    #[serde(rename = "sceneCollections")]
    pub collections: Vec<String>,
}
