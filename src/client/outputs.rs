use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to outputs.
pub struct Outputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Outputs<'a> {
    /// Gets the status of the virtual cam output.
    pub async fn get_virtual_cam_status(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::GetVirtualCamStatus)
            .await
            .map(|oa| oa.output_active)
    }

    /// Toggles the state of the virtual cam output.
    pub async fn toggle_virtual_cam(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::ToggleVirtualCam)
            .await
            .map(|oa| oa.output_active)
    }

    /// Starts the virtual cam output.
    pub async fn start_virtual_cam(&self) -> Result<()> {
        self.client.send_message(RequestType::StartVirtualCam).await
    }

    /// Stops the virtual cam output.
    pub async fn stop_virtual_cam(&self) -> Result<()> {
        self.client.send_message(RequestType::StopVirtualCam).await
    }

    /// Gets the status of the replay buffer output.
    pub async fn get_replay_buffer_status(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::GetReplayBufferStatus)
            .await
            .map(|oa| oa.output_active)
    }

    /// Toggles the state of the replay buffer output.
    pub async fn toggle_replay_buffer(&self) -> Result<bool> {
        self.client
            .send_message::<responses::OutputActive>(RequestType::ToggleReplayBuffer)
            .await
            .map(|oa| oa.output_active)
    }

    /// Starts the replay buffer output.
    pub async fn start_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StartReplayBuffer)
            .await
    }

    /// Stops the replay buffer output.
    pub async fn stop_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StopReplayBuffer)
            .await
    }

    /// Saves the contents of the replay buffer output.
    pub async fn save_replay_buffer(&self) -> Result<()> {
        self.client
            .send_message(RequestType::SaveReplayBuffer)
            .await
    }

    /// Gets the file name of the last replay buffer save file.
    pub async fn get_last_replay_buffer_replay(&self) -> Result<String> {
        self.client
            .send_message::<responses::SavedReplayPath>(RequestType::GetLastReplayBufferReplay)
            .await
            .map(|srp| srp.saved_replay_path)
    }
}
