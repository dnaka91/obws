//! Requests related to recording.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request {
    #[serde(rename = "GetRecordStatus")]
    Status,
    #[serde(rename = "ToggleRecord")]
    Toggle,
    #[serde(rename = "StartRecord")]
    Start,
    #[serde(rename = "StopRecord")]
    Stop,
    #[serde(rename = "ToggleRecordPause")]
    TogglePause,
    #[serde(rename = "PauseRecord")]
    Pause,
    #[serde(rename = "ResumeRecord")]
    Resume,
}

impl From<Request> for super::RequestType<'_> {
    fn from(value: Request) -> Self {
        super::RequestType::Recording(value)
    }
}
