use time::Duration;

use super::Client;
use crate::{common::MediaAction, requests::RequestType, responses, Result};

/// API functions related to media inputs.
pub struct MediaInputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MediaInputs<'a> {
    pub async fn get_media_input_status(&self, input_name: &str) -> Result<responses::MediaStatus> {
        self.client
            .send_message(RequestType::GetMediaInputStatus { input_name })
            .await
    }

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
