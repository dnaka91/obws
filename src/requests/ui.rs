//! Requests related to the user interface.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetStudioModeEnabled")]
    GetStudioModeEnabled,
    #[serde(rename = "SetStudioModeEnabled")]
    SetStudioModeEnabled {
        /// Enable or disable the studio mode.
        #[serde(rename = "studioModeEnabled")]
        enabled: bool,
    },
    #[serde(rename = "OpenInputPropertiesDialog")]
    OpenInputPropertiesDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "OpenInputFiltersDialog")]
    OpenInputFiltersDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "OpenInputInteractDialog")]
    OpenInputInteractDialog {
        /// Name of the input to open the dialog of.
        #[serde(rename = "inputName")]
        input: &'a str,
    },
    #[serde(rename = "GetMonitorList")]
    GetMonitorList,
    #[serde(rename = "OpenVideoMixProjector")]
    OpenVideoMixProjector(OpenVideoMixProjector<'a>),
    #[serde(rename = "OpenSourceProjector")]
    OpenSourceProjector(OpenSourceProjector<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Ui(value)
    }
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`].
#[derive(Serialize)]
pub struct OpenVideoMixProjector<'a> {
    /// Type of mix to open.
    #[serde(rename = "videoMixType")]
    pub r#type: VideoMixType,
    /// Optional location for the new projector window.
    pub location: Option<Location<'a>>,
}

/// Request information for [`crate::client::Ui::open_source_projector`].
#[derive(Serialize)]
pub struct OpenSourceProjector<'a> {
    /// Name of the source to open a projector for.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Optional location for the new projector window.
    #[serde(flatten)]
    pub location: Option<Location<'a>>,
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`] as part of
/// [`OpenVideoMixProjector`] and [`crate::client::Ui::open_source_projector`] as part of
/// [`OpenSourceProjector`], describing the open location of the projector.
#[derive(Serialize)]
pub enum Location<'a> {
    /// Monitor index, passing `-1` opens the projector in windowed mode.
    #[serde(rename = "monitorIndex")]
    MonitorIndex(i32),
    /// Size/Position data for a windowed projector, in `Qt Base64` encoded format.
    #[serde(rename = "projectorGeometry")]
    ProjectorGeometry(&'a str),
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`] as part of
/// [`OpenVideoMixProjector`], defining the type of video mix to open.
#[derive(Serialize)]
pub enum VideoMixType {
    /// Show the preview scene.
    #[serde(rename = "OBS_WEBSOCKET_VIDEO_MIX_TYPE_PREVIEW")]
    Preview,
    /// Show the program scene.
    #[serde(rename = "OBS_WEBSOCKET_VIDEO_MIX_TYPE_PROGRAM")]
    Program,
    /// Show a multi-view.
    #[serde(rename = "OBS_WEBSOCKET_VIDEO_MIX_TYPE_MULTIVIEW")]
    Multiview,
}
