use std::env;

use anyhow::Result;
use obws::{requests::sources::TakeScreenshot, Client};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;

    let screenshot = client
        .sources()
        .take_screenshot(TakeScreenshot {
            source: "OBWS-TEST-Scene",
            width: None,
            height: None,
            compression_quality: None,
            format: "png",
        })
        .await?;

    let pos = screenshot.find("base64,").unwrap();
    let image = base64::decode(&screenshot[pos + 7..])?;

    fs::write("screenshot.png", &image).await?;

    Ok(())
}
