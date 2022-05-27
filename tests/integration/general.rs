use anyhow::Result;
use obws::{events::Event, requests::KeyModifiers};
use serde::Serialize;

use crate::{common, wait_for};

#[tokio::test]
async fn general() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.general();

    tokio::pin!(events);

    client.version().await?;
    client
        .broadcast_custom_event(&CustomEvent { hello: "world!" })
        .await?;
    wait_for!(events, Event::CustomEvent(_));
    client.stats().await?;

    client.list_hotkeys().await?;
    client.trigger_hotkey_by_name("ReplayBuffer.Save").await?;
    client
        .trigger_hotkey_by_key_sequence("OBS_KEY_P", KeyModifiers::default())
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct CustomEvent<'a> {
    hello: &'a str,
}
