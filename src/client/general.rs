use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{
    requests::{CallVendorRequest, CallVendorRequestInternal, KeyModifiers, RequestType},
    responses, Error, Result,
};

/// General functions of the API.
pub struct General<'a> {
    pub(super) client: &'a Client,
}

impl<'a> General<'a> {
    /// Gets data about the current plugin and RPC version.
    pub async fn version(&self) -> Result<responses::Version> {
        self.client.send_message(RequestType::GetVersion).await
    }

    /// Gets statistics about OBS, obs-websocket, and the current session.
    pub async fn stats(&self) -> Result<responses::Stats> {
        self.client.send_message(RequestType::GetStats).await
    }

    /// Broadcasts a custom event to all web-socket clients. Receivers are clients which are
    /// identified and subscribed.
    pub async fn broadcast_custom_event<T>(&self, event_data: &T) -> Result<()>
    where
        T: Serialize,
    {
        let event_data = serde_json::to_value(event_data).map_err(Error::SerializeCustomData)?;
        if !event_data.is_object() {
            return Err(Error::InvalidCustomData);
        }

        self.client
            .send_message(RequestType::BroadcastCustomEvent { event_data })
            .await
    }

    /// Call a request registered to a vendor.
    ///
    /// A vendor is a unique name registered by a third-party plugin or script, which allows for
    /// custom requests and events to be added to obs-websocket. If a plugin or script implements
    /// vendor requests or events, documentation is expected to be provided with them.
    pub async fn call_vendor_request<T, R>(&self, request: CallVendorRequest<'_, T>) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.client
            .send_message::<responses::CallVendorResponse<R>>(RequestType::CallVendorRequest(
                CallVendorRequestInternal {
                    vendor_name: request.vendor_name,
                    request_type: request.request_type,
                    request_data: serde_json::to_value(&request.request_data)
                        .map_err(Error::SerializeCustomData)?,
                },
            ))
            .await
            .map(|cvr| cvr.response_data)
    }

    /// Gets an array of all hotkey names in OBS.
    pub async fn list_hotkeys(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<responses::Hotkeys>(RequestType::GetHotkeyList)
            .await
            .map(|h| h.hotkeys)
    }

    /// Triggers a hotkey using its name. See [`General::list_hotkeys`].
    pub async fn trigger_hotkey_by_name(&self, name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerHotkeyByName { name })
            .await
    }

    /// Triggers a hotkey using a sequence of keys.
    pub async fn trigger_hotkey_by_key_sequence(
        &self,
        id: &str,
        modifiers: KeyModifiers,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerHotkeyByKeySequence { id, modifiers })
            .await
    }
}
