use anyhow::Result;
use obws::{
    common::{BlendMode, BoundsType},
    requests::scene_items::{
        Bounds, CreateSceneItem, Duplicate, Id, SceneItemTransform, SetBlendMode, SetEnabled,
        SetIndex, SetLocked, SetTransform,
    },
};

use crate::common::{self, TEST_GROUP, TEST_SCENE, TEST_SCENE_2, TEST_TEXT};

#[tokio::test]
async fn scene_items() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.scene_items();

    client.list(TEST_SCENE).await?;
    client.list_group(TEST_GROUP).await?;

    let test_text_id = client
        .id(Id {
            scene: TEST_SCENE,
            source: TEST_TEXT.as_name().unwrap(),
            search_offset: None,
        })
        .await?;

    let id = client
        .duplicate(Duplicate {
            scene: TEST_SCENE,
            item_id: test_text_id,
            destination: Some(TEST_SCENE_2.into()),
        })
        .await?;
    client.remove(TEST_SCENE_2, id).await?;

    let id = client
        .create(CreateSceneItem {
            scene: TEST_SCENE_2,
            source: TEST_TEXT.as_source(),
            enabled: Some(true),
        })
        .await?;
    client.remove(TEST_SCENE_2, id).await?;

    let transform = client.transform(TEST_SCENE, test_text_id).await?;
    client
        .set_transform(SetTransform {
            scene: TEST_SCENE,
            item_id: test_text_id,
            transform: SceneItemTransform {
                bounds: Some(Bounds {
                    r#type: Some(BoundsType::Stretch),
                    ..Bounds::default()
                }),
                ..SceneItemTransform::default()
            },
        })
        .await?;
    client
        .set_transform(SetTransform {
            scene: TEST_SCENE,
            item_id: test_text_id,
            transform: SceneItemTransform {
                bounds: Some(Bounds {
                    r#type: Some(transform.bounds_type),
                    ..Bounds::default()
                }),
                ..SceneItemTransform::default()
            },
        })
        .await?;

    let enabled = client.enabled(TEST_SCENE, test_text_id).await?;
    client
        .set_enabled(SetEnabled {
            scene: TEST_SCENE,
            item_id: test_text_id,
            enabled: !enabled,
        })
        .await?;
    client
        .set_enabled(SetEnabled {
            scene: TEST_SCENE,
            item_id: test_text_id,
            enabled,
        })
        .await?;

    let locked = client.locked(TEST_SCENE, test_text_id).await?;
    client
        .set_locked(SetLocked {
            scene: TEST_SCENE,
            item_id: test_text_id,
            locked: !locked,
        })
        .await?;
    client
        .set_locked(SetLocked {
            scene: TEST_SCENE,
            item_id: test_text_id,
            locked,
        })
        .await?;

    let index = client.index(TEST_SCENE, test_text_id).await?;
    client
        .set_index(SetIndex {
            scene: TEST_SCENE,
            item_id: test_text_id,
            index: 0,
        })
        .await?;
    client
        .set_index(SetIndex {
            scene: TEST_SCENE,
            item_id: test_text_id,
            index,
        })
        .await?;

    let mode = client.blend_mode(TEST_SCENE, test_text_id).await?;
    assert_eq!(BlendMode::Normal, mode);
    client
        .set_blend_mode(SetBlendMode {
            scene: TEST_SCENE,
            item_id: test_text_id,
            mode: BlendMode::Multiply,
        })
        .await?;
    client
        .set_blend_mode(SetBlendMode {
            scene: TEST_SCENE,
            item_id: test_text_id,
            mode,
        })
        .await?;

    let _settings = client
        .private_settings::<serde_json::Value>(TEST_SCENE, test_text_id)
        .await?;

    // TODO: Currently obs-websocket doesn't accept empty objects `{}`, and this fails as our
    // test scene item doesn't have any private settings.
    //
    // client
    //     .set_private_settings(SetPrivateSettings {
    //         scene: TEST_SCENE,
    //         item_id: test_text_id,
    //         settings: &settings,
    //     })
    //     .await?;

    Ok(())
}
