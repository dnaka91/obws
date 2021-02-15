#![cfg(feature = "test-integration")]

use anyhow::Result;
use futures_util::{pin_mut, StreamExt};
use obws::events::{Event, EventType};

use common::TEST_MEDIA;

#[macro_use]
mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events();
    let client = client.media_control();

    pin_mut!(events);

    client.play_pause_media(TEST_MEDIA, false).await?;
    wait_for!(events, EventType::MediaPlaying { .. });
    client.next_media(TEST_MEDIA).await?;
    wait_for!(events, EventType::MediaNext { .. });
    client.previous_media(TEST_MEDIA).await?;
    wait_for!(events, EventType::MediaPrevious { .. });
    client.play_pause_media(TEST_MEDIA, true).await?;
    wait_for!(events, EventType::MediaPaused { .. });

    let duration = client.get_media_duration(TEST_MEDIA).await?;
    client.set_media_time(TEST_MEDIA, duration / 2).await?;
    client.get_media_time(TEST_MEDIA).await?;
    client.scrub_media(TEST_MEDIA, duration / 4).await?;
    client.get_media_state(TEST_MEDIA).await?;

    client.restart_media(TEST_MEDIA).await?;
    wait_for!(events, EventType::MediaRestarted { .. });
    client.stop_media(TEST_MEDIA).await?;
    wait_for!(events, EventType::MediaStopped { .. });

    Ok(())
}
