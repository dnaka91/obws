use super::Client;
use crate::requests::{RequestType, SetStreamSettings, Stream};
use crate::responses;
use crate::Result;

/// API functions related to streaming.
pub struct Streaming<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Streaming<'a> {
    /// Get current streaming and recording status.
    pub async fn get_streaming_status(&self) -> Result<responses::StreamingStatus> {
        self.client
            .send_message(RequestType::GetStreamingStatus)
            .await
    }

    /// Toggle streaming on or off (depending on the current stream state).
    pub async fn start_stop_streaming(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StartStopStreaming)
            .await
    }

    /// Start streaming. Will return an `error` if streaming is already active.
    ///
    /// - `stream`: Special stream configuration. Note: these won't be saved to OBS' configuration.
    pub async fn start_streaming(&self, stream: Option<Stream<'_>>) -> Result<()> {
        self.client
            .send_message(RequestType::StartStreaming { stream })
            .await
    }

    /// Stop streaming. Will return an `error` if streaming is not active.
    pub async fn stop_streaming(&self) -> Result<()> {
        self.client.send_message(RequestType::StopStreaming).await
    }

    /// Sets one or more attributes of the current streaming server settings. Any options not passed
    /// will remain unchanged. Returns the updated settings in response. If 'type' is different than
    /// the current streaming service type, all settings are required. Returns the full settings of
    /// the stream (the same as GetStreamSettings).
    pub async fn set_stream_settings(&self, settings: SetStreamSettings<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetStreamSettings(settings))
            .await
    }

    /// Get the current streaming server settings.
    pub async fn get_stream_settings(&self) -> Result<responses::GetStreamSettings> {
        self.client
            .send_message(RequestType::GetStreamSettings)
            .await
    }

    /// Save the current streaming server settings to disk.
    pub async fn save_stream_settings(&self) -> Result<()> {
        self.client
            .send_message(RequestType::SaveStreamSettings)
            .await
    }

    /// Send the provided text as embedded CEA-608 caption data.
    ///
    /// - `text`: Captions text.
    pub async fn send_captions(&self, text: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SendCaptions { text })
            .await
    }
}
