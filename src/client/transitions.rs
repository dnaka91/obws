use chrono::Duration;
use serde::Serialize;

use super::Client;
use crate::{requests::RequestType, responses, Error, Result};

/// API functions related to transitions.
pub struct Transitions<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Transitions<'a> {
    /// List of all transitions available in the front-end's drop-down menu.
    pub async fn get_transition_list(&self) -> Result<responses::TransitionList> {
        self.client
            .send_message(RequestType::GetTransitionList)
            .await
    }

    /// Get the name of the currently selected transition in the front-end's drop-down menu.
    pub async fn get_current_transition(&self) -> Result<responses::CurrentTransition> {
        self.client
            .send_message(RequestType::GetCurrentTransition)
            .await
    }

    /// Set the active transition.
    ///
    /// - `transition_name`: The name of the transition.
    pub async fn set_current_transition(&self, transition_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentTransition { transition_name })
            .await
    }

    /// Set the duration of the currently selected transition if supported.
    ///
    /// - `duration`: Desired duration of the transition (in milliseconds).
    pub async fn set_transition_duration(&self, duration: Duration) -> Result<()> {
        self.client
            .send_message(RequestType::SetTransitionDuration { duration })
            .await
    }

    /// Get the duration of the currently selected transition if supported.
    pub async fn get_transition_duration(&self) -> Result<Duration> {
        self.client
            .send_message::<responses::TransitionDuration>(RequestType::GetTransitionDuration)
            .await
            .map(|td| td.transition_duration)
    }

    /// Get the position of the current transition.
    pub async fn get_transition_position(&self) -> Result<f64> {
        self.client
            .send_message::<responses::TransitionPosition>(RequestType::GetTransitionPosition)
            .await
            .map(|tp| tp.position)
    }

    /// Get the current settings of a transition.
    ///
    /// - `transition_name`: Transition name.
    pub async fn get_transition_settings(
        &self,
        transition_name: &str,
    ) -> Result<serde_json::Value> {
        self.client
            .send_message::<responses::TransitionSettings>(RequestType::GetTransitionSettings {
                transition_name,
            })
            .await
            .map(|ts| ts.transition_settings)
    }

    /// Change the current settings of a transition.
    ///
    /// - `transition_name`: Transition name.
    /// - `transition_settings`: Transition settings (they can be partial)
    pub async fn set_transition_settings<T>(
        &self,
        transition_name: &str,
        transition_settings: &T,
    ) -> Result<serde_json::Value>
    where
        T: Serialize,
    {
        self.client
            .send_message::<responses::TransitionSettings>(RequestType::SetTransitionSettings {
                transition_name,
                transition_settings: &serde_json::to_value(transition_settings)
                    .map_err(Error::SerializeCustomData)?,
            })
            .await
            .map(|ts| ts.transition_settings)
    }

    /// Release the T-Bar (like a user releasing their mouse button after moving it). *YOU MUST CALL
    /// THIS if you called [`set_t_bar_position`](Self::set_t_bar_position) with the `release`
    /// parameter set to `false`.*
    pub async fn release_t_bar(&self) -> Result<()> {
        self.client.send_message(RequestType::ReleaseTBar).await
    }

    /// If your code needs to perform multiple successive T-Bar moves (e.g. : in an animation, or in
    /// response to a user moving a T-Bar control in your User Interface), set `release` to false
    /// and call [`release_t_bar`](Self::release_t_bar) later once the animation/interaction is
    /// over.
    ///
    /// - `position`: T-Bar position. This value must be between 0.0 and 1.0.
    /// - `release`: Whether the T-Bar gets released automatically after setting its new position
    ///   (like a user releasing their mouse button after moving the T-Bar). Call
    ///   [`release_t_bar`](Self::release_t_bar) manually if you set `release` to false. Defaults to
    ///   true.
    pub async fn set_t_bar_position(&self, position: f64, release: Option<bool>) -> Result<()> {
        self.client
            .send_message(RequestType::SetTBarPosition { position, release })
            .await
    }
}
