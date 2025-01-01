//! Requests related to the replay buffer.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request {
    #[serde(rename = "GetReplayBufferStatus")]
    Status,
    #[serde(rename = "ToggleReplayBuffer")]
    Toggle,
    #[serde(rename = "StartReplayBuffer")]
    Start,
    #[serde(rename = "StopReplayBuffer")]
    Stop,
    #[serde(rename = "SaveReplayBuffer")]
    Save,
    #[serde(rename = "GetLastReplayBufferReplay")]
    LastReplay,
}

impl From<Request> for super::RequestType<'_> {
    fn from(value: Request) -> Self {
        super::RequestType::ReplayBuffer(value)
    }
}
