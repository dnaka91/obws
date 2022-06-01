use anyhow::Result;
use obws::requests::hotkeys::KeyModifiers;

use crate::common;

#[tokio::test]
async fn hotkeys() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.hotkeys();

    client.list().await?;
    client.trigger_by_name("ReplayBuffer.Save").await?;
    client
        .trigger_by_sequence("OBS_KEY_P", KeyModifiers::default())
        .await?;

    Ok(())
}
