//! Common data structures shared between [`requests`](crate::requests),
//! [`responses`](crate::responses) and [`events`](crate::events).

use std::convert::TryFrom;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::Error;

/// Response value for [`get_current_scene`](crate::client::Scenes::get_current_scene) as part of
/// [`CurrentScene`](crate::responses::CurrentScene),
/// [`get_scene_list`](crate::client::Scenes::get_scene_list) as part of
/// [`Scene`](crate::responses::Scene),
/// [`get_preview_scene`](crate::client::StudioMode::get_preview_scene) as part of
/// [`PreviewScene`](crate::responses::PreviewScene),
/// [`EventType::SwitchScenes`](crate::events::EventType::SwitchScenes),
/// [`EventType::PreviewSceneChanged`](crate::events::EventType::PreviewSceneChanged),
///  and **itself**.
#[allow(missing_docs)] // Docs missing in the obs-websocket spec.
#[derive(Clone, Debug, Deserialize)]
pub struct SceneItem {
    pub cy: f64,
    pub cx: f64,
    /// The point on the source that the item is manipulated from. Omit to center on that axis.
    #[serde(deserialize_with = "crate::de::bitflags_u8")]
    pub alignment: Alignment,
    /// The name of this Scene Item.
    pub name: String,
    /// Scene item ID.
    pub id: i64,
    /// Whether this scene item is set to "visible".
    pub render: bool,
    /// Whether this scene item is muted.
    pub muted: bool,
    /// Whether this scene item is locked and can't be moved around
    pub locked: bool,
    pub source_cx: f64,
    pub source_cy: f64,
    /// Source type.
    #[serde(rename = "type")]
    pub ty: String,
    pub volume: f64,
    pub x: f64,
    pub y: f64,
    /// Name of the item's parent (if this item belongs to a group).
    #[serde(rename = "parentGroupName")]
    pub parent_group_name: Option<String>,
    /// List of children (if this item is a group).
    #[serde(rename = "groupChildren", default)]
    pub group_children: Vec<SceneItem>,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties),
/// [`EventType::SceneItemTransformChanged`](crate::events::EventType::SceneItemTransformChanged)
/// and **itself**.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    /// Position of the scene item.
    pub position: Position,
    /// The clockwise rotation of the scene item in degrees around the point of alignment.
    pub rotation: f64,
    /// Scaling factor of the scene item.
    pub scale: Scale,
    /// Pixel cropping of the scene item before scaling.
    pub crop: Crop,
    /// If the scene item is visible.
    pub visible: bool,
    /// If the scene item is locked in position.
    pub locked: bool,
    /// Bounding box of the source item.
    pub bounds: Bounds,
    /// Base width (without scaling) of the source.
    pub source_width: u64,
    /// Base source (without scaling) of the source.
    pub source_height: u64,
    /// Scene item width (base source width multiplied by the horizontal scaling factor).
    pub width: f64,
    /// Scene item height (base source height multiplied by the vertical scaling factor).
    pub height: f64,
    /// Name of the item's parent (if this item belongs to a group).
    pub parent_group_name: Option<String>,
    /// List of children (if this item is a group).
    #[serde(default)]
    pub group_children: Vec<SceneItemTransform>,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties).
#[derive(Clone, Debug, Deserialize)]
pub struct Position {
    /// The x position of the source from the left.
    pub x: f64,
    /// The y position of the source from the top.
    pub y: f64,
    /// The point on the source that the item is manipulated from. Omit to center on that axis.
    #[serde(deserialize_with = "crate::de::bitflags_u8")]
    pub alignment: Alignment,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties) and [`SceneItemTransform`].
#[derive(Clone, Debug, Deserialize)]
pub struct Scale {
    /// The x-scale factor of the source.
    pub x: f64,
    /// The y-scale factor of the source.
    pub y: f64,
    /// The scale filter of the source.
    pub filter: ScaleFilter,
}

/// Different scaling filters that can be applied to a scene item as part of [`Scale`].
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ScaleFilter {
    /// Disable any scaling filters.
    #[serde(rename = "OBS_SCALE_DISABLE")]
    Disable,
    /// Nearest neighbor scaling.
    #[serde(rename = "OBS_SCALE_POINT")]
    Point,
    /// Sharpened scaling, 16 samples.
    #[serde(rename = "OBS_SCALE_BICUBIC")]
    Bicubic,
    /// Fast but blurry scaling.
    #[serde(rename = "OBS_SCALE_BILINEAR")]
    Bilinear,
    /// Sharpened scaling, 36 samples.
    #[serde(rename = "OBS_SCALE_LANCZOS")]
    Lanczos,
    /// Weighted sum, 4/6/9 samples.
    #[serde(rename = "OBS_SCALE_AREA")]
    Area,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties) and [`SceneItemTransform`].
