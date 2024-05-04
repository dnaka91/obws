use time::Duration;

use super::Client;
use crate::{
    common::MediaAction,
    requests::{inputs::InputId, media_inputs::Request},
    responses::media_inputs as responses,
    Result,
};

/// API functions related to media inputs.
pub struct MediaInputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MediaInputs<'a> {
    /// Gets the status of a media input.
    #[doc(alias = "GetMediaInputStatus")]
    pub async fn status(&self, input: InputId<'_>) -> Result<responses::MediaStatus> {
        self.client.send_message(Request::Status { input }).await
    }

    /// Sets the cursor position of a media input.
    ///
    /// This request does not perform bounds checking of the cursor position.
    #[doc(alias = "SetMediaInputCursor")]
    pub async fn set_cursor(&self, input: InputId<'_>, cursor: Duration) -> Result<()> {
        self.client
            .send_message(Request::SetCursor { input, cursor })
            .await
    }

    /// Offsets the current cursor position of a media input by the specified value.
    ///
    /// This request does not perform bounds checking of the cursor position.
    #[doc(alias = "OffsetMediaInputCursor")]
    pub async fn offset_cursor(&self, input: InputId<'_>, offset: Duration) -> Result<()> {
        self.client
            .send_message(Request::OffsetCursor { input, offset })
            .await
    }

    /// Triggers an action on a media input.
    #[doc(alias = "TriggerMediaInputAction")]
    pub async fn trigger_action(&self, input: InputId<'_>, action: MediaAction) -> Result<()> {
        self.client
            .send_message(Request::TriggerAction { input, action })
            .await
    }
}
