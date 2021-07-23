#![cfg(feature = "test-integration")]

use anyhow::Result;
use common::{TEST_SCENE, TEST_SCENE_CREATE, TEST_SCENE_RENAME};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let general = client.general();
    let client = client.scenes();

    general.set_studio_mode_enabled(true).await?;

    let scenes = client.get_scene_list().await?.scenes;

    let current = client.get_current_program_scene().await?;
    let other = &scenes
        .iter()
        .find(|s| s.scene_name != current)
        .unwrap()
        .scene_name;
    client.set_current_program_scene(other).await?;
    client.set_current_program_scene(&current).await?;

    let current = client.get_current_preview_scene().await?.unwrap();
    let other = &scenes
        .iter()
        .find(|s| s.scene_name != current)
        .unwrap()
        .scene_name;
    client.set_current_preview_scene(other).await?;
    client.set_current_preview_scene(&current).await?;

    client.set_scene_name(TEST_SCENE, TEST_SCENE_RENAME).await?;
    client.set_scene_name(TEST_SCENE_RENAME, TEST_SCENE).await?;

    client.create_scene(TEST_SCENE_CREATE).await?;
    client.remove_scene(TEST_SCENE_CREATE).await?;

    general.set_studio_mode_enabled(false).await?;

    Ok(())
}
