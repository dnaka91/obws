#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use obws::events::{Event, OutputState};
use tokio::time;

use crate::{common, wait_for};

const OUTPUT_VIRTUALCAM: &str = "virtualcam_output";

#[tokio::test]
async fn outputs() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.outputs();

    tokio::pin!(events);

    client.list().await?;
    client.status(OUTPUT_VIRTUALCAM).await?;

    client.toggle(OUTPUT_VIRTUALCAM).await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle(OUTPUT_VIRTUALCAM).await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.start(OUTPUT_VIRTUALCAM).await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.stop(OUTPUT_VIRTUALCAM).await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    let settings = client
        .settings::<serde_json::Value>(OUTPUT_VIRTUALCAM)
        .await?;
    client.set_settings(OUTPUT_VIRTUALCAM, &settings).await?;

    Ok(())
}
