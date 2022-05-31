use super::Client;
use crate::{requests::outputs::Request, responses, Result};

/// API functions related to outputs.
pub struct Outputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Outputs<'a> {
    /// Gets the status of the virtual cam output.
    pub async fn virtual_cam_status(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::VirtualCamStatus)
            .await
            .map(|oa| oa.active)
    }

    /// Toggles the state of the virtual cam output.
    pub async fn toggle_virtual_cam(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::ToggleVirtualCam)
            .await
            .map(|oa| oa.active)
    }

    /// Starts the virtual cam output.
    pub async fn start_virtual_cam(&self) -> Result<()> {
        self.client.send_message(Request::StartVirtualCam).await
    }

    /// Stops the virtual cam output.
    pub async fn stop_virtual_cam(&self) -> Result<()> {
        self.client.send_message(Request::StopVirtualCam).await
    }

    /// Gets the status of the replay buffer output.
    pub async fn replay_buffer_status(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::ReplayBufferStatus)
            .await
            .map(|oa| oa.active)
    }

    /// Toggles the state of the replay buffer output.
    pub async fn toggle_replay_buffer(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::ToggleReplayBuffer)
            .await
            .map(|oa| oa.active)
    }

    /// Starts the replay buffer output.
    pub async fn start_replay_buffer(&self) -> Result<()> {
        self.client.send_message(Request::StartReplayBuffer).await
    }

    /// Stops the replay buffer output.
    pub async fn stop_replay_buffer(&self) -> Result<()> {
        self.client.send_message(Request::StopReplayBuffer).await
    }

    /// Saves the contents of the replay buffer output.
    pub async fn save_replay_buffer(&self) -> Result<()> {
        self.client.send_message(Request::SaveReplayBuffer).await
    }

    /// Gets the file name of the last replay buffer save file.
    pub async fn last_replay_buffer_replay(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::SavedReplayPath>(Request::LastReplayBufferReplay)
            .await
            .map(|srp| srp.saved_replay_path)
    }
}
