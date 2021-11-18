#![cfg(feature = "test-integration")]

use anyhow::Result;
use obws::common::MediaAction;
use time::Duration;

use crate::common::TEST_MEDIA;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.media_inputs();

    client.get_media_input_status(TEST_MEDIA).await?;
    client
        .set_media_input_cursor(TEST_MEDIA, Duration::seconds(1))
        .await?;
    client
        .offset_media_input_cursor(TEST_MEDIA, Duration::seconds(1))
        .await?;
    client
        .trigger_media_input_action(TEST_MEDIA, MediaAction::Next)
        .await?;

    Ok(())
}