//! Requests related to hotkeys.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetHotkeyList")]
    List,
    #[serde(rename = "TriggerHotkeyByName")]
    TriggerByName {
        /// Name of the hotkey to trigger.
        #[serde(rename = "hotkeyName")]
        name: &'a str,
        /// Name of context of the hotkey to trigger.
        #[serde(rename = "contextName")]
        context: Option<&'a str>,
    },
    #[serde(rename = "TriggerHotkeyByKeySequence")]
    TriggerBySequence {
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
/// [`crate::client::Hotkeys::trigger_by_sequence`].
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
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
