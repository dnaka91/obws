use std::env;

use anyhow::Result;
use futures_util::{pin_mut, StreamExt};
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    pretty_env_logger::init();

    let client = Client::connect("localhost", 4444, env::var("OBS_PASSWORD").ok()).await?;

    let events = client.events()?;
    pin_mut!(events);

    while let Some(event) = events.next().await {
        println!("{:#?}", event);
    }

    Ok(())
}
