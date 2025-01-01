use anyhow::Result;
use obws::requests::EventSubscription;
use test_log::test;

use crate::common;

#[test(tokio::test)]
async fn client() -> Result<()> {
    let (client, server) = common::new_client().await?;

    client.reidentify(EventSubscription::ALL).await?;

    server.stop().await
}