#[derive(Clone, Debug, Deserialize)]
pub struct Crop {
    /// The number of pixels cropped off the top of the source before scaling.
    pub top: u32,
    /// The number of pixels cropped off the right of the source before scaling.
    pub right: u32,
    /// The number of pixels cropped off the bottom of the source before scaling.
    pub bottom: u32,
    /// The number of pixels cropped off the left of the source before scaling.
    pub left: u32,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties) and [`SceneItemTransform`].
#[derive(Clone, Debug, Deserialize)]
pub struct Bounds {
    /// Type of bounding box.
    #[serde(rename = "type")]
    pub ty: BoundsType,
    /// Alignment of the bounding box.
    #[serde(deserialize_with = "crate::de::bitflags_u8")]
    pub alignment: Alignment,
    /// Width of the bounding box.
    pub x: f64,
    /// Height of the bounding box.
    pub y: f64,
}

/// Monitoring type for audio outputs.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitorType {
    /// No monitoring.
    None,
    /// Only monitor but don't output any sounds.
    MonitorOnly,
    /// Monitor the audio and output it at the same time.
    MonitorAndOutput,
}

/// Text alignment used for GDI+ text properties.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    /// Align to the left.
    Left,
    /// Center the text in the middle (horizontally).
    Center,
    /// Align to the right.
    Right,
}

/// Vertical text alignment use for GDI+ text properties.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Valign {
    /// Align to the top.
    Top,
    /// Center the text in the middle (vertically).
    Center,
    /// Align to the bottom.
    Bottom,
}

/// The type of streaming for service configurations.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamType {
    /// Customized RTMP streaming.
    RtmpCustom,
    /// Common RTMP configuration.
    RtmpCommon,
}

bitflags! {
    /// Different flags for font display that can be combined.
    pub struct FontFlags: u8 {
        /// Make the text appear thicker.
        const BOLD = 1;
        /// Make the text appear cursive.
        const ITALIC = 2;
        /// Underline the text with a straight line.
        const UNDERLINE = 4;
        /// Strikeout the text.
        const STRIKEOUT = 8;
    }
}

impl TryFrom<u8> for FontFlags {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or(Error::UnknownFlags(value))
    }
}

impl From<FontFlags> for u8 {
    fn from(value: FontFlags) -> Self {
        value.bits
    }
}

bitflags! {
    /// Alignment for different items on the scene that is described in two axis. The default is
    /// center for both axis.
    ///
    /// For example, only using `LEFT` would arrange the target to the left horizontally and
    /// centered vertically. To align to the top right, the alignments can be combined to
    /// `LEFT | TOP`. Combining both values for a single axis is invalid, like `LEFT | RIGHT`.
    pub struct Alignment: u8 {
        /// Align to the left side.
        const LEFT = 1;
        /// Align to the right side.
        const RIGHT = 2;
        /// Align to the top.
        const TOP = 4;
        /// Align to the bottom.
        const BOTTOM = 8;
    }
}

impl TryFrom<u8> for Alignment {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_bits(value).ok_or(Error::UnknownFlags(value))
    }
}

impl From<Alignment> for u8 {
    fn from(value: Alignment) -> Self {
        value.bits
    }
}

/// Different kinds of bounds that can be applied to different items on the scene as part of the
/// [`Bounds`] type.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BoundsType {
    /// Stretch to bounds.
    #[serde(rename = "OBS_BOUNDS_STRETCH")]
    Stretch,
    /// Scale to inner bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_INNER")]
    ScaleInner,
    /// Scale to outer bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_OUTER")]
    ScaleOuter,
    /// Scale to width of bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_WIDTH")]
    ScaleToWidth,
    /// Scale to height of bounds.
    #[serde(rename = "OBS_BOUNDS_SCALE_TO_HEIGHT")]
    ScaleToHeight,
    /// Maximum size only.
    #[serde(rename = "OBS_BOUNDS_MAX_ONLY")]
    MaxOnly,
    /// No bounds.
    #[serde(rename = "OBS_BOUNDS_NONE")]
    None,
}
