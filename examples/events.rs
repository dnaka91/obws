use std::env;

use anyhow::Result;
use futures_util::{StreamExt, pin_mut};
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;

    let events = client.events()?;
    pin_mut!(events);

    while let Some(event) = events.next().await {
        println!("{event:#?}");
    }

    Ok(())
}
