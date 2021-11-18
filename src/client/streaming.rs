use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to streaming.
pub struct Streaming<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Streaming<'a> {
    pub async fn get_stream_status(&self) -> Result<responses::StreamStatus> {
        self.client.send_message(RequestType::GetStreamStatus).await
    }

    pub async fn toggle_stream(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::ToggleStream)
            .await
            .map(|ts| ts.output_active)
    }

    pub async fn start_stream(&self) -> Result<()> {
        self.client.send_message(RequestType::StartStream).await
    }

    pub async fn stop_stream(&self) -> Result<()> {
        self.client.send_message(RequestType::StopStream).await
    }
}
