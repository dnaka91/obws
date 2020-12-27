//! Common data structures shared between [`requests`](crate::requests),
//! [`responses`](crate::responses) and [`events`](crate::events).

use serde::Deserialize;

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
    /// The point on the source that the item is manipulated from. The sum of 1=Left or 2=Right, and
    /// 4=Top or 8=Bottom, or omit to center on that axis.
    pub alignment: u8,
    /// The name of this Scene Item.
    pub name: String,
    /// Scene item ID.
    pub id: i64,
    /// Whether or not this Scene Item is set to "visible".
    pub render: bool,
    /// Whether or not this Scene Item is muted.
    pub muted: bool,
    /// Whether or not this Scene Item is locked and can't be moved around
    pub locked: bool,
    pub source_cx: f64,
    pub source_cy: f64,
    /// Source type. Value is one of the following: "input", "filter", "transition", "scene" or
    /// "unknown".
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
    /// The point on the source that the item is manipulated from. The sum of 1=Left or 2=Right, and
    /// 4=Top or 8=Bottom, or omit to center on that axis.
    pub alignment: u8,
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
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties) and [`SceneItemTransform`].
#[derive(Clone, Debug, Deserialize)]
pub struct Crop {
    /// The number of pixels cropped off the top of the source before scaling.
    pub top: i64,
    /// The number of pixels cropped off the right of the source before scaling.
    pub right: i64,
    /// The number of pixels cropped off the bottom of the source before scaling.
    pub bottom: i64,
    /// The number of pixels cropped off the left of the source before scaling.
    pub left: i64,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties) as part of
/// [`SceneItemProperties`](crate::responses::SceneItemProperties) and [`SceneItemTransform`].
#[derive(Clone, Debug, Deserialize)]
pub struct Bounds {
    /// Type of bounding box. Can be "OBS_BOUNDS_STRETCH", "OBS_BOUNDS_SCALE_INNER",
    /// "OBS_BOUNDS_SCALE_OUTER", "OBS_BOUNDS_SCALE_TO_WIDTH", "OBS_BOUNDS_SCALE_TO_HEIGHT",
    /// "OBS_BOUNDS_MAX_ONLY" or "OBS_BOUNDS_NONE".
    #[serde(rename = "type")]
    pub ty: String,
    /// Alignment of the bounding box.
    pub alignment: u8,
    /// Width of the bounding box.
    pub x: f64,
    /// Height of the bounding box.
    pub y: f64,
}
