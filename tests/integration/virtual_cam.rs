#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use obws::events::{Event, OutputState};
use tokio::time;

use crate::{common, wait_for};

#[tokio::test]
async fn virtual_cam() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.virtual_cam();

    tokio::pin!(events);

    client.status().await?;

    client.toggle().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.start().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.stop().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
