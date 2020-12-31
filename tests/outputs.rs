#![cfg(feature = "test-integration")]

use anyhow::Result;

use common::TEST_OUTPUT;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.outputs();

    client.list_outputs().await?;
    client.get_output_info(TEST_OUTPUT).await?;
    client.start_output(TEST_OUTPUT).await?;
    client.stop_output(TEST_OUTPUT, Some(true)).await?;

    Ok(())
}
