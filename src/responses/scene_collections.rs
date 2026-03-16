//! Responses related to scene collections.

use serde::{Deserialize, Serialize};

/// Response value for [`crate::client::SceneCollections::list`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub struct SceneCollections {
    /// The name of the current scene collection.
    #[serde(rename = "currentSceneCollectionName")]
    pub current: String,
    /// Array of all available scene collections.
    #[serde(rename = "sceneCollections")]
    pub collections: Vec<String>,
}
