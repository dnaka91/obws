//! Requests related to scenes.

use serde::Serialize;
use serde_with::skip_serializing_none;

pub use super::ids::SceneId;

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request {
    #[serde(rename = "GetCanvasList")]
    List,
}

impl From<Request> for super::RequestType<'_> {
    fn from(value: Request) -> Self {
        super::RequestType::Canvases(value)
    }
}
