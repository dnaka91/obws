//! Requests related to the user interface.

use bitflags::bitflags;
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
    OpenVideoMixProjector(OpenVideoMixProjectorInternal),
    #[serde(rename = "OpenSourceProjector")]
    OpenSourceProjector(OpenSourceProjectorInternal<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Ui(value)
    }
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`].
pub struct OpenVideoMixProjector {
    /// Type of mix to open.
    pub r#type: VideoMixType,
    /// Optional location for the new projector window.
    pub location: Option<Location>,
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`].
#[derive(Serialize)]
pub(crate) struct OpenVideoMixProjectorInternal {
    /// Type of mix to open.
    #[serde(rename = "videoMixType")]
    pub r#type: VideoMixType,
    /// Optional location for the new projector window.
    #[serde(flatten)]
    pub location: Option<LocationInternal>,
}

/// Request information for [`crate::client::Ui::open_source_projector`].
pub struct OpenSourceProjector<'a> {
    /// Name of the source to open a projector for.
    pub source: &'a str,
    /// Optional location for the new projector window.
    pub location: Option<Location>,
}

/// Request information for [`crate::client::Ui::open_source_projector`].
#[derive(Serialize)]
pub(crate) struct OpenSourceProjectorInternal<'a> {
    /// Name of the source to open a projector for.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Optional location for the new projector window.
    #[serde(flatten)]
    pub location: Option<LocationInternal>,
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`] as part of
/// [`OpenVideoMixProjector`] and [`crate::client::Ui::open_source_projector`] as part of
/// [`OpenSourceProjector`], describing the open location of the projector.
#[non_exhaustive]
pub enum Location {
    /// Monitor index, passing `-1` opens the projector in windowed mode.
    MonitorIndex(i32),
    /// Size/Position data for a windowed projector, in `Qt Base64` encoded format.
    ProjectorGeometry(QtGeometry),
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`] as part of
/// [`OpenVideoMixProjector`] and [`crate::client::Ui::open_source_projector`] as part of
/// [`OpenSourceProjector`], describing the open location of the projector.
#[derive(Serialize)]
pub(crate) enum LocationInternal {
    /// Monitor index, passing `-1` opens the projector in windowed mode.
    #[serde(rename = "monitorIndex")]
    MonitorIndex(i32),
    /// Size/Position data for a windowed projector, in `Qt Base64` encoded format.
    #[serde(rename = "projectorGeometry")]
    ProjectorGeometry(String),
}

