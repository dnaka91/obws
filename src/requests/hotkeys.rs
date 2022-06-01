//! Requests related to hotkeys.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetHotkeyList")]
    ListHotkeys,
    #[serde(rename = "TriggerHotkeyByName")]
    TriggerHotkeyByName {
        /// Name of the hotkey to trigger.
        #[serde(rename = "hotkeyName")]
        name: &'a str,
    },
    #[serde(rename = "TriggerHotkeyByKeySequence")]
    TriggerHotkeyByKeySequence {
        /// The OBS key ID to use.
        #[serde(rename = "keyId")]
        id: &'a str,
        /// Object containing key modifiers to apply.
        #[serde(rename = "keyModifiers")]
        modifiers: KeyModifiers,
    },
    // TODO: Sleep
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Hotkeys(value)
    }
}

/// Request information for
/// [`crate::client::General::trigger_hotkey_by_key_sequence`].
#[derive(Default, Serialize)]
pub struct KeyModifiers {
    /// Press Shift.
    #[serde(rename = "shift")]
    pub shift: bool,
    /// Press CTRL.
    #[serde(rename = "control")]
    pub control: bool,
    /// Press ALT.
    #[serde(rename = "alt")]
    pub alt: bool,
    /// Press CMD (Mac).
    #[serde(rename = "command")]
    pub command: bool,
}
