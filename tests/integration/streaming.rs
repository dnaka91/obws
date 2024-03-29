use anyhow::Result;

use crate::common;

#[tokio::test]
async fn streaming() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.streaming();

    client.status().await?;

    // TODO: Dangerous to run as it would make us live stream.
    // client.start_stream().await?;
    // client.stop_stream().await?;

    Ok(())
}