impl From<Location> for LocationInternal {
    fn from(value: Location) -> Self {
        match value {
            Location::MonitorIndex(index) => Self::MonitorIndex(index),
            Location::ProjectorGeometry(geometry) => Self::ProjectorGeometry(geometry.serialize()),
        }
    }
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`] as part of
/// [`OpenVideoMixProjector`], defining the type of video mix to open.
#[derive(Serialize)]
#[non_exhaustive]
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

/// Request information for [`crate::client::Ui::open_video_mix_projector`] and
/// [`crate::client::Ui::open_source_projector`] as part of [`Location`].
#[derive(Debug)]
pub struct QtGeometry {
    /// The screen number to display a widget or [`Self::DEFAULT_SCREEN`] to let OBS pick the
    /// default.
    pub screen_number: i32,
    /// Additional window state like maximized or full-screen.
    pub window_state: QtWindowState,
    /// The width of the screen. Seems to have no specific effect but is used for some internal
    /// calculations in Qt.
    pub screen_width: i32,
    /// The target position and size for a widget to display at.
    pub rect: QtRect,
}

impl QtGeometry {
    /// Value indicating to use the default screen.
    pub const DEFAULT_SCREEN: i32 = -1;

    /// Create a new geometry instance without only size information set.
    pub fn new(rect: QtRect) -> Self {
        Self {
            rect,
            ..Self::default()
        }
    }

    /// Serialize this instance into a `base64` encoded byte array.
    ///
    /// The exact format can be found in the
    /// [Qt source code](https://code.woboq.org/qt5/qtbase/src/widgets/kernel/qwidget.cpp.html#_ZNK7QWidget12saveGeometryEv).
    ///
    /// | Length | Content                                                  |
    /// |--------|----------------------------------------------------------|
    /// | 4      | Magic number                                             |
    /// | 2      | Major format version                                     |
    /// | 2      | Minor format version                                     |
    /// | 16     | Frame rectangle (left, top, right, bottom) 4 bytes each  |
    /// | 16     | Normal rectangle (left, top, right, bottom) 4 bytes each |
    /// | 4      | Screen number                                            |
    /// | 1      | Window maximized (1 or 0)                                |
    /// | 1      | Window full-screen (1 or 0)                              |
    /// | 4      | Screen width                                             |
    /// | 16     | Main rectangle (left, top, right, bottom) 4 bytes each   |
    pub(crate) fn serialize(&self) -> String {
        use base64::engine::{general_purpose, Engine};

        /// Indicator for serialized Qt geometry data.
        const MAGIC_NUMBER: u32 = 0x1D9D0CB;
        /// Major version of this format.
        const MAJOR_VERSION: u16 = 3;
        /// Minor version of this format.
        const MINOR_VERSION: u16 = 0;
        /// Output data length BEFORE `base64` encoding. This allows to reduce allocations in the
        /// byte buffer and must be updated whenever the format changes.
        const DATA_LENGTH: usize = 66;

        fn serialize_rect(data: &mut Vec<u8>, rect: &QtRect) {
            data.extend(rect.left.to_be_bytes());
            data.extend(rect.top.to_be_bytes());
            data.extend(rect.right.to_be_bytes());
            data.extend(rect.bottom.to_be_bytes());
        }

        let mut data = Vec::<u8>::with_capacity(DATA_LENGTH);

        data.extend(MAGIC_NUMBER.to_be_bytes());
        data.extend(MAJOR_VERSION.to_be_bytes());
        data.extend(MINOR_VERSION.to_be_bytes());

        serialize_rect(&mut data, &self.rect); // frame geometry
        serialize_rect(&mut data, &self.rect); // normal geometry

        data.extend(self.screen_number.to_be_bytes());
        data.extend(self.window_state.to_be_bytes());
        data.extend(self.screen_width.to_be_bytes());

        serialize_rect(&mut data, &self.rect);

        general_purpose::STANDARD.encode(data)
    }
}

impl Default for QtGeometry {
    fn default() -> Self {
        Self {
            screen_number: Self::DEFAULT_SCREEN,
            window_state: QtWindowState::default(),
            screen_width: 0,
            rect: QtRect::default(),
        }
    }
}

bitflags! {
    /// Request information for [`open_projector`](crate::client::General::open_projector) as part of
    /// [`Projector`].
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct QtWindowState: u32 {
        /// Window with maximum size, taking up as much space as possible but still showing
        /// the window frame.
        const MAXIMIZED = 2;
        /// Show the window in full-screen mode, taking up the whole display.
        const FULLSCREEN = 4;
    }
}

impl QtWindowState {
    /// Convert the state into a byte array for usage in [`QtGeometry::serialize`] .
    fn to_be_bytes(self) -> [u8; 2] {
        [
            u8::from(self.contains(Self::MAXIMIZED)),
            u8::from(self.contains(Self::FULLSCREEN)),
        ]
    }
}

/// Request information for [`crate::client::Ui::open_video_mix_projector`] and
/// [`crate::client::Ui::open_source_projector`] as part of [`QtGeometry`].
///
/// This describes a position on the screen starting from the top left corner with 0.
///
/// ```txt
/// Screen
/// ┌────────────────────── X
/// │
/// │          top
/// │       ┌────────┐
/// │  left │  Rect  │ right
/// │       └────────┘
/// │         bottom
/// │
/// Y
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct QtRect {
    /// Left or X/horizontal position of the rectangle.
    pub left: i32,
    /// Top or Y/vertical position of the rectangle.
    pub top: i32,
    /// The right side of a rectangle counted from the left. For example with `left = 100` and
    /// `right = 300` the width would be `200`.
    pub right: i32,
    /// Bottom side of a rectangle counted from the top. For example with `top = 100` and
    /// `bottom = 300` the height would be `200`.
    pub bottom: i32,
}
