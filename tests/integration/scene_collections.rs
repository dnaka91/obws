use anyhow::Result;
use obws::responses::scene_collections::SceneCollections;
use serde_json::json;
use test_log::test;

use crate::common;

#[test(tokio::test)]
async fn scene_collections() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.scene_collections();

    server.expect(
        "GetSceneCollectionList",
        json!(null),
        json!({
            "currentSceneCollectionName": "main",
            "sceneCollections": ["main", "other"],
        }),
    );

    let SceneCollections {
        current,
        collections,
    } = client.list().await?;

    server.expect(
        "GetSceneCollectionList",
        json!(null),
        json!({
            "currentSceneCollectionName": "main",
            "sceneCollections": ["main", "other"],
        }),
    );

    client.current().await?;
    let other = collections.iter().find(|sc| *sc != &current).unwrap();

    server.expect(
        "SetCurrentSceneCollection",
        json!({"sceneCollectionName": "other"}),
        json!(null),
    );

    client.set_current(other).await?;

    server.expect(
        "CreateSceneCollection",
        json!({"sceneCollectionName": "new"}),
        json!(null),
    );

    client.create("new").await?;

    server.stop().await
}
