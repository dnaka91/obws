use chrono::Duration;

use super::Client;
use crate::requests::RequestType;
use crate::responses;
use crate::Result;

/// API functions related to transitions.
pub struct Transitions<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Transitions<'a> {
    /// List of all transitions available in the frontend's dropdown menu.
    pub async fn get_transition_list(&self) -> Result<responses::TransitionList> {
        self.client
            .send_message(RequestType::GetTransitionList)
            .await
    }

    /// Get the name of the currently selected transition in the frontend's dropdown menu.
    pub async fn get_current_transition(&self) -> Result<responses::CurrentTransition> {
        self.client
            .send_message(RequestType::GetCurrentTransition)
            .await
    }

    /// Set the active transition.
    ///
    /// - `transition_name`: The name of the transition.
    pub async fn set_current_transition(&self, transition_name: String) -> Result<()> {
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
}
