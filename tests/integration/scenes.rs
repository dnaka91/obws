use anyhow::Result;
use obws::requests::scenes::SetTransitionOverride;
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
    let other = &scenes.iter().find(|s| s.id != current.id).unwrap().id;
    client.set_current_program_scene(other).await?;
    client.set_current_program_scene(current.id).await?;

    let current = client.current_preview_scene().await?;
    let other = &scenes.iter().find(|s| s.id != current.id).unwrap().id;
    client.set_current_preview_scene(other).await?;
    client.set_current_preview_scene(current.id).await?;

    client
        .set_name(TEST_SCENE, TEST_SCENE_RENAME.as_name().unwrap())
        .await?;
    client
        .set_name(TEST_SCENE_RENAME, TEST_SCENE.as_name().unwrap())
        .await?;

    client.create(TEST_SCENE_CREATE.as_name().unwrap()).await?;
    client.remove(TEST_SCENE_CREATE).await?;

    let to = client.transition_override(TEST_SCENE).await?;
    client
        .set_transition_override(SetTransitionOverride {
            scene: TEST_SCENE,
            transition: Some(TEST_TRANSITION),
            duration: Some(Duration::seconds(5)),
        })
        .await?;
    client
        .set_transition_override(SetTransitionOverride {
            scene: TEST_SCENE,
            transition: to.name.as_deref(),
            duration: to.duration,
        })
        .await?;

    ui.set_studio_mode_enabled(false).await?;

    Ok(())
}
