//! Responses related to scenes.

use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

pub use super::ids::{CurrentPreviewSceneId, CurrentProgramSceneId, SceneId};

/// Response value for [`crate::client::Scenes::list`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Scenes {
    /// Current program scene identifier. Can be [`None`] if internal state desync.
    #[serde(flatten)]
    pub current_program_scene: Option<CurrentProgramSceneId>,
    /// Current preview scene identifier. [`None`] if not in studio mode.
    #[serde(flatten)]
    pub current_preview_scene: Option<CurrentPreviewSceneId>,
    /// Array of scenes in OBS.
    #[serde(rename = "scenes")]
    pub scenes: Vec<Scene>,
}

/// Response value for [`crate::client::Scenes::list`] as part of [`Scenes`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Scene {
    /// Identifier of the scene.
    #[serde(flatten)]
    pub id: SceneId,
    /// Positional index in the list of scenes.
    #[serde(rename = "sceneIndex")]
    pub index: usize,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Groups {
    /// Array of group names.
    #[serde(rename = "groups")]
    pub groups: Vec<String>,
}

/// Response value for [`crate::client::Scenes::current_program_scene`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CurrentProgramScene {
    /// Current program scene identifier.
    #[serde(flatten)]
    pub id: SceneId,
}

/// Response value for [`crate::client::Scenes::current_preview_scene`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CurrentPreviewScene {
    /// Current preview scene identifier.
    #[serde(flatten)]
    pub id: SceneId,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateScene {
    /// UUID of the created scene.
    #[serde(rename = "sceneUuid")]
    pub uuid: Uuid,
}

/// Response value for [`crate::client::Scenes::transition_override`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
