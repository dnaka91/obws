use super::Client;
use crate::{requests::recording::Request, responses::recording as responses, Result};

/// API functions related to recording.
pub struct Recording<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Recording<'a> {
    /// Gets the status of the record output.
    pub async fn status(&self) -> Result<responses::RecordStatus> {
        self.client.send_message(Request::Status).await
    }

    /// Toggles the status of the record output.
    pub async fn toggle(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Toggle)
            .await
            .map(|oa| oa.active)
    }

    /// Starts the record output.
    pub async fn start(&self) -> Result<()> {
        self.client.send_message(Request::Start).await
    }

    /// Stops the record output.
    pub async fn stop(&self) -> Result<()> {
        self.client.send_message(Request::Stop).await
    }

    /// Toggles pause on the record output.
    pub async fn toggle_pause(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputPaused>(Request::TogglePause)
            .await
            .map(|op| op.paused)
    }

    /// Pauses the record output.
    pub async fn pause(&self) -> Result<()> {
        self.client.send_message(Request::Pause).await
    }

    /// Resumes the record output.
    pub async fn resume(&self) -> Result<()> {
        self.client.send_message(Request::Resume).await
    }
}
