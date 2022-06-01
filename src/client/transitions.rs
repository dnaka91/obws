use serde::Serialize;
use time::Duration;

use super::Client;
use crate::{requests::transitions::Request, responses::transitions as responses, Error, Result};

/// API functions related to transitions.
pub struct Transitions<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Transitions<'a> {
    /// Gets an array of all available transition kinds.
    pub async fn list_kinds(&self) -> Result<Vec<String>> {
        self.client
            .send_message::<_, responses::TransitionKinds>(Request::GetTransitionKindList)
            .await
            .map(|tk| tk.transition_kinds)
    }

    /// Gets an array of all scene transitions in OBS.
    pub async fn list(&self) -> Result<responses::SceneTransitionList> {
        self.client
            .send_message(Request::GetSceneTransitionList)
            .await
    }

    /// Gets information about the current scene transition.
    pub async fn current(&self) -> Result<responses::CurrentSceneTransition> {
        self.client
            .send_message(Request::GetCurrentSceneTransition)
            .await
    }

    /// Sets the current scene transition.
    ///
    /// **Small note:** While the namespace of scene transitions is generally unique, that
    /// uniqueness is not a guarantee as it is with other resources like inputs.
    pub async fn set_current(&self, name: &str) -> Result<()> {
        self.client
            .send_message(Request::SetCurrentSceneTransition { name })
            .await
    }

    /// Sets the duration of the current scene transition, if it is not fixed.
    pub async fn set_current_duration(&self, duration: Duration) -> Result<()> {
        self.client
            .send_message(Request::SetCurrentSceneTransitionDuration { duration })
            .await
    }

    /// Sets the settings of the current scene transition.
    pub async fn set_current_settings<T>(&self, settings: T, overlay: Option<bool>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetCurrentSceneTransitionSettings {
                settings: serde_json::to_value(&settings).map_err(Error::SerializeCustomData)?,
                overlay,
            })
            .await
    }

    /// Gets the cursor position of the current scene transition.
    ///
    /// **Note:** `transitionCursor` will return `1.0` when the transition is inactive.
    pub async fn current_cursor(&self) -> Result<f32> {
        self.client
            .send_message::<_, responses::TransitionCursor>(
                Request::GetCurrentSceneTransitionCursor,
            )
            .await
            .map(|tc| tc.transition_cursor)
    }

    /// Triggers the current scene transition. Same functionality as the `Transition` button in
    /// studio mode.
    pub async fn trigger(&self) -> Result<()> {
        self.client
            .send_message(Request::TriggerStudioModeTransition)
            .await
    }

    /// Sets the position of the T-Bar.
    ///
    /// **Very important note:** This will be deprecated and replaced in a future version of
    /// `obs-websocket`.
    pub async fn set_tbar_position(&self, position: f32, release: Option<bool>) -> Result<()> {
        self.client
            .send_message(Request::SetTbarPosition { position, release })
            .await
    }
}
