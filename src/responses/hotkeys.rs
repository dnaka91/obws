//! Responses related to hotkeys.

use serde::Deserialize;

/// Response value for [`crate::client::General::get_hotkey_list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Hotkeys {
    /// Array of hotkey names.
    pub hotkeys: Vec<String>,
}
