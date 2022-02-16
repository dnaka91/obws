use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to the user interface.
pub struct Ui<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Ui<'a> {
    /// Gets whether studio is enabled.
    pub async fn get_studio_mode_enabled(&self) -> Result<bool> {
        self.client
            .send_message::<responses::StudioModeEnabled>(RequestType::GetStudioModeEnabled)
            .await
            .map(|sme| sme.studio_mode_enabled)
    }

    /// Enables or disables studio mode.
    ///
    /// - `studio_mode_enabled`: Enable or disable the studio mode.
    pub async fn set_studio_mode_enabled(&self, studio_mode_enabled: bool) -> Result<()> {
        self.client
            .send_message(RequestType::SetStudioModeEnabled {
                studio_mode_enabled,
            })
            .await
    }

    /// Opens the properties dialog of an input.
    ///
    /// - `input_name`: Name of the input to open the dialog of.
    pub async fn open_input_properties_dialog(&self, input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::OpenInputPropertiesDialog { input_name })
            .await
    }

    /// Opens the filters dialog of an input.
    ///
    /// - `input_name`: Name of the input to open the dialog of.
    pub async fn open_input_filters_dialog(&self, input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::OpenInputFiltersDialog { input_name })
            .await
    }

    /// Opens the interact dialog of an input.
    ///
    /// - `input_name`: Name of the input to open the dialog of.
    pub async fn open_input_interact_dialog(&self, input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::OpenInputInteractDialog { input_name })
            .await
    }
}
