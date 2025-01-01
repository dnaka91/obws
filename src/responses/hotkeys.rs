//! Responses related to hotkeys.

use serde::Deserialize;

/// Response value for [`crate::client::Hotkeys::list`].
#[derive(Debug, Deserialize)]
pub(crate) struct Hotkeys {
    /// Array of hotkey names.
    #[serde(rename = "hotkeys")]
    pub hotkeys: Vec<String>,
}
