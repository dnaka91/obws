use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to recording.
pub struct Recording<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Recording<'a> {
    /// Gets the status of the record output.
    pub async fn get_record_status(&self) -> Result<responses::RecordStatus> {
        self.client.send_message(RequestType::GetRecordStatus).await
    }

    /// Toggles the status of the record output.
    pub async fn toggle_record(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::ToggleRecord)
            .await
            .map(|oa| oa.output_active)
    }

    /// Starts the record output.
    pub async fn start_record(&self) -> Result<()> {
        self.client.send_message(RequestType::StartRecord).await
    }

    /// Stops the record output.
    pub async fn stop_record(&self) -> Result<()> {
        self.client.send_message(RequestType::StopRecord).await
    }

    /// Toggles pause on the record output.
    pub async fn toggle_record_pause(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputPaused>(RequestType::ToggleRecordPause)
            .await
            .map(|op| op.output_paused)
    }

    /// Pauses the record output.
    pub async fn pause_record(&self) -> Result<()> {
        self.client.send_message(RequestType::PauseRecord).await
    }

    /// Resumes the record output.
    pub async fn resume_record(&self) -> Result<()> {
        self.client.send_message(RequestType::ResumeRecord).await
    }
}
