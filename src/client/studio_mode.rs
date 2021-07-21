use super::Client;
use crate::{
    requests::{RequestType, Transition},
    responses, Result,
};

/// API functions related to the studio mode.
pub struct StudioMode<'a> {
    pub(super) client: &'a Client,
}

impl<'a> StudioMode<'a> {
    /// Indicates if Studio Mode is currently enabled.
    pub async fn get_studio_mode_status(&self) -> Result<bool> {
        self.client
            .send_message::<responses::StudioModeStatus>(RequestType::GetStudioModeStatus)
            .await
            .map(|sms| sms.studio_mode)
    }

    /// Get the name of the currently previewed scene and its list of sources. Will return an
    /// `error` if Studio Mode is not enabled.
    pub async fn get_preview_scene(&self) -> Result<responses::PreviewScene> {
        self.client.send_message(RequestType::GetPreviewScene).await
    }

    /// Set the active preview scene. Will return an `error` if Studio Mode is not enabled.
    ///
    /// - `scene_name`: The name of the scene to preview.
    pub async fn set_preview_scene(&self, scene_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetPreviewScene { scene_name })
            .await
    }

    /// Transitions the currently previewed scene to the main output. Will return an `error` if
    /// Studio Mode is not enabled.
    ///
    /// - `with_transition`: Change the active transition before switching scenes. Defaults to the
    ///   active transition.
    pub async fn transition_to_program(
        &self,
        with_transition: Option<Transition<'_>>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::TransitionToProgram { with_transition })
            .await
    }

    /// Enables Studio Mode.
    pub async fn enable_studio_mode(&self) -> Result<()> {
        self.client
            .send_message(RequestType::EnableStudioMode)
            .await
    }

    /// Disables Studio Mode.
    pub async fn disable_studio_mode(&self) -> Result<()> {
        self.client
            .send_message(RequestType::DisableStudioMode)
            .await
    }

    /// Toggles Studio Mode (depending on the current state of studio mode).
    pub async fn toggle_studio_mode(&self) -> Result<()> {
        self.client
            .send_message(RequestType::ToggleStudioMode)
            .await
    }
}
