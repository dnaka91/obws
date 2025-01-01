use anyhow::Result;
use obws::{
    common::{BlendMode, BoundsType},
    requests::scene_items::{
        Bounds, CreateSceneItem, Duplicate, Id, SceneItemTransform, SetBlendMode, SetEnabled,
        SetIndex, SetLocked, SetPrivateSettings, SetTransform, Source,
    },
};
use serde_json::json;
use test_log::test;
use uuid::Uuid;

use crate::common::{self, TEST_GROUP, TEST_SCENE, TEST_SCENE_2, TEST_TEXT};

#[test(tokio::test)]
async fn scene_items() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.scene_items();

    server.expect(
        "GetSceneItemList",
        json!({"sceneName": "OBWS-TEST-Scene"}),
        json!({"sceneItems": []}),
    );

    client.list(TEST_SCENE).await?;

    server.expect(
        "GetGroupSceneItemList",
        json!({"sceneName": "OBWS-TEST-Group"}),
        json!({"sceneItems": []}),
    );

    client.list_group(TEST_GROUP).await?;

    server.expect(
        "GetSceneItemId",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sourceName": "OBWS-TEST-Text",
        }),
        json!({"sceneItemId": 1}),
    );

    let test_text_id = client
        .id(Id {
            scene: TEST_SCENE,
            source: TEST_TEXT.as_name().unwrap(),
            search_offset: None,
        })
        .await?;

    server.expect(
        "GetSceneItemSource",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({
            "sourceName": "test-source",
            "sourceUuid": Uuid::nil(),
        }),
    );

    client
        .source(Source {
            scene: TEST_SCENE,
            item_id: test_text_id,
        })
        .await?;

    server.expect(
        "DuplicateSceneItem",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "destinationSceneName": "OBWS-TEST-Scene2",
        }),
        json!({"sceneItemId": 2}),
    );

    client
        .duplicate(Duplicate {
            scene: TEST_SCENE,
            item_id: test_text_id,
            destination: Some(TEST_SCENE_2.into()),
        })
        .await?;

    server.expect(
        "CreateSceneItem",
        json!({
            "sceneName": "OBWS-TEST-Scene2",
            "sourceName": "OBWS-TEST-Text",
            "sceneItemEnabled": true,
        }),
        json!({"sceneItemId": 3}),
    );

    let id = client
        .create(CreateSceneItem {
            scene: TEST_SCENE_2,
            source: TEST_TEXT.as_source(),
            enabled: Some(true),
        })
        .await?;

    server.expect(
        "RemoveSceneItem",
        json!({
            "sceneName": "OBWS-TEST-Scene2",
            "sceneItemId": 3,
        }),
        json!(null),
    );

    client.remove(TEST_SCENE_2, id).await?;

    server.expect(
        "GetSceneItemTransform",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({
            "sceneItemTransform": {
                "sourceWidth": 1920,
                "sourceHeight": 1080,
                "positionX": 5,
                "positionY": 10,
                "rotation": 30.0,
                "scaleX": 1.1,
                "scaleY": 1.2,
                "width": 800,
                "height": 600,
                "alignment": 0b1111,
                "boundsType": "OBS_BOUNDS_SCALE_OUTER",
                "boundsAlignment": 0,
                "boundsWidth": 200,
                "boundsHeight": 100,
                "cropLeft": 1,
                "cropRight": 2,
                "cropTop": 3,
                "cropBottom": 4,
                "cropToBounds": false,
            },
        }),
    );

    client.transform(TEST_SCENE, test_text_id).await?;

    server.expect(
        "SetSceneItemTransform",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "sceneItemTransform": {
                "boundsType": "OBS_BOUNDS_STRETCH",
            },
        }),
        json!(null),
    );

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

    server.expect(
        "GetSceneItemEnabled",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({"sceneItemEnabled": true}),
    );

    let enabled = client.enabled(TEST_SCENE, test_text_id).await?;

    server.expect(
        "SetSceneItemEnabled",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "sceneItemEnabled": false,
        }),
        json!(null),
    );

    client
        .set_enabled(SetEnabled {
            scene: TEST_SCENE,
            item_id: test_text_id,
            enabled: !enabled,
        })
        .await?;

    server.expect(
        "GetSceneItemLocked",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({"sceneItemLocked": false}),
    );

    let locked = client.locked(TEST_SCENE, test_text_id).await?;

    server.expect(
        "SetSceneItemLocked",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "sceneItemLocked": true,
        }),
        json!(null),
    );

    client
        .set_locked(SetLocked {
            scene: TEST_SCENE,
            item_id: test_text_id,
            locked: !locked,
        })
        .await?;

    server.expect(
        "GetSceneItemIndex",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({"sceneItemIndex": 1}),
    );

    client.index(TEST_SCENE, test_text_id).await?;

    server.expect(
        "SetSceneItemIndex",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "sceneItemIndex": 0,
        }),
        json!(null),
    );

    client
        .set_index(SetIndex {
            scene: TEST_SCENE,
            item_id: test_text_id,
            index: 0,
        })
        .await?;

    server.expect(
        "GetSceneItemBlendMode",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({"sceneItemBlendMode": "OBS_BLEND_NORMAL"}),
    );

    let mode = client.blend_mode(TEST_SCENE, test_text_id).await?;
    assert_eq!(BlendMode::Normal, mode);

    server.expect(
        "SetSceneItemBlendMode",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "sceneItemBlendMode": "OBS_BLEND_MULTIPLY",
        }),
        json!(null),
    );

    client
        .set_blend_mode(SetBlendMode {
            scene: TEST_SCENE,
            item_id: test_text_id,
            mode: BlendMode::Multiply,
        })
        .await?;

    server.expect(
        "GetSceneItemPrivateSettings",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
        }),
        json!({"sceneItemSettings": {}}),
    );

    let settings = client
        .private_settings::<serde_json::Value>(TEST_SCENE, test_text_id)
        .await?;

    server.expect(
        "SetSceneItemPrivateSettings",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "sceneItemId": 1,
            "sceneItemSettings": {},
        }),
        json!(null),
    );

    client
        .set_private_settings(SetPrivateSettings {
            scene: TEST_SCENE,
            item_id: test_text_id,
            settings: &settings,
        })
        .await?;

    server.stop().await
}
