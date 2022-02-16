use serde::Serialize;
use time::Duration;

use super::Client;
use crate::{requests::RequestType, responses, Error, Result};

/// API functions related to transitions.
pub struct Transitions<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Transitions<'a> {
    /// Gets an array of all available transition kinds.
    pub async fn get_transition_kind_list(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<responses::TransitionKinds>(RequestType::GetTransitionKindList)
            .await
            .map(|tk| tk.transition_kinds)
    }

    /// Gets an array of all scene transitions in OBS.
    pub async fn get_scene_transition_list(&self) -> Result<responses::SceneTransitionList> {
        self.client
            .send_message(RequestType::GetSceneTransitionList)
            .await
    }

    /// Gets information about the current scene transition.
    pub async fn get_current_scene_transition(&self) -> Result<responses::CurrentSceneTransition> {
        self.client
            .send_message(RequestType::GetCurrentSceneTransition)
            .await
    }

    /// Sets the current scene transition.
    ///
    /// **Small note:** While the namespace of scene transitions is generally unique, that
    /// uniqueness is not a guarantee as it is with other resources like inputs.
    ///
    /// - `transition_name`: Name of the transition to make active.
    pub async fn set_current_scene_transition(&self, transition_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentSceneTransition { transition_name })
            .await
    }

    /// Sets the duration of the current scene transition, if it is not fixed.
    ///
    /// - `transition_duration`: Duration in milliseconds.
    pub async fn set_current_scene_transition_duration(
        &self,
        transition_duration: Duration,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentSceneTransitionDuration {
                transition_duration,
            })
            .await
    }

    /// Sets the settings of the current scene transition.
    ///
    /// - `transition_settings`: Settings object to apply to the transition.
    /// - `overlay`: Whether to overlay over the current settings or replace them.
    pub async fn set_current_scene_transition_settings<T>(
        &self,
        transition_settings: T,
        overlay: Option<bool>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetCurrentSceneTransitionSettings {
                transition_settings: serde_json::to_value(&transition_settings)
                    .map_err(Error::SerializeCustomData)?,
                overlay,
            })
            .await
    }

    /// Gets the cursor position of the current scene transition.
    ///
    /// **Note:** `transitionCursor` will return `1.0` when the transition is inactive.
    pub async fn get_current_scene_transition_cursor(&self) -> Result<f32> {
        self.client
            .send_message::<responses::TransitionCursor>(
                RequestType::GetCurrentSceneTransitionCursor,
            )
            .await
            .map(|tc| tc.transition_cursor)
    }

    /// Triggers the current scene transition. Same functionality as the `Transition` button in
    /// studio mode.
    pub async fn trigger_studio_mode_transition(&self) -> Result<()> {
        self.client
            .send_message(RequestType::TriggerStudioModeTransition)
            .await
    }

    /// Sets the position of the TBar.
    ///
    /// **Very important note:** This will be deprecated and replaced in a future version of
    /// `obs-websocket`.
    ///
    /// - `position`: New position.
    /// - `release`: Whether to release the TBar. Only set `false` if you know that you will be
    ///   sending another position update.
    pub async fn set_tbar_position(&self, position: f32, release: Option<bool>) -> Result<()> {
        self.client
            .send_message(RequestType::SetTbarPosition { position, release })
            .await
    }
}
