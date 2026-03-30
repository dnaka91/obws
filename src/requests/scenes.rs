//! Requests related to scenes.

use serde::Serialize;
use serde_with::skip_serializing_none;
use time::Duration;
use uuid::Uuid;

pub use super::ids::SceneId;

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetSceneList")]
    List,
    #[serde(rename = "GetGroupList")]
    ListGroups,
    #[serde(rename = "GetCurrentProgramScene")]
    CurrentProgramScene,
    #[serde(rename = "SetCurrentProgramScene")]
    SetCurrentProgramScene {
        /// Scene to set as the current program scene.
        #[serde(flatten)]
        scene: SceneId<'a>,
    },
    #[serde(rename = "GetCurrentPreviewScene")]
    CurrentPreviewScene,
    #[serde(rename = "SetCurrentPreviewScene")]
    SetCurrentPreviewScene {
        /// Scene to set as the current preview scene.
        #[serde(flatten)]
        scene: SceneId<'a>,
    },
    #[serde(rename = "SetSceneName")]
    SetName(SetName<'a>),
    #[serde(rename = "CreateScene")]
    Create {
        /// UUID of the canvas to create the new scene in. Leave [`None`] to assume main canvas.
        #[serde(rename = "canvasUuid")]
        canvas: Option<Uuid>,
        /// Name for the new scene.
        #[serde(rename = "sceneName")]
        name: &'a str,
    },
    #[serde(rename = "RemoveScene")]
    Remove {
        /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
        #[serde(rename = "canvasUuid")]
        canvas: Option<Uuid>,
        /// The scene to remove.
        #[serde(flatten)]
        scene: SceneId<'a>,
    },
    #[serde(rename = "GetSceneSceneTransitionOverride")]
    TransitionOverride {
        /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
        #[serde(rename = "canvasUuid")]
        canvas: Option<Uuid>,
        /// Identifier of the scene.
        #[serde(flatten)]
        scene: SceneId<'a>,
    },
    #[serde(rename = "SetSceneSceneTransitionOverride")]
    SetTransitionOverride(SetTransitionOverride<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Scenes(value)
    }
}

/// Request information for [`crate::client::Scenes::set_name`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetName<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// The scene to be renamed.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// New name for the scene.
    #[serde(rename = "newSceneName")]
    pub new_name: &'a str,
}

/// Request information for [`crate::client::Scenes::set_transition_override`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetTransitionOverride<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// The target scene.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Name of the scene transition to use as override.
    #[serde(rename = "transitionName")]
    pub transition: Option<&'a str>,
    /// Duration to use for any overridden transition.
    #[serde(
        rename = "transitionDuration",
        with = "crate::serde::duration_millis::option"
    )]
    pub duration: Option<Duration>,
}
