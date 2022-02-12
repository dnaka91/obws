use anyhow::Result;
use obws::{
    common::BoundsType,
    requests::{
        CreateSceneItem, DuplicateSceneItem, SceneItemTransform, SetSceneItemEnabled,
        SetSceneItemIndex, SetSceneItemLocked, SetSceneItemTransform,
    },
};

use crate::common::{self, TEST_GROUP, TEST_SCENE, TEST_SCENE_2, TEST_TEXT};

#[tokio::test]
async fn scene_items() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.scene_items();

    client.get_scene_item_list(TEST_SCENE).await?;
    client.get_group_scene_item_list(TEST_GROUP).await?;

    let test_text_id = client.get_scene_item_id(TEST_SCENE, TEST_TEXT).await?;

    let id = client
        .duplicate_scene_item(DuplicateSceneItem {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            destination_scene_name: Some(TEST_SCENE_2),
        })
        .await?;
    client.remove_scene_item(TEST_SCENE_2, id).await?;

    let id = client
        .create_scene_item(CreateSceneItem {
            scene_name: TEST_SCENE_2,
            source_name: TEST_TEXT,
            scene_item_enabled: Some(true),
        })
        .await?;
    client.remove_scene_item(TEST_SCENE_2, id).await?;

    let transform = client
        .get_scene_item_transform(TEST_SCENE, test_text_id)
        .await?;
    client
        .set_scene_item_transform(SetSceneItemTransform {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_transform: SceneItemTransform {
                bounds_type: Some(BoundsType::Stretch),
                ..SceneItemTransform::default()
            },
        })
        .await?;
    client
        .set_scene_item_transform(SetSceneItemTransform {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_transform: SceneItemTransform {
                bounds_type: Some(transform.bounds_type),
                ..SceneItemTransform::default()
            },
        })
        .await?;

    let enabled = client
        .get_scene_item_enabled(TEST_SCENE, test_text_id)
        .await?;
    client
        .set_scene_item_enabled(SetSceneItemEnabled {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_enabled: !enabled,
        })
        .await?;
    client
        .set_scene_item_enabled(SetSceneItemEnabled {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_enabled: enabled,
        })
        .await?;

    let locked = client
        .get_scene_item_locked(TEST_SCENE, test_text_id)
        .await?;
    client
        .set_scene_item_locked(SetSceneItemLocked {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_locked: !locked,
        })
        .await?;
    client
        .set_scene_item_locked(SetSceneItemLocked {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_locked: locked,
        })
        .await?;

    let index = client
        .get_scene_item_index(TEST_SCENE, test_text_id)
        .await?;
    client
        .set_scene_item_index(SetSceneItemIndex {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_index: 0,
        })
        .await?;
    client
        .set_scene_item_index(SetSceneItemIndex {
            scene_name: TEST_SCENE,
            scene_item_id: test_text_id,
            scene_item_index: index,
        })
        .await?;

    Ok(())
}
