//! Responses related to the user interface.

use serde::Deserialize;

/// Response value for [`crate::client::Ui::get_studio_mode_enabled`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StudioModeEnabled {
    /// Whether studio mode is enabled.
    #[serde(rename = "studioModeEnabled")]
    pub enabled: bool,
}

/// Response value for [`crate::client::Ui::get_monitor_list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MonitorList {
    pub monitors: Vec<Monitor>,
}

/// Response value for [`crate::client::Ui::list_monitors`].
#[derive(Debug, Deserialize)]
pub struct Monitor {
    /// Name of this monitor.
    #[serde(rename = "monitorName")]
    pub name: String,
    /// Pixel size.
    #[serde(flatten)]
    pub size: MonitorSize,
    /// Position on the screen.
    #[serde(flatten)]
    pub position: MonitorPosition,
}

/// Response value for [`crate::client::Ui::list_monitors`] as part of [`Monitor`].
#[derive(Debug, Deserialize)]
pub struct MonitorSize {
    /// Pixel width.
    #[serde(rename = "monitorWidth")]
    pub width: u16,
    /// Pixel height.
    #[serde(rename = "monitorHeight")]
    pub height: u16,
}

/// Response value for [`crate::client::Ui::list_monitors`] as part of [`Monitor`].
#[derive(Debug, Deserialize)]
pub struct MonitorPosition {
    /// Horizontal position on the screen.
    #[serde(rename = "monitorPositionX")]
    pub x: u16,
    /// Vertical position on the screen.
    #[serde(rename = "monitorPositionY")]
    pub y: u16,
}
