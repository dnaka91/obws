use anyhow::Result;

use crate::common::{self, TEST_FILTER, TEST_TEXT};

#[tokio::test]
async fn filters() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.filters();

    client.get_source_filter(TEST_TEXT, TEST_FILTER).await?;

    Ok(())
}
