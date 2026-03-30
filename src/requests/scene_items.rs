//! Requests related to scene items.

use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

use super::{ids::DestinationSceneId, scenes::SceneId, sources::SourceId};
use crate::common::{self, Alignment, BoundsType};

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetSceneItemList")]
    List {
        /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
        #[serde(rename = "canvasUuid")]
        canvas: Option<Uuid>,
        /// Identifier of the scene to get the items of.
        #[serde(flatten)]
        scene: SceneId<'a>,
    },
    #[serde(rename = "GetGroupSceneItemList")]
    ListGroup {
        /// UUID of the canvas the group is in, if using the [`SceneId::Name`].
        #[serde(rename = "canvasUuid")]
        canvas: Option<Uuid>,
        /// Identifier of the group to get the items of.
        #[serde(flatten)]
        scene: SceneId<'a>,
    },
    #[serde(rename = "GetSceneItemId")]
    Id(Id<'a>),
    #[serde(rename = "GetSceneItemSource")]
    Source(Source<'a>),
    #[serde(rename = "CreateSceneItem")]
    Create(CreateSceneItem<'a>),
    #[serde(rename = "RemoveSceneItem")]
    Remove(Remove<'a>),
    #[serde(rename = "DuplicateSceneItem")]
    Duplicate(Duplicate<'a>),
    #[serde(rename = "GetSceneItemTransform")]
    Transform(Transform<'a>),
    #[serde(rename = "SetSceneItemTransform")]
    SetTransform(SetTransform<'a>),
    #[serde(rename = "GetSceneItemEnabled")]
    Enabled(Enabled<'a>),
    #[serde(rename = "SetSceneItemEnabled")]
    SetEnabled(SetEnabled<'a>),
    #[serde(rename = "GetSceneItemLocked")]
    Locked(Locked<'a>),
    #[serde(rename = "SetSceneItemLocked")]
    SetLocked(SetLocked<'a>),
    #[serde(rename = "GetSceneItemIndex")]
    Index(Index<'a>),
    #[serde(rename = "SetSceneItemIndex")]
    SetIndex(SetIndex<'a>),
    #[serde(rename = "GetSceneItemBlendMode")]
    BlendMode(BlendMode<'a>),
    #[serde(rename = "SetSceneItemBlendMode")]
    SetBlendMode(SetBlendMode<'a>),
    #[serde(rename = "GetSceneItemPrivateSettings")]
    PrivateSettings {
        /// Identifier of the scene the item is in.
        #[serde(flatten)]
        scene: SceneId<'a>,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "SetSceneItemPrivateSettings")]
    SetPrivateSettings(SetPrivateSettingsInternal<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::SceneItems(value)
    }
}

/// Request information for [`crate::client::SceneItems::id`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Id<'a> {
    /// UUID of the canvas the scene or group is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene or group to search in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Name of the source to find.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Number of matches to skip during search.
    ///
    /// `>= 0` means first forward. `-1` means last (top) item.
    #[serde(rename = "searchOffset")]
    pub search_offset: Option<i32>,
}

/// Request information for [`crate::client::SceneItems::source`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Source<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::create`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct CreateSceneItem<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene to create the new item in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Identifier of the source to add to the scene.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Enable state to apply to the scene item on creation.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: Option<bool>,
}

/// Request information for [`crate::client::SceneItems::remove`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Remove<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::duplicate`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Duplicate<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Identifier of the scene to create the duplicated item in.
    #[serde(flatten)]
    pub destination: Option<DestinationSceneId<'a>>,
}

/// Request information for [`crate::client::SceneItems::transform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Transform<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::set_transform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetTransform<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Object containing scene item transform info to update.
    #[serde(rename = "sceneItemTransform")]
    pub transform: SceneItemTransform,
}

/// Request information for [`crate::client::SceneItems::set_transform`] as part of
/// [`SetTransform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SceneItemTransform {
    /// Position (or offset) on the screen.
    #[serde(rename = "position", flatten)]
    pub position: Option<Position>,
    /// The clockwise rotation of the scene item in degrees around the point of alignment.
    #[serde(rename = "rotation")]
    pub rotation: Option<f32>,
    /// Scaling of the item.
    #[serde(rename = "scale", flatten)]
    pub scale: Option<Scale>,
    /// The point on the source that the item is manipulated from.
    #[serde(rename = "alignment")]
    pub alignment: Option<Alignment>,
    /// Bound restrictions on the item.
    #[serde(rename = "bounds", flatten)]
    pub bounds: Option<Bounds>,
    /// Cropping values on up to 4 sides.
    #[serde(rename = "crop", flatten)]
    pub crop: Option<Crop>,
}

impl From<crate::responses::scene_items::SceneItemTransform> for SceneItemTransform {
    fn from(t: crate::responses::scene_items::SceneItemTransform) -> Self {
        Self {
            position: Some(Position {
                x: Some(t.position_x),
                y: Some(t.position_y),
            }),
            rotation: Some(t.rotation),
            scale: Some(Scale {
                x: Some(t.scale_x),
                y: Some(t.scale_y),
            }),
            alignment: Some(t.alignment),
            bounds: Some(Bounds {
                r#type: Some(t.bounds_type),
                alignment: Some(t.bounds_alignment),
                width: Some(t.bounds_width),
                height: Some(t.bounds_height),
            }),
            crop: Some(Crop {
                left: Some(t.crop_left),
                right: Some(t.crop_right),
                top: Some(t.crop_top),
                bottom: Some(t.crop_bottom),
            }),
        }
    }
}

/// Request information for [`crate::client::SceneItems::set_transform`] as part of
/// [`SceneItemTransform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Position {
    /// The x position of the source from the left.
    #[serde(rename = "positionX")]
    pub x: Option<f32>,
    /// The y position of the source from the top.
    #[serde(rename = "positionY")]
    pub y: Option<f32>,
}

/// Request information for [`crate::client::SceneItems::set_transform`] as part of
/// [`SceneItemTransform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Scale {
    /// The x-scale factor of the source.
    #[serde(rename = "scaleX")]
    pub x: Option<f32>,
    /// The y-scale factor of the source.
    #[serde(rename = "scaleY")]
    pub y: Option<f32>,
}

/// Request information for [`crate::client::SceneItems::set_transform`] as part of
/// [`SceneItemTransform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Bounds {
    /// Type of bounding box.
    #[serde(rename = "boundsType")]
    pub r#type: Option<BoundsType>,
    /// Alignment of the bounding box.
    #[serde(rename = "boundsAlignment")]
    pub alignment: Option<Alignment>,
    /// Width of the bounding box.
    #[serde(rename = "boundsWidth")]
    pub width: Option<f32>,
    /// Height of the bounding box.
    #[serde(rename = "boundsHeight")]
    pub height: Option<f32>,
}

/// Request information for [`crate::client::SceneItems::set_transform`] as part of
/// [`SceneItemTransform`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Crop {
    /// The number of pixels cropped off the left of the source before scaling.
    #[serde(rename = "cropLeft")]
    pub left: Option<u32>,
    /// The number of pixels cropped off the right of the source before scaling.
    #[serde(rename = "cropRight")]
    pub right: Option<u32>,
    /// The number of pixels cropped off the top of the source before scaling.
    #[serde(rename = "cropTop")]
    pub top: Option<u32>,
    /// The number of pixels cropped off the bottom of the source before scaling.
    #[serde(rename = "cropBottom")]
    pub bottom: Option<u32>,
}

/// Request information for [`crate::client::SceneItems::enabled`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Enabled<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::set_enabled`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetEnabled<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New enable state of the scene item.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: bool,
}

/// Request information for [`crate::client::SceneItems::locked`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Locked<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::set_locked`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetLocked<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New lock state of the scene item.
    #[serde(rename = "sceneItemLocked")]
    pub locked: bool,
}

/// Request information for [`crate::client::SceneItems::index`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Index<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::set_index`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetIndex<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New index position of the scene item.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
}

/// Request information for [`crate::client::SceneItems::blend_mode`].
#[skip_serializing_none]
#[derive(Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct BlendMode<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    ///  Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
}

/// Request information for [`crate::client::SceneItems::set_blend_mode`].
#[skip_serializing_none]
#[derive(Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetBlendMode<'a> {
    /// UUID of the canvas the scene is in, if using the [`SceneId::Name`].
    #[serde(rename = "canvasUuid")]
    pub canvas: Option<Uuid>,
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New blend mode.
    #[serde(rename = "sceneItemBlendMode")]
    pub mode: common::BlendMode,
}

/// Request information for [`crate::client::SceneItems::set_private_settings`].
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetPrivateSettings<'a, T> {
    /// Identifier of the scene the item is in.
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    pub item_id: i64,
    /// Object of settings to apply.
    pub settings: &'a T,
}

/// Request information for [`crate::client::SceneItems::set_private_settings`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub(crate) struct SetPrivateSettingsInternal<'a> {
    /// Identifier of the scene the item is in.
    #[serde(flatten)]
    pub scene: SceneId<'a>,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Object of settings to apply.
    #[serde(rename = "sceneItemSettings")]
    pub settings: serde_json::Value,
}
