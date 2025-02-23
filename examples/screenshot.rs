use std::env;

use anyhow::Result;
use base64::engine::{Engine, general_purpose};
use obws::{Client, requests::sources::TakeScreenshot};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;

    let screenshot = client
        .sources()
        .take_screenshot(TakeScreenshot {
            source: "OBWS-TEST-Scene".into(),
            width: None,
            height: None,
            compression_quality: None,
            format: "png",
        })
        .await?;

    let pos = screenshot.find("base64,").unwrap();
    let image = general_purpose::STANDARD.decode(&screenshot[pos + 7..])?;

    fs::write("screenshot.png", &image).await?;

    Ok(())
}
