use std::time::Duration;

use anyhow::Result;
use obws::events::{Event, OutputState};
use tokio::time;

use crate::{common, wait_for};

#[tokio::test]
async fn recording() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.recording();

    tokio::pin!(events);

    client.status().await?;

    client.start().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Paused,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.resume().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Resumed,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.stop().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    client.toggle().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Paused,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Resumed,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
