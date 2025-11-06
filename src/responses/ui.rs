//! Responses related to the user interface.

use serde::{Deserialize, Serialize};

/// Response value for [`crate::client::Ui::studio_mode_enabled`].
#[derive(Debug, Deserialize)]
pub(crate) struct StudioModeEnabled {
    /// Whether studio mode is enabled.
    #[serde(rename = "studioModeEnabled")]
    pub enabled: bool,
}

/// Response value for [`crate::client::Ui::list_monitors`].
#[derive(Debug, Deserialize)]
pub(crate) struct MonitorList {
    #[serde(rename = "monitors")]
    pub monitors: Vec<Monitor>,
}

/// Response value for [`crate::client::Ui::list_monitors`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Monitor {
    /// Name of this monitor.
    #[serde(rename = "monitorName")]
    pub name: String,
    /// Positional index in the list of monitors.
    #[serde(rename = "monitorIndex")]
    pub index: u32,
    /// Pixel size.
    #[serde(flatten)]
    pub size: MonitorSize,
    /// Position on the screen.
    #[serde(flatten)]
    pub position: MonitorPosition,
}

/// Response value for [`crate::client::Ui::list_monitors`] as part of [`Monitor`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MonitorSize {
    /// Pixel width.
    #[serde(rename = "monitorWidth")]
    pub width: u16,
    /// Pixel height.
    #[serde(rename = "monitorHeight")]
    pub height: u16,
}

/// Response value for [`crate::client::Ui::list_monitors`] as part of [`Monitor`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MonitorPosition {
    /// Horizontal position on the screen.
    #[serde(rename = "monitorPositionX")]
    pub x: i32,
    /// Vertical position on the screen.
    #[serde(rename = "monitorPositionY")]
    pub y: i32,
}
