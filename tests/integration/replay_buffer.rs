#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use obws::events::{Event, OutputState};
use tokio::time;

use crate::{common, wait_for};

#[tokio::test]
async fn replay_buffer() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.replay_buffer();

    tokio::pin!(events);

    client.status().await?;

    client.toggle().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.start().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.save().await?;
    client.last_replay().await?;
    client.stop().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
