//! Requests related to the user interface.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetStudioModeEnabled")]
    GetStudioModeEnabled,
    #[serde(rename = "SetStudioModeEnabled")]
    SetStudioModeEnabled {
        /// Enable or disable the studio mode.
        #[serde(rename = "studioModeEnabled")]
        enabled: bool,
    },
    #[serde(rename = "OpenInputPropertiesDialog")]
    OpenInputPropertiesDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "OpenInputFiltersDialog")]
    OpenInputFiltersDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "OpenInputInteractDialog")]
    OpenInputInteractDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "GetMonitorList")]
    GetMonitorList,
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Ui(value)
    }
}
