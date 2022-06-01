//! Responses related to scene items.

use serde::Deserialize;

use crate::common::{Alignment, BoundsType};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemId {
    /// Numeric ID of the scene item.
    pub scene_item_id: i64,
}

/// Response value for [`crate::client::SceneItems::get_list`] and
/// [`crate::client::SceneItems::get_group_list`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemList {
    /// Array of scene items in the scene or group.
    pub scene_items: Vec<SceneItem>,
}

/// Response value for [`crate::client::SceneItems::list`] and
/// [`crate::client::SceneItems::list_group`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItem {
    /// Identifier of the scene item.
    #[serde(rename = "sceneItemId")]
    pub id: i64,
    /// Positional index within a scene.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
    /// Name of this source.
    pub source_name: String,
    /// The kind of source this item represents.
    pub source_type: SourceType,
    /// Kind of input. Only present if this is a [`SourceType::Input`].
    pub input_kind: Option<String>,
    /// Whether this item is a group. Only present if this is a [`SourceType::Scene`].
    pub is_group: Option<bool>,
}

/// Kind of source that is represented by a [`SceneItem`].
#[derive(Copy, Clone, Debug, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub(crate) struct GetSceneItemTransform {
    pub scene_item_transform: SceneItemTransform,
}

/// Response value for [`crate::client::SceneItems::transform`].
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    /// Base width (without scaling) of the source.
    pub source_width: f32,
    /// Base height (without scaling) of the source.
    pub source_height: f32,
    /// The x position of the source from the left.
    pub position_x: f32,
    /// The y position of the source from the top.
    pub position_y: f32,
    /// The clockwise rotation of the scene item in degrees around the point of alignment.
    pub rotation: f32,
    /// The x-scale factor of the source.
    pub scale_x: f32,
    /// The y-scale factor of the source.
    pub scale_y: f32,
    /// Scene item width (base source width multiplied by the horizontal scaling factor).
    pub width: f32,
    /// Scene item height (base source height multiplied by the vertical scaling factor).
    pub height: f32,
    /// The point on the source that the item is manipulated from.
    #[serde(deserialize_with = "crate::de::bitflags_u8")]
    pub alignment: Alignment,
    /// Type of bounding box.
    pub bounds_type: BoundsType,
    /// Alignment of the bounding box.
    #[serde(deserialize_with = "crate::de::bitflags_u8")]
    pub bounds_alignment: Alignment,
    /// Width of the bounding box.
    pub bounds_width: f32,
    /// Height of the bounding box.
    pub bounds_height: f32,
    /// The number of pixels cropped off the left of the source before scaling.
    pub crop_left: u32,
    /// The number of pixels cropped off the right of the source before scaling.
    pub crop_right: u32,
    /// The number of pixels cropped off the top of the source before scaling.
    pub crop_top: u32,
    /// The number of pixels cropped off the bottom of the source before scaling.
    pub crop_bottom: u32,
}

/// Response value for [`crate::client::SceneItems::get_enabled`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemEnabled {
    /// Whether the scene item is enabled.
    pub scene_item_enabled: bool,
}

/// Response value for [`crate::client::SceneItems::get_locked`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemLocked {
    /// Whether the scene item is locked.
    pub scene_item_locked: bool,
}

/// Response value for [`crate::client::SceneItems::get_index`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemIndex {
    /// Index position of the scene item.
    pub scene_item_index: u32,
}

/// Response value for [`crate::client::SceneItems::get_private_settings`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemSettings<T> {
    pub scene_item_settings: T,
}
