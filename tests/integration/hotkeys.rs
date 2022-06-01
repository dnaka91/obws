use anyhow::Result;
use obws::requests::hotkeys::KeyModifiers;

use crate::common;

#[tokio::test]
async fn general() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.hotkeys();

    client.list_hotkeys().await?;
    client.trigger_hotkey_by_name("ReplayBuffer.Save").await?;
    client
        .trigger_hotkey_by_key_sequence("OBS_KEY_P", KeyModifiers::default())
        .await?;

    Ok(())
}
