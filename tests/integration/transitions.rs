use anyhow::Result;

use crate::common::{self, TEST_TRANSITION};

#[tokio::test]
async fn transitions() -> Result<()> {
    let client = common::new_client().await?;
    let ui = client.ui();
    let client = client.transitions();

    client.get_transition_kind_list().await?;
    client.get_scene_transition_list().await?;

    client.set_current_scene_transition(TEST_TRANSITION).await?;
    let transition = client.get_current_scene_transition().await?;

    client
        .set_current_scene_transition_duration(transition.transition_duration.unwrap())
        .await?;
    client
        .set_current_scene_transition_settings(transition.transition_settings.unwrap(), None)
        .await?;
    client.get_current_scene_transition_cursor().await?;

    ui.set_studio_mode_enabled(true).await?;

    client.trigger_studio_mode_transition().await?;
    client.set_tbar_position(0.5, None).await?;

    ui.set_studio_mode_enabled(false).await?;

    Ok(())
}
