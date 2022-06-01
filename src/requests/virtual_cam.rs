//! Requests related to the virtual camera.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request {
    #[serde(rename = "GetVirtualCamStatus")]
    Status,
    #[serde(rename = "ToggleVirtualCam")]
    Toggle,
    #[serde(rename = "StartVirtualCam")]
    Start,
    #[serde(rename = "StopVirtualCam")]
    Stop,
}

impl<'a> From<Request> for super::RequestType<'a> {
    fn from(value: Request) -> Self {
        super::RequestType::VirtualCam(value)
    }
}
