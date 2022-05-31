//! Requests related to outputs.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request {
    #[serde(rename = "GetVirtualCamStatus")]
    VirtualCamStatus,
    #[serde(rename = "ToggleVirtualCam")]
    ToggleVirtualCam,
    #[serde(rename = "StartVirtualCam")]
    StartVirtualCam,
    #[serde(rename = "StopVirtualCam")]
    StopVirtualCam,
    #[serde(rename = "GetReplayBufferStatus")]
    ReplayBufferStatus,
    #[serde(rename = "ToggleReplayBuffer")]
    ToggleReplayBuffer,
    #[serde(rename = "StartReplayBuffer")]
    StartReplayBuffer,
    #[serde(rename = "StopReplayBuffer")]
    StopReplayBuffer,
    #[serde(rename = "SaveReplayBuffer")]
    SaveReplayBuffer,
    #[serde(rename = "GetLastReplayBufferReplay")]
    LastReplayBufferReplay,
}

impl<'a> From<Request> for super::RequestType<'a> {
    fn from(value: Request) -> Self {
        super::RequestType::Outputs(value)
    }
}
