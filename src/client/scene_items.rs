use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{
    requests::{
        CreateSceneItem, DuplicateSceneItem, GetSceneItemId, RequestType, SetSceneItemEnabled,
        SetSceneItemIndex, SetSceneItemLocked, SetSceneItemPrivateSettings,
        SetSceneItemPrivateSettingsInternal, SetSceneItemTransform,
    },
    responses, Error, Result,
};

/// API functions related to scene items.
pub struct SceneItems<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneItems<'a> {
    /// Gets a list of all scene items in a scene.
    pub async fn list(&self, scene: &str) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<responses::SceneItemList>(RequestType::GetSceneItemList { scene })
            .await
            .map(|sil| sil.scene_items)
    }

    /// Basically [`Self::list`], but for groups.
    ///
    /// Using groups at all in OBS is discouraged, as they are very broken under the hood.
    pub async fn list_group(&self, scene: &str) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<responses::SceneItemList>(RequestType::GetGroupSceneItemList { scene })
            .await
            .map(|sil| sil.scene_items)
    }

    /// Searches a scene for a source, and returns its id.
    pub async fn id(&self, get: GetSceneItemId<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::GetSceneItemId(get))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Creates a new scene item using a source.
    pub async fn create(&self, create: CreateSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::CreateSceneItem(create))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Removes a scene item from a scene.
    pub async fn remove(&self, scene: &str, item_id: i64) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveSceneItem { scene, item_id })
            .await
    }

    /// Duplicates a scene item, copying all transform and crop info.
    pub async fn duplicate(&self, duplicate: DuplicateSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::DuplicateSceneItem(duplicate))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Gets the transform and crop info of a scene item.
    pub async fn transform(
        &self,
        scene: &str,
        item_id: i64,
    ) -> Result<responses::SceneItemTransform> {
        self.client
            .send_message::<responses::GetSceneItemTransform>(RequestType::GetSceneItemTransform {
                scene,
                item_id,
            })
            .await
            .map(|gsit| gsit.scene_item_transform)
    }

    /// Sets the transform and crop info of a scene item.
    pub async fn set_transform(&self, transform: SetSceneItemTransform<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemTransform(transform))
            .await
    }

    /// Gets the enable state of a scene item.
    pub async fn enabled(&self, scene: &str, item_id: i64) -> Result<bool> {
        self.client
            .send_message::<responses::SceneItemEnabled>(RequestType::GetSceneItemEnabled {
                scene,
                item_id,
            })
            .await
            .map(|sie| sie.scene_item_enabled)
    }

    /// Sets the enable state of a scene item.
    pub async fn set_enabled(&self, enabled: SetSceneItemEnabled<'a>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemEnabled(enabled))
            .await
    }

    /// Gets the lock state of a scene item.
    pub async fn locked(&self, scene: &str, item_id: i64) -> Result<bool> {
        self.client
            .send_message::<responses::SceneItemLocked>(RequestType::GetSceneItemLocked {
                scene,
                item_id,
            })
            .await
            .map(|sil| sil.scene_item_locked)
    }

    /// Sets the lock state of a scene item.
    pub async fn set_locked(&self, locked: SetSceneItemLocked<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemLocked(locked))
            .await
    }

    /// Gets the index position of a scene item in a scene.
    ///
    /// An index of 0 is at the bottom of the source list in the UI.
    pub async fn index(&self, scene: &str, item_id: i64) -> Result<u32> {
        self.client
            .send_message::<responses::SceneItemIndex>(RequestType::GetSceneItemIndex {
                scene,
                item_id,
            })
            .await
            .map(|sii| sii.scene_item_index)
    }

    /// Sets the index position of a scene item in a scene.
    pub async fn set_index(&self, index: SetSceneItemIndex<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSceneItemIndex(index))
            .await
    }

    /// Gets private scene item settings.
    pub async fn private_settings<T>(&self, scene: &str, item_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message::<responses::SceneItemSettings<T>>(
                RequestType::GetSceneItemPrivateSettings { scene, item_id },
            )
            .await
            .map(|sis| sis.scene_item_settings)
    }

    /// Sets private scene item settings.
    pub async fn set_private_settings<T>(
        &self,
        settings: SetSceneItemPrivateSettings<'_, T>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetSceneItemPrivateSettings(
                SetSceneItemPrivateSettingsInternal {
                    scene: settings.scene,
                    item_id: settings.item_id,
                    settings: serde_json::to_value(&settings.settings)
                        .map_err(Error::SerializeCustomData)?,
                },
            ))
            .await
    }
}
