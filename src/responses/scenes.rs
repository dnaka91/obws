//! Responses related to scenes.

use serde::Deserialize;
use time::Duration;

/// Response value for [`crate::client::Scenes::list`].
#[derive(Debug, Deserialize)]
pub struct Scenes {
    /// Current program scene.
    #[serde(rename = "currentProgramSceneName")]
    pub current_program_scene_name: Option<String>,
    /// Current preview scene. [`None`] if not in studio mode.
    #[serde(rename = "currentPreviewSceneName")]
    pub current_preview_scene_name: Option<String>,
    /// Array of scenes in OBS.
    #[serde(rename = "scenes")]
    pub scenes: Vec<Scene>,
}

/// Response value for [`crate::client::Scenes::list`] as part of [`Scenes`].
#[derive(Debug, Deserialize)]
pub struct Scene {
    /// Name of the scene.
    #[serde(rename = "sceneName")]
    pub name: String,
    /// Positional index in the list of scenes.
    #[serde(rename = "sceneIndex")]
    pub index: usize,
}

/// Response value for [`crate::client::Scenes::get_group_list`].
#[derive(Debug, Deserialize)]
pub(crate) struct Groups {
    /// Array of group names.
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
}

/// Response value for
/// [`crate::client::Scenes::get_current_program_scene`].
#[derive(Debug, Deserialize)]
pub(crate) struct CurrentProgramScene {
    /// Current program scene.
    #[serde(rename = "currentProgramSceneName")]
    pub current_program_scene_name: String,
}

/// Response value for
/// [`crate::client::Scenes::get_current_preview_scene`].
#[derive(Debug, Deserialize)]
pub(crate) struct CurrentPreviewScene {
    /// Current preview scene.
    #[serde(rename = "currentPreviewSceneName")]
    pub current_preview_scene_name: String,
}

/// Response value for [`crate::client::Scenes::transition_override`].
#[derive(Debug, Deserialize)]
pub struct SceneTransitionOverride {
    /// Name of the overridden scene transition.
    #[serde(rename = "transitionName")]
    pub name: Option<String>,
    /// Duration of the overridden scene transition.
    #[serde(
        rename = "transitionDuration",
        with = "crate::serde::duration_millis::option"
    )]
    pub duration: Option<Duration>,
}
