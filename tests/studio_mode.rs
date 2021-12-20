#![cfg(feature = "test-integration")]

use anyhow::Result;
use obws::requests::Transition;
use time::Duration;

use crate::common::{TEST_SCENE_2, TEST_TRANSITION};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.studio_mode();

    client.get_studio_mode_status().await?;
    client.enable_studio_mode().await?;

    let original = client.get_preview_scene().await?.name;
    client.set_preview_scene(TEST_SCENE_2).await?;
    client.set_preview_scene(&original).await?;

    client
        .transition_to_program(Some(Transition {
            name: TEST_TRANSITION,
            duration: Some(Duration::milliseconds(10)),
        }))
        .await?;

    client.disable_studio_mode().await?;
    client.toggle_studio_mode().await?;
    client.toggle_studio_mode().await?;

    Ok(())
}
