use anyhow::Result;
use obws::requests::SetSceneSceneTransitionOverride;
use time::Duration;

use crate::common::{self, TEST_SCENE, TEST_SCENE_CREATE, TEST_SCENE_RENAME, TEST_TRANSITION};

#[tokio::test]
async fn scenes() -> Result<()> {
    let client = common::new_client().await?;
    let ui = client.ui();
    let client = client.scenes();

    ui.set_studio_mode_enabled(true).await?;

    let scenes = client.list().await?.scenes;
    client.list_groups().await?;

    let current = client.current_program_scene().await?;
    let other = &scenes.iter().find(|s| s.name != current).unwrap().name;
    client.set_current_program_scene(other).await?;
    client.set_current_program_scene(&current).await?;

    let current = client.current_preview_scene().await?;
    let other = &scenes.iter().find(|s| s.name != current).unwrap().name;
    client.set_current_preview_scene(other).await?;
    client.set_current_preview_scene(&current).await?;

    client.set_name(TEST_SCENE, TEST_SCENE_RENAME).await?;
    client.set_name(TEST_SCENE_RENAME, TEST_SCENE).await?;

    client.create(TEST_SCENE_CREATE).await?;
    client.remove(TEST_SCENE_CREATE).await?;

    let to = client.transition_override(TEST_SCENE).await?;
    client
        .set_transition_override(SetSceneSceneTransitionOverride {
            scene: TEST_SCENE,
            transition: Some(TEST_TRANSITION),
            duration: Some(Duration::seconds(5)),
        })
        .await?;
    client
        .set_transition_override(SetSceneSceneTransitionOverride {
            scene: TEST_SCENE,
            transition: to.name.as_deref(),
            duration: to.duration,
        })
        .await?;

    ui.set_studio_mode_enabled(false).await?;

    Ok(())
}
