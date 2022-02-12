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

    client.get_record_status().await?;
    client.get_record_directory().await?;

    client.start_record().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.pause_record().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Paused,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.resume_record().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Resumed,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.stop_record().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    client.toggle_record().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Started,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_record_pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Paused,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_record_pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Resumed,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;
    client.toggle_record().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            output_state: OutputState::Stopped,
            ..
        }
    );
    time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
