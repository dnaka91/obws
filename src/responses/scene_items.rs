//! Responses related to scene items.

use serde::{Deserialize, Serialize};

use crate::common::{Alignment, BlendMode, BoundsType};

#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemId {
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub id: i64,
}

/// Response value for [`crate::client::SceneItems::get_list`] and
/// [`crate::client::SceneItems::get_group_list`].
#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemList {
    /// Array of scene items in the scene or group.
    #[serde(rename = "sceneItems")]
    pub scene_items: Vec<SceneItem>,
}

/// Response value for [`crate::client::SceneItems::list`] and
/// [`crate::client::SceneItems::list_group`].
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SceneItem {
    /// Identifier of the scene item.
    #[serde(rename = "sceneItemId")]
    pub id: i64,
    /// Positional index within a scene.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
    /// Name of this source.
    #[serde(rename = "sourceName")]
    pub source_name: String,
    /// The kind of source this item represents.
    #[serde(rename = "sourceType")]
    pub source_type: SourceType,
    /// Kind of input. Only present if this is a [`SourceType::Input`].
    #[serde(rename = "inputKind")]
    pub input_kind: Option<String>,
    /// Whether this item is a group. Only present if this is a [`SourceType::Scene`].
    #[serde(rename = "isGroup")]
    pub is_group: Option<bool>,
}

/// Kind of source that is represented by a [`SceneItem`].
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[non_exhaustive]
pub enum SourceType {
    /// Input source from outside of OBS.
    #[serde(rename = "OBS_SOURCE_TYPE_INPUT")]
    Input,
    /// Filter applied to other items.
    #[serde(rename = "OBS_SOURCE_TYPE_FILTER")]
    Filter,
    /// Transition when switching scenes.
    #[serde(rename = "OBS_SOURCE_TYPE_TRANSITION")]
    Transition,
    /// Scene in OBS.
    #[serde(rename = "OBS_SOURCE_TYPE_SCENE")]
    Scene,
}

/// Response value for
/// [`crate::client::SceneItems::get_scene_item_transform`].
#[derive(Debug, Deserialize)]
pub(crate) struct GetSceneItemTransform {
    #[serde(rename = "sceneItemTransform")]
    pub transform: SceneItemTransform,
}

/// Response value for [`crate::client::SceneItems::transform`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SceneItemTransform {
    /// Base width (without scaling) of the source.
    #[serde(rename = "sourceWidth")]
    pub source_width: f32,
    /// Base height (without scaling) of the source.
    #[serde(rename = "sourceHeight")]
    pub source_height: f32,
    /// The x position of the source from the left.
    #[serde(rename = "positionX")]
    pub position_x: f32,
    /// The y position of the source from the top.
    #[serde(rename = "positionY")]
    pub position_y: f32,
    /// The clockwise rotation of the scene item in degrees around the point of alignment.
    #[serde(rename = "rotation")]
    pub rotation: f32,
    /// The x-scale factor of the source.
    #[serde(rename = "scaleX")]
    pub scale_x: f32,
    /// The y-scale factor of the source.
    #[serde(rename = "scaleY")]
    pub scale_y: f32,
    /// Scene item width (base source width multiplied by the horizontal scaling factor).
    #[serde(rename = "width")]
    pub width: f32,
    /// Scene item height (base source height multiplied by the vertical scaling factor).
    #[serde(rename = "height")]
    pub height: f32,
    /// The point on the source that the item is manipulated from.
    #[serde(rename = "alignment", with = "crate::serde::bitflags_u8")]
    pub alignment: Alignment,
    /// Type of bounding box.
    #[serde(rename = "boundsType")]
    pub bounds_type: BoundsType,
    /// Alignment of the bounding box.
    #[serde(rename = "boundsAlignment", with = "crate::serde::bitflags_u8")]
    pub bounds_alignment: Alignment,
    /// Width of the bounding box.
    #[serde(rename = "boundsWidth")]
    pub bounds_width: f32,
    /// Height of the bounding box.
    #[serde(rename = "boundsHeight")]
    pub bounds_height: f32,
    /// The number of pixels cropped off the left of the source before scaling.
    #[serde(rename = "cropLeft")]
    pub crop_left: u32,
    /// The number of pixels cropped off the right of the source before scaling.
    #[serde(rename = "cropRight")]
    pub crop_right: u32,
    /// The number of pixels cropped off the top of the source before scaling.
    #[serde(rename = "cropTop")]
    pub crop_top: u32,
    /// The number of pixels cropped off the bottom of the source before scaling.
    #[serde(rename = "cropBottom")]
    pub crop_bottom: u32,
}

/// Response value for [`crate::client::SceneItems::enabled`].
#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemEnabled {
    /// Whether the scene item is enabled.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: bool,
}

/// Response value for [`crate::client::SceneItems::locked`].
#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemLocked {
    /// Whether the scene item is locked.
    #[serde(rename = "sceneItemLocked")]
    pub locked: bool,
}

/// Response value for [`crate::client::SceneItems::index`].
#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemIndex {
    /// Index position of the scene item.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
}

/// Response value for [`crate::client::SceneItems::blend_mode`].
#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemBlendMode {
    #[serde(rename = "sceneItemBlendMode")]
    pub blend_mode: BlendMode,
}

/// Response value for [`crate::client::SceneItems::private_settings`].
#[derive(Debug, Deserialize)]
pub(crate) struct SceneItemSettings<T> {
    #[serde(rename = "sceneItemSettings")]
    pub settings: T,
}
