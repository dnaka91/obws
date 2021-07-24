#![cfg(feature = "test-integration")]

use anyhow::Result;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.streaming();

    client.get_stream_status().await?;

    // TODO: Dangerous to run as it would make us live stream.
    // client.start_stream().await?;
    // client.stop_stream().await?;

    Ok(())
}
