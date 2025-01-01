use anyhow::Result;
use obws::requests::scenes::SetTransitionOverride;
use serde_json::json;
use test_log::test;
use time::Duration;
use uuid::Uuid;

use crate::common::{self, TEST_SCENE, TEST_SCENE_CREATE, TEST_SCENE_RENAME, TEST_TRANSITION};

#[test(tokio::test)]
async fn scenes() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let ui = client.ui();
    let client = client.scenes();

    server.expect(
        "SetStudioModeEnabled",
        json!({"studioModeEnabled": true}),
        json!(null),
    );

    ui.set_studio_mode_enabled(true).await?;

    server.expect(
        "GetSceneList",
        json!(null),
        json!({
            "currentProgramSceneName": "main",
            "currentProgramSceneUuid": Uuid::new_v8([1; 16]),
            "currentPreviewSceneName": "other",
            "currentPreviewSceneUuid": Uuid::new_v8([2; 16]),
            "scenes": [
                {
                    "sceneName": "main",
                    "sceneUuid":  Uuid::new_v8([1; 16]),
                    "sceneIndex": 0,
                },
                {
                    "sceneName": "other",
                    "sceneUuid": Uuid::new_v8([2; 16]),
                    "sceneIndex": 1,
                },
            ],
        }),
    );

    let scenes = client.list().await?.scenes;

    server.expect("GetGroupList", json!(null), json!({"groups": ["one"]}));

    client.list_groups().await?;

    server.expect(
        "GetCurrentProgramScene",
        json!(null),
        json!({
            "sceneName": "main",
            "sceneUuid": Uuid::new_v8([1; 16]),
        }),
    );

    let current = client.current_program_scene().await?;

    server.expect(
        "SetCurrentProgramScene",
        json!({"sceneUuid": Uuid::new_v8([2; 16])}),
        json!(null),
    );

    let other = &scenes.iter().find(|s| s.id != current.id).unwrap().id;
    client.set_current_program_scene(other).await?;

    server.expect(
        "GetCurrentPreviewScene",
        json!(null),
        json!({
            "sceneName": "main",
            "sceneUuid": Uuid::new_v8([2; 16]),
        }),
    );

    let current = client.current_preview_scene().await?;

    server.expect(
        "SetCurrentPreviewScene",
        json!({"sceneUuid": Uuid::new_v8([1; 16])}),
        json!(null),
    );

    let other = &scenes.iter().find(|s| s.id != current.id).unwrap().id;
    client.set_current_preview_scene(other).await?;

    server.expect(
        "SetSceneName",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "newSceneName": "OBWS-TEST-Scene-Renamed",
        }),
        json!(null),
    );

    client
        .set_name(TEST_SCENE, TEST_SCENE_RENAME.as_name().unwrap())
        .await?;

    server.expect(
        "CreateScene",
        json!({"sceneName": "OBWS-TEST-Scene-Created"}),
        json!({"sceneUuid": Uuid::new_v8([3; 16])}),
    );

    client.create(TEST_SCENE_CREATE.as_name().unwrap()).await?;

    server.expect(
        "RemoveScene",
        json!({"sceneName": "OBWS-TEST-Scene-Created"}),
        json!(null),
    );

    client.remove(TEST_SCENE_CREATE).await?;

    server.expect(
        "GetSceneSceneTransitionOverride",
        json!({"sceneName": "OBWS-TEST-Scene"}),
        json!({
            "transitionName": "some-transition",
            "transitionDuration": 500,
        }),
    );

    client.transition_override(TEST_SCENE).await?;

    server.expect(
        "SetSceneSceneTransitionOverride",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "transitionName": "OBWS-TEST-Transition",
            "transitionDuration": 5000,
        }),
        json!(null),
    );

    client
        .set_transition_override(SetTransitionOverride {
            scene: TEST_SCENE,
            transition: Some(TEST_TRANSITION),
            duration: Some(Duration::seconds(5)),
        })
        .await?;

    server.expect(
        "SetStudioModeEnabled",
        json!({"studioModeEnabled": false}),
        json!(null),
    );

    ui.set_studio_mode_enabled(false).await?;

    server.stop().await
}
