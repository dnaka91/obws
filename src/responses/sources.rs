//! Responses related to sources.

use serde::{Deserialize, Serialize};

pub use super::ids::SourceId;

/// Response value for [`crate::client::Sources::active`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourceActive {
    /// Whether the source is showing in program.
    #[serde(rename = "videoActive")]
    pub active: bool,
    /// Whether the source is showing in the UI (preview, projector, properties).
    #[serde(rename = "videoShowing")]
    pub showing: bool,
}

/// Response value for [`crate::client::Sources::get_screenshot`].
#[derive(Debug, Deserialize)]
pub(crate) struct ImageData {
    /// Base64-encoded screenshot.
    #[serde(rename = "imageData")]
    pub image_data: String,
}
