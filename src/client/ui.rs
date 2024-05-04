use super::Client;
use crate::{
    requests::{
        inputs::InputId,
        ui::{
            OpenSourceProjector, OpenSourceProjectorInternal, OpenVideoMixProjector,
            OpenVideoMixProjectorInternal, Request,
        },
    },
    responses::ui as responses,
    Result,
};

/// API functions related to the user interface.
pub struct Ui<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Ui<'a> {
    /// Gets whether studio is enabled.
    #[doc(alias = "GetStudioModeEnabled")]
    pub async fn studio_mode_enabled(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::StudioModeEnabled>(Request::GetStudioModeEnabled)
            .await
            .map(|sme| sme.enabled)
    }

    /// Enables or disables studio mode.
    ///
    /// - `enabled`: Enable or disable the studio mode.
    #[doc(alias = "SetStudioModeEnabled")]
    pub async fn set_studio_mode_enabled(&self, enabled: bool) -> Result<()> {
        self.client
            .send_message(Request::SetStudioModeEnabled { enabled })
            .await
    }

    /// Opens the properties dialog of an input.
    #[doc(alias = "OpenInputPropertiesDialog")]
    pub async fn open_properties_dialog(&self, input: InputId<'_>) -> Result<()> {
        self.client
            .send_message(Request::OpenInputPropertiesDialog { input })
            .await
    }

    /// Opens the filters dialog of an input.
    #[doc(alias = "OpenInputFiltersDialog")]
    pub async fn open_filters_dialog(&self, input: InputId<'_>) -> Result<()> {
        self.client
            .send_message(Request::OpenInputFiltersDialog { input })
            .await
    }

    /// Opens the interact dialog of an input.
    #[doc(alias = "OpenInputInteractDialog")]
    pub async fn open_interact_dialog(&self, input: InputId<'_>) -> Result<()> {
        self.client
            .send_message(Request::OpenInputInteractDialog { input })
            .await
    }

    /// Gets a list of connected monitors and information about them.
    #[doc(alias = "GetMonitorList")]
    pub async fn list_monitors(&self) -> Result<Vec<responses::Monitor>> {
        self.client
            .send_message::<_, responses::MonitorList>(Request::GetMonitorList)
            .await
            .map(|ml| ml.monitors)
    }

    /// Open a projector for a specific output video mix.
    #[doc(alias = "OpenVideoMixProjector")]
    pub async fn open_video_mix_projector(&self, open: OpenVideoMixProjector) -> Result<()> {
        self.client
            .send_message(Request::OpenVideoMixProjector(
                OpenVideoMixProjectorInternal {
                    r#type: open.r#type,
                    location: open.location.map(Into::into),
                },
            ))
            .await
    }

    /// Opens a projector for a source.
    #[doc(alias = "OpenSourceProjector")]
    pub async fn open_source_projector(&self, open: OpenSourceProjector<'a>) -> Result<()> {
        self.client
            .send_message(Request::OpenSourceProjector(OpenSourceProjectorInternal {
                source: open.source,
                location: open.location.map(Into::into),
            }))
            .await
    }
}
