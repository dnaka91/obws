//! Requests related to transitions.

use serde::Serialize;
use time::Duration;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetTransitionKindList")]
    GetTransitionKindList,
    #[serde(rename = "GetSceneTransitionList")]
    GetSceneTransitionList,
    #[serde(rename = "GetCurrentSceneTransition")]
    GetCurrentSceneTransition,
    #[serde(rename = "SetCurrentSceneTransition")]
    SetCurrentSceneTransition {
        /// Name of the transition to make active.
        #[serde(rename = "transitionName")]
        name: &'a str,
    },
    #[serde(rename = "SetCurrentSceneTransitionDuration")]
    SetCurrentSceneTransitionDuration {
        /// Duration in milliseconds.
        #[serde(
            rename = "transitionDuration",
            serialize_with = "super::ser::duration_millis"
        )]
        duration: Duration,
    },
    #[serde(rename = "SetCurrentSceneTransitionSettings")]
    SetCurrentSceneTransitionSettings {
        /// Settings object to apply to the transition.
        #[serde(rename = "transitionSettings")]
        settings: serde_json::Value,
        /// Whether to overlay over the current settings or replace them.
        #[serde(rename = "overlay", skip_serializing_if = "Option::is_none")]
        overlay: Option<bool>,
    },
    #[serde(rename = "GetCurrentSceneTransitionCursor")]
    GetCurrentSceneTransitionCursor,
    #[serde(rename = "TriggerStudioModeTransition")]
    TriggerStudioModeTransition,
    #[serde(rename = "SetTBarPosition")]
    SetTbarPosition {
        /// New position.
        #[serde(rename = "position")]
        position: f32,
        /// Whether to release the T-Bar. Only set `false` if you know that you will be sending
        /// another position update.
        #[serde(rename = "release", skip_serializing_if = "Option::is_none")]
        release: Option<bool>,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Transitions(value)
    }
}
