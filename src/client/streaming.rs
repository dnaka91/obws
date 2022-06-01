use super::Client;
use crate::{requests::streaming::Request, responses::streaming as responses, Result};

/// API functions related to streaming.
pub struct Streaming<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Streaming<'a> {
    /// Gets the status of the stream output.
    pub async fn status(&self) -> Result<responses::StreamStatus> {
        self.client.send_message(Request::GetStreamStatus).await
    }

    /// Toggles the status of the stream output.
    pub async fn toggle(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::ToggleStream)
            .await
            .map(|ts| ts.active)
    }

    /// Starts the stream output.
    pub async fn start(&self) -> Result<()> {
        self.client.send_message(Request::StartStream).await
    }

    /// Stops the stream output.
    pub async fn stop(&self) -> Result<()> {
        self.client.send_message(Request::StopStream).await
    }

    /// Sends CEA-608 caption text over the stream output.
    pub async fn send_caption(&self, caption_text: &str) -> Result<()> {
        self.client
            .send_message(Request::SendStreamCaption { caption_text })
            .await
    }
}
