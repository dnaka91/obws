//! Requests related to scene collections.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
   #[serde(rename = "GetSceneCollectionList")]
    List,
    #[serde(rename = "SetCurrentSceneCollection")]
    SetCurrent {
        /// Name of the scene collection to switch to.
        #[serde(rename = "sceneCollectionName")]
        name: &'a str,
    },
    #[serde(rename = "CreateSceneCollection")]
    Create {
        /// Name for the new scene collection.
        #[serde(rename = "sceneCollectionName")]
        name: &'a str,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::SceneCollections(value)
    }
}
