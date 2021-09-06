use super::Client;
use crate::{
    requests::{
        CreateSceneItem, RequestType, SetSceneItemEnabled, SetSceneItemIndex, SetSceneItemLocked,
    },
    responses, Result,
};

/// API functions related to scene items.
pub struct SceneItems<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneItems<'a> {
    pub async fn get_scene_item_list(&self, scene_name: &str) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<responses::SceneItemList>(RequestType::GetSceneItemList { scene_name })
            .await
            .map(|sil| sil.scene_items)
    }

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

    pub async fn create_scene_item(&self, create: CreateSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::CreateSceneItem(create))
            .await
            .map(|sii| sii.scene_item_id)
    }

    pub async fn remote_scene_item(&self, scene_name: &str, scene_item_id: i64) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveSceneItem {
                scene_name,
                scene_item_id,
            })
            .await
    }

    pub async fn get_scene_item_transform(
        &self,
        scene_name: &str,
        scene_item_id: i64,
    ) -> Result<responses::SceneItemTransform> {
        self.client
            .send_message(RequestType::GetSceneItemTransform {
                scene_name,
                scene_item_id,
            })
            .await
    }

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

    pub async fn set_scene_item_enabled(&self, enabled: SetSceneItemEnabled<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemEnabled(enabled))
            .await
    }

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

    pub async fn set_scene_item_locked(&self, locked: SetSceneItemLocked<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemLocked(locked))
            .await
    }

    pub async fn get_scene_item_index(&self, scene_name: &str, scene_item_id: i64) -> Result<u32> {
        self.client
            .send_message::<responses::SceneItemIndex>(RequestType::GetSceneItemIndex {
                scene_name,
                scene_item_id,
            })
            .await
            .map(|sii| sii.scene_item_index)
    }

    pub async fn set_scene_item_index(&self, index: SetSceneItemIndex<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemIndex(index))
            .await
    }
}
