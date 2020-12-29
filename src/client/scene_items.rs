use either::Either;

use super::Client;
use crate::requests::{
    AddSceneItem, DuplicateSceneItem, RequestType, SceneItemProperties, SceneItemRender,
    SceneItemSpecification,
};
use crate::responses;
use crate::Result;

/// API functions related to scene items.
pub struct SceneItems<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneItems<'a> {
    /// Gets the scene specific properties of the specified source item. Coordinates are relative to
    /// the item's parent (the scene or group it belongs to).
    ///
    /// - `scene_name`: Name of the scene the scene item belongs to. Defaults to the current scene.
    /// - `item`: Scene Item name (if this field is a string) or specification (if it is an object).
    pub async fn get_scene_item_properties(
        &self,
        scene_name: Option<&str>,
        item: Either<&str, SceneItemSpecification<'_>>,
    ) -> Result<Vec<responses::SceneItemProperties>> {
        self.client
            .send_message(RequestType::GetSceneItemProperties { scene_name, item })
            .await
    }

    /// Sets the scene specific properties of a source. Unspecified properties will remain
    /// unchanged. Coordinates are relative to the item's parent (the scene or group it belongs to).
    pub async fn set_scene_item_properties(
        &self,
        properties: SceneItemProperties<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemProperties(properties))
            .await
    }

    /// Reset a scene item.
    ///
    /// - `scene_name`: Name of the scene the scene item belongs to. Defaults to the current scene.
    /// - `item`: Scene Item name (if this field is a string) or specification (if it is an object).
    pub async fn reset_scene_item(
        &self,
        scene_name: Option<&str>,
        item: Either<&str, SceneItemSpecification<'_>>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::ResetSceneItem { scene_name, item })
            .await
    }

    /// Show or hide a specified source item in a specified scene.
    pub async fn set_scene_item_render(
        &self,
        scene_item_render: SceneItemRender<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemRender(scene_item_render))
            .await
    }

    /// Deletes a scene item.
    ///
    /// - `scene`: Name of the scene the scene item belongs to. Defaults to the current scene.
    /// - `item`: Scene item to delete.
    pub async fn delete_scene_item(
        &self,
        scene: Option<&str>,
        item: SceneItemSpecification<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::DeleteSceneItem { scene, item })
            .await
    }

    /// Creates a scene item in a scene. In other words, this is how you add a source into a scene.
    pub async fn add_scene_item(&self, scene_item: AddSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::AddSceneItem>(RequestType::AddSceneItem(scene_item))
            .await
            .map(|asi| asi.item_id)
    }

    /// Duplicates a scene item.
    pub async fn duplicate_scene_item(
        &self,
        scene_item: DuplicateSceneItem<'_>,
    ) -> Result<responses::DuplicateSceneItem> {
        self.client
            .send_message(RequestType::DuplicateSceneItem(scene_item))
            .await
    }
}
