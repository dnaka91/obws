use super::Client;
use crate::{
    requests::{
        CreateSceneItem, DuplicateSceneItem, RequestType, SetSceneItemEnabled, SetSceneItemIndex,
        SetSceneItemLocked, SetSceneItemTransform,
    },
    responses, Result,
};

/// API functions related to scene items.
pub struct SceneItems<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneItems<'a> {
    /// Gets a list of all scene items in a scene.
    ///
    /// - `scene_name`: Name of the scene to get the items of.
    pub async fn get_scene_item_list(&self, scene_name: &str) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<responses::SceneItemList>(RequestType::GetSceneItemList { scene_name })
            .await
            .map(|sil| sil.scene_items)
    }

    /// Basically [`get_scene_item_list`](Self::get_scene_item_list), but for groups.
    ///
    /// Using groups at all in OBS is discouraged, as they are very broken under the hood.
    ///
    /// - `scene_name`: Name of the group to get the items of.
    pub async fn get_group_scene_item_list(
        &self,
        scene_name: &str,
    ) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<responses::SceneItemList>(RequestType::GetGroupSceneItemList {
                scene_name,
            })
            .await
            .map(|sil| sil.scene_items)
    }

    /// Searches a scene for a source, and returns its id.
    ///
    /// - `scene_name`: Name of the scene or group to search in.
    /// - `source_name`: Name of the source to find.
    pub async fn get_scene_item_id(&self, scene_name: &str, source_name: &str) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::GetSceneItemId {
                scene_name,
                source_name,
            })
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Creates a new scene item using a source.
    pub async fn create_scene_item(&self, create: CreateSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::CreateSceneItem(create))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Removes a scene item from a scene.
    ///
    /// - `scene_name`: Name of the scene the item is in.
    /// - `scene_item_id`: Numeric ID of the scene item.
    pub async fn remove_scene_item(&self, scene_name: &str, scene_item_id: i64) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveSceneItem {
                scene_name,
                scene_item_id,
            })
            .await
    }

    /// Duplicates a scene item, copying all transform and crop info.
    pub async fn duplicate_scene_item(&self, duplicate: DuplicateSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::DuplicateSceneItem(duplicate))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Gets the transform and crop info of a scene item.
    ///
    /// - `scene_name`: Name of the scene the item is in.
    /// - `scene_item_id`: Numeric ID of the scene item.
    pub async fn get_scene_item_transform(
        &self,
        scene_name: &str,
        scene_item_id: i64,
    ) -> Result<responses::SceneItemTransform> {
        self.client
            .send_message::<responses::GetSceneItemTransform>(RequestType::GetSceneItemTransform {
                scene_name,
                scene_item_id,
            })
            .await
            .map(|gsit| gsit.scene_item_transform)
    }

    /// Sets the transform and crop info of a scene item.
    pub async fn set_scene_item_transform(
        &self,
        transform: SetSceneItemTransform<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemTransform(transform))
            .await
    }

    /// Gets the enable state of a scene item.
    ///
    /// - `scene_name`: Name of the scene the item is in.
    /// - `scene_item_id`: Numeric ID of the scene item.
    pub async fn get_scene_item_enabled(
        &self,
        scene_name: &str,
        scene_item_id: i64,
    ) -> Result<bool> {
        self.client
            .send_message::<responses::SceneItemEnabled>(RequestType::GetSceneItemEnabled {
                scene_name,
                scene_item_id,
            })
            .await
            .map(|sie| sie.scene_item_enabled)
    }

    /// Sets the enable state of a scene item.
    pub async fn set_scene_item_enabled(&self, enabled: SetSceneItemEnabled<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemEnabled(enabled))
            .await
    }

    /// Gets the lock state of a scene item.
    ///
    /// - `scene_name`: Name of the scene the item is in.
    /// - `scene_item_id`: Numeric ID of the scene item.
    pub async fn get_scene_item_locked(
        &self,
        scene_name: &str,
        scene_item_id: i64,
    ) -> Result<bool> {
        self.client
            .send_message::<responses::SceneItemLocked>(RequestType::GetSceneItemLocked {
                scene_name,
                scene_item_id,
            })
            .await
            .map(|sil| sil.scene_item_locked)
    }

    /// Sets the lock state of a scene item.
    pub async fn set_scene_item_locked(&self, locked: SetSceneItemLocked<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemLocked(locked))
            .await
    }

    /// Gets the index position of a scene item in a scene.
    ///
    /// An index of 0 is at the bottom of the source list in the UI.
    ///
    /// - `scene_name`: Name of the scene the item is in.
    /// - `scene_item_id`: Numeric ID of the scene item.
    pub async fn get_scene_item_index(&self, scene_name: &str, scene_item_id: i64) -> Result<u32> {
        self.client
            .send_message::<responses::SceneItemIndex>(RequestType::GetSceneItemIndex {
                scene_name,
                scene_item_id,
            })
            .await
            .map(|sii| sii.scene_item_index)
    }

    /// Sets the index position of a scene item in a scene.
    pub async fn set_scene_item_index(&self, index: SetSceneItemIndex<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemIndex(index))
            .await
    }
}
