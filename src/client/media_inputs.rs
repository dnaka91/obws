use time::Duration;

use super::Client;
use crate::{common::MediaAction, requests::RequestType, responses, Result};

/// API functions related to media inputs.
pub struct MediaInputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MediaInputs<'a> {
    /// Gets the status of a media input.
    ///
    /// - `input_name`: Name of the media input.
    pub async fn get_media_input_status(&self, input_name: &str) -> Result<responses::MediaStatus> {
        self.client
            .send_message(RequestType::GetMediaInputStatus { input_name })
            .await
    }

    /// Sets the cursor position of a media input.
    ///
    /// This request does not perform bounds checking of the cursor position.
    ///
    /// - `input_name`: Name of the media input.
    /// - `media_cursor`: New cursor position to set.
    pub async fn set_media_input_cursor(
        &self,
        input_name: &str,
        media_cursor: Duration,
    ) -> Result<()> {
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
    ///
    /// - `input_name`: Name of the media input.
    /// - `media_cursor_offset`: Value to offset the current cursor position by.
    pub async fn offset_media_input_cursor(
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
    ///
    /// - `input_name`: Name of the media input.
    /// - `media_action`: Identifier of the media action.
    pub async fn trigger_media_input_action(
        &self,
        input_name: &str,
        media_action: MediaAction,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerMediaInputAction {
                input_name,
                media_action,
            })
            .await
    }
}
