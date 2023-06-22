use anyhow::Result;
use obws::requests::EventSubscription;

use crate::common;

#[tokio::test]
async fn client() -> Result<()> {
    let client = common::new_client().await?;

    client.reidentify(EventSubscription::ALL).await?;

    Ok(())
}
