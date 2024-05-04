//! Requests related to media inputs.

use serde::Serialize;
use time::Duration;

use super::inputs::InputId;
use crate::common::MediaAction;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetMediaInputStatus")]
    Status {
        /// Identifier of the media input.
        #[serde(flatten)]
        input: InputId<'a>,
    },
    #[serde(rename = "SetMediaInputCursor")]
    SetCursor {
        /// Identifier of the media input.
        #[serde(flatten)]
        input: InputId<'a>,
        /// New cursor position to set.
        #[serde(rename = "mediaCursor", with = "crate::serde::duration_millis")]
        cursor: Duration,
    },
    #[serde(rename = "OffsetMediaInputCursor")]
    OffsetCursor {
        /// Identifier of the media input.
        #[serde(flatten)]
        input: InputId<'a>,
        /// Value to offset the current cursor position by.
        #[serde(rename = "mediaCursorOffset", with = "crate::serde::duration_millis")]
        offset: Duration,
    },
    #[serde(rename = "TriggerMediaInputAction")]
    TriggerAction {
        /// Identifier of the media input.
        #[serde(flatten)]
        input: InputId<'a>,
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
