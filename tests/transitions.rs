#![cfg(feature = "test-integration")]

use anyhow::Result;

use common::TEST_TRANSITION_2;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let studio_mode = client.studio_mode();
    let client = client.transitions();

    client.get_transition_list().await?;
    let original = client.get_current_transition().await?.name;
    client.set_current_transition(TEST_TRANSITION_2).await?;
    client.set_current_transition(&original).await?;

    let original = client.get_transition_duration().await?;
    client.set_transition_duration(original * 2).await?;
    client.set_transition_duration(original).await?;

    client.get_transition_position().await?;
    let settings = client.get_transition_settings(TEST_TRANSITION_2).await?;
    client
        .set_transition_settings(TEST_TRANSITION_2, &settings)
        .await?;

    studio_mode.enable_studio_mode().await?;
    client.set_t_bar_position(0.5, Some(false)).await?;
    client.set_t_bar_position(0.0, Some(false)).await?;
    client.release_t_bar().await?;
    studio_mode.disable_studio_mode().await?;

    Ok(())
}
