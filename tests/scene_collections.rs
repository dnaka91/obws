#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::{Context, Result};
use tokio::time;

use crate::common::TEST_COLLECTION;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.scene_collections();

    let other = client
        .list_scene_collections()
        .await?
        .into_iter()
        .find(|sc| sc.sc_name != TEST_COLLECTION)
        .context("only the test scene collection exists, but another is needed for tests")?
        .sc_name;

    let original = client.get_current_scene_collection().await?;
    client.set_current_scene_collection(&other).await?;

    // Give OBS some time to load the scene collection
    time::sleep(Duration::from_secs(1)).await;

    client.set_current_scene_collection(&original).await?;

    Ok(())
}
