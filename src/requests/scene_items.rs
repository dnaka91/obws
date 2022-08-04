//! Requests related to scene items.

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::common::{Alignment, BlendMode, BoundsType};

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetSceneItemList")]
    List {
        /// Name of the scene to get the items of.
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename = "GetGroupSceneItemList")]
    ListGroup {
        /// Name of the group to get the items of.
        #[serde(rename = "sceneName")]
        scene: &'a str,
    },
    #[serde(rename = "GetSceneItemId")]
    Id(Id<'a>),
    #[serde(rename = "CreateSceneItem")]
    Create(CreateSceneItem<'a>),
    #[serde(rename = "RemoveSceneItem")]
    Remove {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "DuplicateSceneItem")]
    Duplicate(Duplicate<'a>),
    #[serde(rename = "GetSceneItemTransform")]
    Transform {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "SetSceneItemTransform")]
    SetTransform(SetTransform<'a>),
    #[serde(rename = "GetSceneItemEnabled")]
    Enabled {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "SetSceneItemEnabled")]
    SetEnabled(SetEnabled<'a>),
    #[serde(rename = "GetSceneItemLocked")]
    Locked {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "SetSceneItemLocked")]
    SetLocked(SetLocked<'a>),
    #[serde(rename = "GetSceneItemIndex")]
    Index {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        /// Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "SetSceneItemIndex")]
    SetIndex(SetIndex<'a>),
    #[serde(rename = "GetSceneItemBlendMode")]
    BlendMode {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
        ///  Numeric ID of the scene item.
        #[serde(rename = "sceneItemId")]
        item_id: i64,
    },
    #[serde(rename = "SetSceneItemBlendMode")]
    SetBlendMode(SetBlendMode<'a>),
    #[serde(rename = "GetSceneItemPrivateSettings")]
    PrivateSettings {
        /// Name of the scene the item is in.
        #[serde(rename = "sceneName")]
        scene: &'a str,
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
pub struct Id<'a> {
    /// Name of the scene or group to search in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Name of the source to find.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Number of matches to skip during search.
    ///
    /// `>= 0` means first forward. `-1` means last (top) item.
    #[serde(rename = "searchOffset")]
    pub search_offset: Option<i32>,
}

/// Request information for [`crate::client::SceneItems::create`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct CreateSceneItem<'a> {
    /// Name of the scene to create the new item in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Name of the source to add to the scene.
    #[serde(rename = "sourceName")]
    pub source: &'a str,
    /// Enable state to apply to the scene item on creation.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: Option<bool>,
}

/// Request information for [`crate::client::SceneItems::duplicate`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub struct Duplicate<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Name of the scene to create the duplicated item in.
    #[serde(rename = "destinationSceneName")]
    pub destination: Option<&'a str>,
}

/// Request information for [`crate::client::SceneItems::set_transform`].
#[derive(Default, Serialize)]
pub struct SetTransform<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
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

/// Request information for [`crate::client::SceneItems::set_enabled`].
#[derive(Default, Serialize)]
pub struct SetEnabled<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New enable state of the scene item.
    #[serde(rename = "sceneItemEnabled")]
    pub enabled: bool,
}

/// Request information for [`crate::client::SceneItems::set_locked`].
#[derive(Default, Serialize)]
pub struct SetLocked<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New lock state of the scene item.
    #[serde(rename = "sceneItemLocked")]
    pub locked: bool,
}

/// Request information for [`crate::client::SceneItems::set_index`].
#[derive(Default, Serialize)]
pub struct SetIndex<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New index position of the scene item.
    #[serde(rename = "sceneItemIndex")]
    pub index: u32,
}

/// Request information for [`crate::client::SceneItems::set_blend_mode`].
#[derive(Serialize)]
pub struct SetBlendMode<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// New blend mode.
    #[serde(rename = "sceneItemBlendMode")]
    pub mode: BlendMode,
}

/// Request information for [`crate::client::SceneItems::set_private_settings`].
pub struct SetPrivateSettings<'a, T> {
    /// Name of the scene the item is in.
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    pub item_id: i64,
    /// Object of settings to apply.
    pub settings: &'a T,
}

/// Request information for
/// [`crate::client::SceneItems::set_scene_item_private_settings`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub(crate) struct SetPrivateSettingsInternal<'a> {
    /// Name of the scene the item is in.
    #[serde(rename = "sceneName")]
    pub scene: &'a str,
    /// Numeric ID of the scene item.
    #[serde(rename = "sceneItemId")]
    pub item_id: i64,
    /// Object of settings to apply.
    #[serde(rename = "sceneItemSettings")]
    pub settings: serde_json::Value,
}
