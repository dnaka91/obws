//! Requests related to media inputs.

use serde::Serialize;
use time::Duration;

use crate::common::MediaAction;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetMediaInputStatus")]
    Status {
        /// Name of the media input.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "SetMediaInputCursor")]
    SetCursor {
        /// Name of the media input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// New cursor position to set.
        #[serde(rename = "mediaCursor", with = "crate::serde::duration_millis")]
        cursor: Duration,
    },
    #[serde(rename = "OffsetMediaInputCursor")]
    OffsetCursor {
        /// Name of the media input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// Value to offset the current cursor position by.
        #[serde(rename = "mediaCursorOffset", with = "crate::serde::duration_millis")]
        offset: Duration,
    },
    #[serde(rename = "TriggerMediaInputAction")]
    TriggerAction {
        /// Name of the media input.
        #[serde(rename = "inputName")]
        input: &'a str,
        /// Identifier of the media action.
        #[serde(rename = "mediaAction")]
        action: MediaAction,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::MediaInputs(value)
    }
}
