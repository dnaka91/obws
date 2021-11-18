use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to recording.
pub struct Recording<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Recording<'a> {
    pub async fn get_record_status(&self) -> Result<responses::RecordStatus> {
        self.client.send_message(RequestType::GetRecordStatus).await
    }

    pub async fn toggle_record(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::ToggleRecord)
            .await
            .map(|oa| oa.output_active)
    }

    pub async fn start_record(&self) -> Result<()> {
        self.client.send_message(RequestType::StartRecord).await
    }

    pub async fn stop_record(&self) -> Result<()> {
        self.client.send_message(RequestType::StopRecord).await
    }

    pub async fn toggle_record_pause(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputPaused>(RequestType::ToggleRecordPause)
            .await
            .map(|op| op.output_paused)
    }

    pub async fn pause_record(&self) -> Result<()> {
        self.client.send_message(RequestType::PauseRecord).await
    }

    pub async fn resume_record(&self) -> Result<()> {
        self.client.send_message(RequestType::ResumeRecord).await
    }

    // Currently disabled in obs-websocket and will always fail.
    #[doc(hidden)]
    pub async fn get_record_directory(&self) -> Result<String> {
        self.client
            .send_message::<responses::RecordDirectory>(RequestType::GetRecordDirectory)
            .await
            .map(|rd| rd.record_directory)
    }
}
