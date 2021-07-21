use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to the replay buffer.
pub struct ReplayBuffer<'a> {
    pub(super) client: &'a Client,
}

impl<'a> ReplayBuffer<'a> {
    /// Get the status of the OBS replay buffer.
    pub async fn get_replay_buffer_status(&self) -> Result<bool> {
        self.client
            .send_message::<responses::ReplayBufferStatus>(RequestType::GetReplayBufferStatus)
            .await
            .map(|rbs| rbs.is_replay_buffer_active)
    }

    /// Toggle the Replay Buffer on/off (depending on the current state of the replay buffer).
    pub async fn start_stop_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StartStopReplayBuffer)
            .await
    }

    /// Start recording into the Replay Buffer. Will return an `error` if the Replay Buffer is
    /// already active or if the "Save Replay Buffer" hotkey is not set in OBS' settings. Setting
    /// this hotkey is mandatory, even when triggering saves only through obs-websocket.
    pub async fn start_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StartReplayBuffer)
            .await
    }

    /// Stop recording into the Replay Buffer. Will return an `error` if the Replay Buffer is not
    /// active.
    pub async fn stop_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StopReplayBuffer)
            .await
    }

    /// Flush and save the contents of the Replay Buffer to disk. This is basically the same as
    /// triggering the "Save Replay Buffer" hotkey. Will return an `error` if the Replay Buffer is
    /// not active.
    pub async fn save_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::SaveReplayBuffer)
            .await
    }
}
