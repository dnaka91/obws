use anyhow::Result;
use obws::events::Event;
use serde::Serialize;

use crate::{common, wait_for};

#[tokio::test]
async fn general() -> Result<()> {
    let client = common::new_client().await?;
    let events = client.events()?;
    let client = client.general();

    tokio::pin!(events);

    client.version().await?;
    client
        .broadcast_custom_event(&CustomEvent { hello: "world!" })
        .await?;
    wait_for!(events, Event::CustomEvent(_));
    client.stats().await?;

    Ok(())
}

#[derive(Serialize)]
struct CustomEvent<'a> {
    hello: &'a str,
}
