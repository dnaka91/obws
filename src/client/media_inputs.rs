use time::Duration;

use super::Client;
use crate::{common::MediaAction, requests::media_inputs::Request, responses, Result};

/// API functions related to media inputs.
pub struct MediaInputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MediaInputs<'a> {
    /// Gets the status of a media input.
    pub async fn status(&self, input: &str) -> Result<responses::MediaStatus> {
        self.client.send_message(Request::Status { input }).await
    }

    /// Sets the cursor position of a media input.
    ///
    /// This request does not perform bounds checking of the cursor position.
    pub async fn set_cursor(&self, input: &str, cursor: Duration) -> Result<()> {
        self.client
            .send_message(Request::SetCursor { input, cursor })
            .await
    }

    /// Offsets the current cursor position of a media input by the specified value.
    ///
    /// This request does not perform bounds checking of the cursor position.
    pub async fn offset_cursor(&self, input: &str, offset: Duration) -> Result<()> {
        self.client
            .send_message(Request::OffsetCursor { input, offset })
            .await
    }

    /// Triggers an action on a media input.
    pub async fn trigger_action(&self, input: &str, action: MediaAction) -> Result<()> {
        self.client
            .send_message(Request::TriggerAction { input, action })
            .await
    }
}
