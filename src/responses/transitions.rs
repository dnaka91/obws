//! Responses related to transitions.

use serde::{Deserialize, Serialize};
use time::Duration;

pub use super::ids::{CurrentSceneTransitionId, TransitionId};

/// Response value for [`crate::client::Transitions::list_kinds`].
#[derive(Debug, Deserialize)]
pub(crate) struct TransitionKinds {
    /// Array of transition kinds.
    #[serde(rename = "transitionKinds")]
    pub transition_kinds: Vec<String>,
}

/// Response value for [`crate::client::Transitions::list`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SceneTransitionList {
    /// Identifier of the current scene transition.
    #[serde(flatten)]
    pub current_scene_transition: Option<CurrentSceneTransitionId>,
    /// Kind of the current scene transition.
    #[serde(rename = "currentSceneTransitionKind")]
    pub current_scene_transition_kind: Option<String>,
    /// Array of transitions.
    #[serde(rename = "transitions")]
    pub transitions: Vec<Transition>,
}

/// Response value for [`crate::client::Transitions::list`] as part of [`SceneTransitionList`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Transition {
    /// Identifier of the transition.
    #[serde(flatten)]
    pub id: TransitionId,
    /// Kind of the transition.
    #[serde(rename = "transitionKind")]
    pub kind: String,
    /// Whether the transition uses a fixed (non-configurable) duration.
    #[serde(rename = "transitionFixed")]
    pub fixed: bool,
    /// Whether the transition supports being configured.
    #[serde(rename = "transitionConfigurable")]
    pub configurable: bool,
}

/// Response value for [`crate::client::Transitions::current`].
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentSceneTransition {
    /// Identifier of the transition.
    #[serde(flatten)]
    pub id: TransitionId,
    /// Kind of the transition.
    #[serde(rename = "transitionKind")]
    pub kind: String,
    /// Whether the transition uses a fixed (non-configurable) duration.
    #[serde(rename = "transitionFixed")]
    pub fixed: bool,
    /// Configured transition duration in milliseconds.
    #[serde(
        rename = "transitionDuration",
        with = "crate::serde::duration_millis::option"
    )]
    pub duration: Option<Duration>,
    /// Whether the transition supports being configured.
    #[serde(rename = "transitionConfigurable")]
    pub configurable: bool,
    /// Object of settings for the transition.
    #[serde(rename = "transitionSettings")]
    pub settings: Option<serde_json::Value>,
}

/// Response value for [`crate::client::Transitions::current_cursor`].
#[derive(Debug, Deserialize)]
pub(crate) struct TransitionCursor {
    /// Cursor position, between `0.0` and `1.0`.
    #[serde(rename = "transitionCursor")]
    pub transition_cursor: f32,
}
