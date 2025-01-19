use super::Client;
use crate::{
    error::Result, requests::replay_buffer::Request, responses::replay_buffer as responses,
};

/// API functions related to the replay buffer.
pub struct ReplayBuffer<'a> {
    pub(super) client: &'a Client,
}

impl ReplayBuffer<'_> {
    /// Gets the status of the replay buffer output.
    #[doc(alias = "GetReplayBufferStatus")]
    pub async fn status(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Status)
            .await
            .map(|oa| oa.active)
    }

    /// Toggles the state of the replay buffer output.
    #[doc(alias = "ToggleReplayBuffer")]
    pub async fn toggle(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Toggle)
            .await
            .map(|oa| oa.active)
    }

    /// Starts the replay buffer output.
    #[doc(alias = "StartReplayBuffer")]
    pub async fn start(&self) -> Result<()> {
        self.client.send_message(Request::Start).await
    }

    /// Stops the replay buffer output.
    #[doc(alias = "StopReplayBuffer")]
    pub async fn stop(&self) -> Result<()> {
        self.client.send_message(Request::Stop).await
    }

    /// Saves the contents of the replay buffer output.
    #[doc(alias = "SaveReplayBuffer")]
    pub async fn save(&self) -> Result<()> {
        self.client.send_message(Request::Save).await
    }

    /// Gets the file name of the last replay buffer save file.
    #[doc(alias = "GetLastReplayBufferReplay")]
    pub async fn last_replay(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::SavedReplayPath>(Request::LastReplay)
            .await
            .map(|srp| srp.saved_replay_path)
    }
}
