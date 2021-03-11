#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use futures_util::{pin_mut, StreamExt};
use obws::events::{Event, EventType};
use tokio::time;

#[macro_use]
mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.replay_buffer();

    pin_mut!(events);

    client.get_replay_buffer_status().await?;

    client.start_stop_replay_buffer().await?;
    wait_for!(events, EventType::ReplayStarted { .. });
    client.start_stop_replay_buffer().await?;
    wait_for!(events, EventType::ReplayStopped { .. });

    // Wait a little more as the replay buffer sometimes doesn't start when started/stopped
    // frequently.
    time::sleep(Duration::from_secs(1)).await;

    client.start_replay_buffer().await?;
    wait_for!(events, EventType::ReplayStarted { .. });
    client.save_replay_buffer().await?;
    client.stop_replay_buffer().await?;
    wait_for!(events, EventType::ReplayStopped { .. });

    Ok(())
}
