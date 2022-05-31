//! General requests, not fitting into any category.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetVersion")]
    Version,
    #[serde(rename = "GetStats")]
    Stats,
    #[serde(rename = "BroadcastCustomEvent")]
    BroadcastCustomEvent {
        /// Data payload to emit to all receivers.
        #[serde(rename = "eventData")]
        event_data: serde_json::Value,
    },
    #[serde(rename = "CallVendorRequest")]
    CallVendorRequest(CallVendorRequestInternal<'a>),
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
        super::RequestType::General(value)
    }
}

/// Request information for [`crate::client::General::call_vendor_request`].
pub struct CallVendorRequest<'a, T> {
    /// Name of the vendor to use.
    pub vendor_name: &'a str,
    /// The request type to call.
    pub request_type: &'a str,
    /// Object containing appropriate request data.
    pub request_data: &'a T,
}

/// Request information for [`crate::client::General::call_vendor_request`].
#[derive(Default, Serialize)]
pub(crate) struct CallVendorRequestInternal<'a> {
    /// Name of the vendor to use.
    #[serde(rename = "vendorName")]
    pub vendor_name: &'a str,
    /// The request type to call.
    #[serde(rename = "requestType")]
    pub request_type: &'a str,
    /// Object containing appropriate request data.
    #[serde(rename = "requestData")]
    pub request_data: serde_json::Value,
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
