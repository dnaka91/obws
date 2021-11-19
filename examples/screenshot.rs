use std::env;

use anyhow::Result;
use obws::{requests::GetSourceScreenshot, Client};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4444, env::var("OBS_PASSWORD").ok()).await?;

    let screenshot = client
        .sources()
        .get_source_screenshot(GetSourceScreenshot {
            source_name: "OBWS-TEST-Scene",
            image_width: None,
            image_height: None,
            image_compression_quality: None,
            image_format: "png",
        })
        .await?;

    let pos = screenshot.find("base64,").unwrap();
    let image = base64::decode(&screenshot[pos + 7..])?;

    fs::write("screenshot.png", &image).await?;

    Ok(())
}
