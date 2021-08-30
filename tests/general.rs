#![cfg(feature = "test-integration")]

use anyhow::Result;
use obws::{events::Event, requests::KeyModifiers};
use serde::Serialize;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.general();

    tokio::pin!(events);

    client.get_version().await?;
    client
        .broadcast_custom_event(&CustomEvent { hello: "world!" })
        .await?;
    wait_for!(events, Event::CustomEvent(_));
    client.get_stats().await?;

    client.get_hotkey_list().await?;
    client.trigger_hotkey_by_name("ReplayBuffer.Save").await?;
    client
        .trigger_hotkey_by_key_sequence("OBS_KEY_P", KeyModifiers::default())
        .await?;

    let enabled = client.get_studio_mode_enabled().await?;
    client.set_studio_mode_enabled(!enabled).await?;
    client.set_studio_mode_enabled(enabled).await?;

    Ok(())
}

#[derive(Serialize)]
struct CustomEvent<'a> {
    hello: &'a str,
}
