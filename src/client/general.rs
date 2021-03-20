use serde::Serialize;

use super::Client;
use crate::requests::{KeyModifier, Projector, ProjectorInternal, QtGeometry, RequestType};
use crate::responses;
use crate::{Error, Result};

/// General functions of the API.
pub struct General<'a> {
    pub(super) client: &'a Client,
}

impl<'a> General<'a> {
    /// Returns the latest version of the plugin and the API.
    pub async fn get_version(&self) -> Result<responses::Version> {
        self.client.send_message(RequestType::GetVersion).await
    }

    /// Tells the client if authentication is required. If so, returns authentication parameters
    /// `challenge` and `salt`.
    pub async fn get_auth_required(&self) -> Result<responses::AuthRequired> {
        self.client.send_message(RequestType::GetAuthRequired).await
    }

    /// Attempt to authenticate the client to the server.
    ///
    /// - `auth`: Response to the auth challenge.
    pub async fn authenticate(&self, auth: &str) -> Result<()> {
        self.client
            .send_message(RequestType::Authenticate { auth })
            .await
    }

    /// Set the filename formatting string.
    ///
    /// - `filename_formatting`: Filename formatting string to set.
    pub async fn set_filename_formatting(&self, filename_formatting: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetFilenameFormatting {
                filename_formatting,
            })
            .await
    }

    /// Get the filename formatting string.
    pub async fn get_filename_formatting(&self) -> Result<String> {
        self.client
            .send_message::<responses::FilenameFormatting>(RequestType::GetFilenameFormatting)
            .await
            .map(|ff| ff.filename_formatting)
    }

    /// Get OBS stats (almost the same info as provided in OBS' stats window).
    pub async fn get_stats(&self) -> Result<responses::ObsStats> {
        self.client
            .send_message::<responses::Stats>(RequestType::GetStats)
            .await
            .map(|s| s.stats)
    }

    /// Broadcast custom message to all connected WebSocket clients.
    ///
    /// - `realm`: Identifier to be choosen by the client.
    /// - `data`: User-defined data.
    pub async fn broadcast_custom_message<T>(&self, realm: &str, data: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::BroadcastCustomMessage {
                realm,
                data: &serde_json::to_value(data).map_err(Error::SerializeCustomData)?,
            })
            .await
    }

    /// Get basic OBS video information.
    pub async fn get_video_info(&self) -> Result<responses::VideoInfo> {
        self.client.send_message(RequestType::GetVideoInfo).await
    }

    /// Open a projector window or create a projector on a monitor. Requires OBS v24.0.4 or newer.
    pub async fn open_projector(&self, projector: Projector<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::OpenProjector(ProjectorInternal {
                ty: projector.ty,
                monitor: projector.monitor,
                geometry: projector.geometry.map(QtGeometry::serialize).as_deref(),
                name: projector.name,
            }))
            .await
    }

    /// Executes hotkey routine, identified by hotkey unique name.
    ///
    /// - `hotkey_name`: Unique name of the hotkey, as defined when registering the hotkey (e.g.
    ///   "ReplayBuffer.Save").
    pub async fn trigger_hotkey_by_name(&self, hotkey_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerHotkeyByName { hotkey_name })
            .await
    }

    /// Executes hotkey routine, identified by bound combination of keys. A single key combination
    /// might trigger multiple hotkey routines depending on user settings.
    ///
    /// - `key_id`: Main key identifier (e.g. `OBS_KEY_A` for key "A"). Available identifiers
    ///   [here](https://github.com/obsproject/obs-studio/blob/master/libobs/obs-hotkeys.h)
    /// - `key_modifiers`: Optional key modifiers object. False entries can be ommitted.
    pub async fn trigger_hotkey_by_sequence(
        &self,
        key_id: &str,
        key_modifiers: &[KeyModifier],
    ) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerHotkeyBySequence {
                key_id,
                key_modifiers,
            })
            .await
    }

    // TODO: Add `ExecuteBatch` request
}
