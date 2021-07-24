use serde::Serialize;

use super::Client;
use crate::{
    requests::{KeyModifiers, RequestType},
    responses, Error, Result,
};

/// General functions of the API.
pub struct General<'a> {
    pub(super) client: &'a Client,
}

impl<'a> General<'a> {
    pub async fn get_version(&self) -> Result<responses::Version> {
        self.client.send_message(RequestType::GetVersion).await
    }

    pub async fn broadcast_custom_event<T>(&self, event_data: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::BroadcastCustomEvent {
                event_data: serde_json::to_value(event_data).map_err(Error::SerializeCustomData)?,
            })
            .await
    }

    pub async fn get_hotkey_list(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<responses::Hotkeys>(RequestType::GetHotkeyList)
            .await
            .map(|h| h.hotkeys)
    }

    pub async fn trigger_hotkey_by_name(&self, hotkey_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerHotkeyByName { hotkey_name })
            .await
    }

    pub async fn trigger_hotkey_by_key_sequence(
        &self,
        key_id: &str,
        key_modifiers: KeyModifiers,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerHotkeyByKeySequence {
                key_id,
                key_modifiers,
            })
            .await
    }

    pub async fn get_studio_mode_enabled(&self) -> Result<bool> {
        self.client
            .send_message::<responses::StudioModeEnabled>(RequestType::GetStudioModeEnabled)
            .await
            .map(|sme| sme.studio_mode_enabled)
    }

    pub async fn set_studio_mode_enabled(&self, studio_mode_enabled: bool) -> Result<()> {
        self.client
            .send_message(RequestType::SetStudioModeEnabled {
                studio_mode_enabled,
            })
            .await
    }
}
