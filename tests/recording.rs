#![cfg(feature = "test-integration")]

use std::{path::Path, time::Duration};

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
    let client = client.recording();

    pin_mut!(events);

    client.get_recording_status().await?;

    client.start_stop_recording().await?;
    wait_for!(events, EventType::RecordingStarted { .. });
    client.start_stop_recording().await?;
    wait_for!(events, EventType::RecordingStopped { .. });

    // Wait a little more as recording sometimes doesn't start when started/stopped frequently.
    time::sleep(Duration::from_secs(1)).await;

    client.start_recording().await?;
    wait_for!(events, EventType::RecordingStarted { .. });
    time::sleep(Duration::from_secs(1)).await;
    client.pause_recording().await?;
    wait_for!(events, EventType::RecordingPaused);
    time::sleep(Duration::from_secs(1)).await;
    client.resume_recording().await?;
    wait_for!(events, EventType::RecordingResumed);
    time::sleep(Duration::from_secs(1)).await;
    client.stop_recording().await?;
    wait_for!(events, EventType::RecordingStopped { .. });

    let original = client.get_recording_folder().await?;
    client.set_recording_folder(Path::new("test")).await?;
    client.set_recording_folder(&original).await?;

    Ok(())
}
