use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to the virtual cam.
pub struct VirtualCam<'a> {
    pub(super) client: &'a Client,
}

impl<'a> VirtualCam<'a> {
    /// Get current virtual cam status.
    pub async fn get_virtual_cam_status(&self) -> Result<responses::VirtualCamStatus> {
        self.client
            .send_message(RequestType::GetVirtualCamStatus)
            .await
    }

    /// Toggle virtual cam on or off (depending on the current virtual cam state).
    pub async fn start_stop_virtual_cam(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StartStopVirtualCam)
            .await
    }

    /// Start virtual cam. Will return an `error` if virtual cam is already active.
    pub async fn start_virtual_cam(&self) -> Result<()> {
        self.client.send_message(RequestType::StartVirtualCam).await
    }

    /// Stop virtual cam. Will return an error if virtual cam is not active.
    pub async fn stop_virtual_cam(&self) -> Result<()> {
        self.client.send_message(RequestType::StopVirtualCam).await
    }
}
