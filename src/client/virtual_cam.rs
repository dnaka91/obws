use super::Client;
use crate::{requests::virtual_cam::Request, responses::virtual_cam as responses, Result};

/// API functions related to the virtual camera.
pub struct VirtualCam<'a> {
    pub(super) client: &'a Client,
}

impl<'a> VirtualCam<'a> {
    /// Gets the status of the virtual cam output.
    #[doc(alias = "GetVirtualCamStatus")]
    pub async fn status(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Status)
            .await
            .map(|oa| oa.active)
    }

    /// Toggles the state of the virtual cam output.
    #[doc(alias = "ToggleVirtualCam")]
    pub async fn toggle(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Toggle)
            .await
            .map(|oa| oa.active)
    }

    /// Starts the virtual cam output.
    #[doc(alias = "StartVirtualCam")]
    pub async fn start(&self) -> Result<()> {
        self.client.send_message(Request::Start).await
    }

    /// Stops the virtual cam output.
    #[doc(alias = "StopVirtualCam")]
    pub async fn stop(&self) -> Result<()> {
        self.client.send_message(Request::Stop).await
    }
}
