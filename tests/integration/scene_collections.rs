use std::time::Duration;

use anyhow::Result;
use obws::responses::scene_collections::SceneCollections;
use tokio::time;

use crate::common;

#[tokio::test]
async fn scene_collections() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.scene_collections();

    let SceneCollections {
        current,
        collections,
    } = client.list().await?;
    client.current().await?;
    let other = collections.iter().find(|sc| *sc != &current).unwrap();
    client.set_current(other).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.set_current(&current).await?;
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
