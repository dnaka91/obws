use std::env;

use anyhow::Result;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;

    let version = client.general().version().await?;
    println!("{:#?}", version);

    let scene_list = client.scenes().list().await?.scenes;
    println!("{:#?}", scene_list);

    Ok(())
}
