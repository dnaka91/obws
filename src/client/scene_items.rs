use serde::{Serialize, de::DeserializeOwned};

use super::Client;
use crate::{
    common::BlendMode,
    error::Result,
    requests::{
        scene_items::{
            CreateSceneItem, Duplicate, Id, Request, SetBlendMode, SetEnabled, SetIndex, SetLocked,
            SetPrivateSettings, SetPrivateSettingsInternal, SetTransform, Source,
        },
        scenes::SceneId,
    },
    responses::{scene_items as responses, sources as source_responses},
};

/// API functions related to scene items.
pub struct SceneItems<'a> {
    pub(super) client: &'a Client,
}

impl<'a> SceneItems<'a> {
    /// Gets a list of all scene items in a scene.
    #[doc(alias = "GetSceneItemList")]
    pub async fn list(&self, scene: SceneId<'_>) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<_, responses::SceneItemList>(Request::List { scene })
            .await
            .map(|sil| sil.scene_items)
    }

    /// Basically [`Self::list`], but for groups.
    ///
    /// Using groups at all in OBS is discouraged, as they are very broken under the hood.
    #[doc(alias = "GetGroupSceneItemList")]
    pub async fn list_group(&self, scene: SceneId<'_>) -> Result<Vec<responses::SceneItem>> {
        self.client
            .send_message::<_, responses::SceneItemList>(Request::ListGroup { scene })
            .await
            .map(|sil| sil.scene_items)
    }

    /// Searches a scene for a source, and returns its id.
    #[doc(alias = "GetSceneItemId")]
    pub async fn id(&self, get: Id<'_>) -> Result<i64> {
        self.client
            .send_message::<_, responses::SceneItemId>(Request::Id(get))
            .await
            .map(|sii| sii.id)
    }

    /// Gets the source associated with a scene item.
    #[doc(alias = "GetSceneItemSource")]
    pub async fn source(&self, get: Source<'_>) -> Result<source_responses::SourceId> {
        self.client.send_message(Request::Source(get)).await
    }

    /// Creates a new scene item using a source.
    #[doc(alias = "CreateSceneItem")]
    pub async fn create(&self, create: CreateSceneItem<'_>) -> Result<i64> {
        self.client
            .send_message::<_, responses::SceneItemId>(Request::Create(create))
            .await
            .map(|sii| sii.id)
    }

    /// Removes a scene item from a scene.
    #[doc(alias = "RemoveSceneItem")]
    pub async fn remove(&self, scene: SceneId<'_>, item_id: i64) -> Result<()> {
        self.client
            .send_message(Request::Remove { scene, item_id })
            .await
    }

    /// Duplicates a scene item, copying all transform and crop info.
    #[doc(alias = "DuplicateSceneItem")]
    pub async fn duplicate(&self, duplicate: Duplicate<'_>) -> Result<i64> {
        self.client
            .send_message::<_, responses::SceneItemId>(Request::Duplicate(duplicate))
            .await
            .map(|sii| sii.id)
    }

    /// Gets the transform and crop info of a scene item.
    #[doc(alias = "GetSceneItemTransform")]
    pub async fn transform(
        &self,
        scene: SceneId<'_>,
        item_id: i64,
    ) -> Result<responses::SceneItemTransform> {
        self.client
            .send_message::<_, responses::GetSceneItemTransform>(Request::Transform {
                scene,
                item_id,
            })
            .await
            .map(|gsit| gsit.transform)
    }

    /// Sets the transform and crop info of a scene item.
    #[doc(alias = "SetSceneItemTransform")]
    pub async fn set_transform(&self, transform: SetTransform<'_>) -> Result<()> {
        self.client
            .send_message(Request::SetTransform(transform))
            .await
    }

    /// Gets the enable state of a scene item.
    #[doc(alias = "GetSceneItemEnabled")]
    pub async fn enabled(&self, scene: SceneId<'_>, item_id: i64) -> Result<bool> {
        self.client
            .send_message::<_, responses::SceneItemEnabled>(Request::Enabled { scene, item_id })
            .await
            .map(|sie| sie.enabled)
    }

    /// Sets the enable state of a scene item.
    #[doc(alias = "SetSceneItemEnabled")]
    pub async fn set_enabled(&self, enabled: SetEnabled<'a>) -> Result<()> {
        self.client.send_message(Request::SetEnabled(enabled)).await
    }

    /// Gets the lock state of a scene item.
    #[doc(alias = "GetSceneItemLocked")]
    pub async fn locked(&self, scene: SceneId<'_>, item_id: i64) -> Result<bool> {
        self.client
            .send_message::<_, responses::SceneItemLocked>(Request::Locked { scene, item_id })
            .await
            .map(|sil| sil.locked)
    }

    /// Sets the lock state of a scene item.
    #[doc(alias = "SetSceneItemLocked")]
    pub async fn set_locked(&self, locked: SetLocked<'_>) -> Result<()> {
        self.client.send_message(Request::SetLocked(locked)).await
    }

    /// Gets the index position of a scene item in a scene.
    ///
    /// An index of 0 is at the bottom of the source list in the UI.
    #[doc(alias = "GetSceneItemIndex")]
    pub async fn index(&self, scene: SceneId<'_>, item_id: i64) -> Result<u32> {
        self.client
            .send_message::<_, responses::SceneItemIndex>(Request::Index { scene, item_id })
            .await
            .map(|sii| sii.index)
    }

    /// Sets the index position of a scene item in a scene.
    #[doc(alias = "SetSceneItemIndex")]
    pub async fn set_index(&self, index: SetIndex<'_>) -> Result<()> {
        self.client.send_message(Request::SetIndex(index)).await
    }

    /// Gets the blend mode of a scene item.
    #[doc(alias = "GetSceneItemBlendMode")]
    pub async fn blend_mode(&self, scene: SceneId<'_>, item_id: i64) -> Result<BlendMode> {
        self.client
            .send_message::<_, responses::SceneItemBlendMode>(Request::BlendMode { scene, item_id })
            .await
            .map(|sibm| sibm.blend_mode)
    }

    /// Sets the blend mode of a scene item.
    #[doc(alias = "SetSceneItemBlendMode")]
    pub async fn set_blend_mode(&self, mode: SetBlendMode<'a>) -> Result<()> {
        self.client.send_message(Request::SetBlendMode(mode)).await
    }

    /// Gets private scene item settings.
    #[doc(alias = "GetSceneItemPrivateSettings")]
    pub async fn private_settings<T>(&self, scene: SceneId<'_>, item_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message::<_, responses::SceneItemSettings<T>>(Request::PrivateSettings {
                scene,
                item_id,
            })
            .await
            .map(|sis| sis.settings)
    }

    /// Sets private scene item settings.
    #[doc(alias = "SetSceneItemPrivateSettings")]
    pub async fn set_private_settings<T>(&self, settings: SetPrivateSettings<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetPrivateSettings(SetPrivateSettingsInternal {
                scene: settings.scene,
                item_id: settings.item_id,
                settings: serde_json::to_value(settings.settings)
                    .map_err(crate::error::SerializeCustomDataError)?,
            }))
            .await
    }
}
