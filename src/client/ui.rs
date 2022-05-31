use super::Client;
use crate::{requests::ui::Request, responses, Result};

/// API functions related to the user interface.
pub struct Ui<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Ui<'a> {
    /// Gets whether studio is enabled.
    pub async fn studio_mode_enabled(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::StudioModeEnabled>(Request::GetStudioModeEnabled)
            .await
            .map(|sme| sme.enabled)
    }

    /// Enables or disables studio mode.
    ///
    /// - `enabled`: Enable or disable the studio mode.
    pub async fn set_studio_mode_enabled(&self, enabled: bool) -> Result<()> {
        self.client
            .send_message(Request::SetStudioModeEnabled { enabled })
            .await
    }

    /// Opens the properties dialog of an input.
    pub async fn open_properties_dialog(&self, input: &str) -> Result<()> {
        self.client
            .send_message(Request::OpenInputPropertiesDialog { input })
            .await
    }

    /// Opens the filters dialog of an input.
    pub async fn open_filters_dialog(&self, input: &str) -> Result<()> {
        self.client
            .send_message(Request::OpenInputFiltersDialog { input })
            .await
    }

    /// Opens the interact dialog of an input.
    pub async fn open_interact_dialog(&self, input: &str) -> Result<()> {
        self.client
            .send_message(Request::OpenInputInteractDialog { input })
            .await
    }

    /// Gets a list of connected monitors and information about them.
    pub async fn list_monitors(&self) -> Result<Vec<responses::Monitor>> {
        self.client
            .send_message::<_, responses::MonitorList>(Request::GetMonitorList)
            .await
            .map(|ml| ml.monitors)
    }
}
