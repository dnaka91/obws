use serde::{Serialize, de::DeserializeOwned};

use super::Client;
use crate::{
    error::{Error, Result},
    requests::general::{CallVendorRequest, CallVendorRequestInternal, Request},
    responses::general as responses,
};

/// General functions of the API.
pub struct General<'a> {
    pub(super) client: &'a Client,
}

impl General<'_> {
    /// Gets data about the current plugin and RPC version.
    #[doc(alias = "GetVersion")]
    pub async fn version(&self) -> Result<responses::Version> {
        self.client.send_message(Request::Version).await
    }

    /// Gets statistics about OBS, obs-websocket, and the current session.
    #[doc(alias = "GetStats")]
    pub async fn stats(&self) -> Result<responses::Stats> {
        self.client.send_message(Request::Stats).await
    }

    /// Broadcasts a custom event to all web-socket clients. Receivers are clients which are
    /// identified and subscribed.
    #[doc(alias = "BroadcastCustomEvent")]
    pub async fn broadcast_custom_event<T>(&self, event_data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let event_data =
            serde_json::to_value(event_data).map_err(crate::error::SerializeCustomDataError)?;
        if !event_data.is_object() {
            return Err(Error::InvalidCustomData);
        }

        self.client
            .send_message(Request::BroadcastCustomEvent { event_data })
            .await
    }

    /// Call a request registered to a vendor.
    ///
    /// A vendor is a unique name registered by a third-party plugin or script, which allows for
    /// custom requests and events to be added to obs-websocket. If a plugin or script implements
    /// vendor requests or events, documentation is expected to be provided with them.
    #[doc(alias = "CallVendorRequest")]
    pub async fn call_vendor_request<T, R>(
        &self,
        request: CallVendorRequest<'_, T>,
    ) -> Result<responses::VendorResponse<R>>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.client
            .send_message(Request::CallVendorRequest(CallVendorRequestInternal {
                vendor_name: request.vendor_name,
                request_type: request.request_type,
                request_data: serde_json::to_value(request.request_data)
                    .map_err(crate::error::SerializeCustomDataError)?,
            }))
            .await
    }
}
