//! Requests related to scenes.

use serde::Serialize;
use serde_with::skip_serializing_none;
use time::Duration;

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
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename = "GetCurrentPreviewScene")]
    CurrentPreviewScene,
    #[serde(rename = "SetCurrentPreviewScene")]
    SetCurrentPreviewScene {
        /// Scene to set as the current preview scene.
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename = "SetSceneName")]
    SetName {
        /// Name of the scene to be renamed.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// New name for the scene.
        #[serde(rename = "newSceneName")]
        new_name: &'a str,
    },
    #[serde(rename = "CreateScene")]
    Create {
        /// Name for the new scene.
        #[serde(rename = "sceneName")]
        name: &'a str,
    },
    #[serde(rename = "RemoveScene")]
    Remove {
        /// Name of the scene to remove.
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename = "GetSceneSceneTransitionOverride")]
    TransitionOverride {
        /// Name of the scene.
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename = "SetSceneSceneTransitionOverride")]
    SetTransitionOverride(SetTransitionOverride<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Scenes(value)
    }
}

/// Request information for [`crate::client::Scenes::set_transition_override`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct SetTransitionOverride<'a> {
    /// Name of the scene.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
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
