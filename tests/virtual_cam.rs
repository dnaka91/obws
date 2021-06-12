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
    let client = client.virtual_cam();

    pin_mut!(events);

    client.get_virtual_cam_status().await?;

    client.start_stop_virtual_cam().await?;
    wait_for!(events, EventType::VirtualCamStarted { .. });
    client.start_stop_virtual_cam().await?;
    wait_for!(events, EventType::VirtualCamStopped { .. });

    // Wait a little more as the virtual cam sometimes doesn't start when started/stopped
    // frequently.
    time::sleep(Duration::from_secs(1)).await;

    client.start_virtual_cam().await?;
    wait_for!(events, EventType::VirtualCamStarted { .. });
    client.stop_virtual_cam().await?;
    wait_for!(events, EventType::VirtualCamStopped { .. });

    Ok(())
}
