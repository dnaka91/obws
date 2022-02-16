use anyhow::Result;
use obws::requests::SetSceneSceneTransitionOverride;
use time::Duration;

use crate::common::{self, TEST_SCENE, TEST_SCENE_CREATE, TEST_SCENE_RENAME, TEST_TRANSITION};

#[tokio::test]
async fn scenes() -> Result<()> {
    let client = common::new_client().await?;
    let general = client.general();
    let client = client.scenes();

    general.set_studio_mode_enabled(true).await?;

    let scenes = client.get_scene_list().await?.scenes;
    client.get_group_list().await?;

    let current = client.get_current_program_scene().await?;
    let other = &scenes
        .iter()
        .find(|s| s.scene_name != current)
        .unwrap()
        .scene_name;
    client.set_current_program_scene(other).await?;
    client.set_current_program_scene(&current).await?;

    let current = client.get_current_preview_scene().await?;
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

    let to = client
        .get_scene_scene_transition_override(TEST_SCENE)
        .await?;
    client
        .set_scene_scene_transition_override(SetSceneSceneTransitionOverride {
            scene_name: TEST_SCENE,
            transition_name: Some(TEST_TRANSITION),
            transition_duration: Some(Duration::seconds(5)),
        })
        .await?;
    client
        .set_scene_scene_transition_override(SetSceneSceneTransitionOverride {
            scene_name: TEST_SCENE,
            transition_name: to.transition_name.as_deref(),
            transition_duration: to.transition_duration,
        })
        .await?;

    general.set_studio_mode_enabled(false).await?;

    Ok(())
}
