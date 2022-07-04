//! Responses related to the virtual camera.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct OutputActive {
    /// New state of the stream output.
    #[serde(rename = "outputActive")]
    pub active: bool,
}
