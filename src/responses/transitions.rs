//! Responses related to transitions.

use serde::Deserialize;
use time::Duration;

/// Response value for
/// [`crate::client::Transitions::get_transition_kind_list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransitionKinds {
    /// Array of transition kinds.
    pub transition_kinds: Vec<String>,
}

/// Response value for [`crate::client::Transitions::list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneTransitionList {
    /// Name of the current scene transition.
    pub current_scene_transition_name: Option<String>,
    /// Kind of the current scene transition.
    pub current_scene_transition_kind: Option<String>,
    /// Array of transitions.
    pub transitions: Vec<Transition>,
}

/// Response value for [`crate::client::Transitions::list`] as part of [`SceneTransitionList`].
#[derive(Debug, Deserialize)]
pub struct Transition {
    /// Name of the transition.
    #[serde(rename = "transitionName")]
    pub name: String,
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
#[derive(Debug, Deserialize)]
pub struct CurrentSceneTransition {
    /// Name of the transition.
    #[serde(rename = "transitionName")]
    pub name: String,
    /// Kind of the transition.
    #[serde(rename = "transitionKind")]
    pub kind: String,
    /// Whether the transition uses a fixed (non-configurable) duration.
    #[serde(rename = "transitionFixed")]
    pub fixed: bool,
    /// Configured transition duration in milliseconds.
    #[serde(
        rename = "transitionDuration",
        deserialize_with = "crate::de::duration_millis_opt"
    )]
    pub duration: Option<Duration>,
    /// Whether the transition supports being configured.
    #[serde(rename = "transitionConfigurable")]
    pub configurable: bool,
    /// Object of settings for the transition.
    #[serde(rename = "transitionSettings")]
    pub settings: Option<serde_json::Value>,
}

/// Response value for
/// [`crate::client::Transitions::get_current_scene_transition_cursor`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransitionCursor {
    /// Cursor position, between `0.0` and `1.0`.
    pub transition_cursor: f32,
}
