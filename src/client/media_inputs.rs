use time::Duration;

use super::Client;
use crate::{common::MediaAction, requests::RequestType, responses, Result};

/// API functions related to media inputs.
pub struct MediaInputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MediaInputs<'a> {
    /// Gets the status of a media input.
    pub async fn status(&self, input_name: &str) -> Result<responses::MediaStatus> {
        self.client
            .send_message(RequestType::GetMediaInputStatus { input_name })
            .await
    }

    /// Sets the cursor position of a media input.
    ///
    /// This request does not perform bounds checking of the cursor position.
    pub async fn set_cursor(&self, input_name: &str, media_cursor: Duration) -> Result<()> {
        self.client
            .send_message(RequestType::SetMediaInputCursor {
                input_name,
                media_cursor,
            })
            .await
    }

    /// Offsets the current cursor position of a media input by the specified value.
    ///
    /// This request does not perform bounds checking of the cursor position.
    pub async fn offset_cursor(
        &self,
        input_name: &str,
        media_cursor_offset: Duration,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::OffsetMediaInputCursor {
                input_name,
                media_cursor_offset,
            })
            .await
    }

    /// Triggers an action on a media input.
    pub async fn trigger_action(&self, input_name: &str, media_action: MediaAction) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerMediaInputAction {
                input_name,
                media_action,
            })
            .await
    }
}
