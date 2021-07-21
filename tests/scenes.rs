#![cfg(feature = "test-integration")]

use anyhow::Result;
use chrono::Duration;
use obws::requests::{SceneItem, SceneTransitionOverride};

use crate::common::{
    TEST_BROWSER, TEST_MEDIA, TEST_SCENE, TEST_SCENE_2, TEST_TRANSITION, TEXT_SOURCE, TEXT_SOURCE_2,
};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.scenes();

    let original = client.get_current_scene().await?.name;
    client.set_current_scene(TEST_SCENE_2).await?;
    client.set_current_scene(&original).await?;

    client.get_scene_list().await?;

    // TODO: Currently no way of deleting scenes so we skip this to not
    // fill up OBS with random scenes on every run.
    // client.create_scene("__TEMP").await?;

    client
        .reorder_scene_items(
            Some(TEST_SCENE),
            &[
                SceneItem {
                    id: None,
                    name: Some(TEXT_SOURCE_2),
                },
                SceneItem {
                    id: None,
                    name: Some(TEXT_SOURCE),
                },
                SceneItem {
                    id: None,
                    name: Some(TEST_BROWSER),
                },
                SceneItem {
                    id: None,
                    name: Some(TEST_MEDIA),
                },
            ],
        )
        .await?;
    client
        .reorder_scene_items(
            Some(TEST_SCENE),
            &[
                SceneItem {
                    id: None,
                    name: Some(TEXT_SOURCE),
                },
                SceneItem {
                    id: None,
                    name: Some(TEXT_SOURCE_2),
                },
                SceneItem {
                    id: None,
                    name: Some(TEST_BROWSER),
                },
                SceneItem {
                    id: None,
                    name: Some(TEST_MEDIA),
                },
            ],
        )
        .await?;

    client
        .set_scene_transition_override(SceneTransitionOverride {
            scene_name: TEST_SCENE,
            transition_name: TEST_TRANSITION,
            transition_duration: Some(Duration::milliseconds(10)),
        })
        .await?;
    client.get_scene_transition_override(TEST_SCENE).await?;
    client.remove_scene_transition_override(TEST_SCENE).await?;

    Ok(())
}
