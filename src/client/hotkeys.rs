use super::Client;
use crate::{
    requests::hotkeys::{KeyModifiers, Request},
    responses, Result,
};

/// API functions related to hotkeys.
pub struct Hotkeys<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Hotkeys<'a> {
    /// Gets an array of all hotkey names in OBS.
    pub async fn list_hotkeys(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<_, responses::Hotkeys>(Request::ListHotkeys)
            .await
            .map(|h| h.hotkeys)
    }

    /// Triggers a hotkey using its name. See [`General::list_hotkeys`].
    pub async fn trigger_hotkey_by_name(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::TriggerHotkeyByName { name })
            .await
    }

    /// Triggers a hotkey using a sequence of keys.
    pub async fn trigger_hotkey_by_key_sequence(
        &self,
        id: &str,
        modifiers: KeyModifiers,
    ) -> Result<()> {
        self.client
            .send_message(Request::TriggerHotkeyByKeySequence { id, modifiers })
            .await
    }
}
