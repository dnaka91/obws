use std::env;

use anyhow::Result;
use tokio::fs;

use obws::{client::Client, requests::SourceScreenshot};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    pretty_env_logger::init();

    let client = Client::connect("localhost", 4444).await?;

    client.login(env::var("OBS_PASSWORD").ok()).await?;

    let screenshot = client
        .sources()
        .take_source_screenshot(SourceScreenshot {
            source_name: "Start",
            embed_picture_format: Some("png"),
            ..Default::default()
        })
        .await?;

    let image = screenshot.img.unwrap();
    let pos = image.find("base64,").unwrap();
    let image = base64::decode(&image[pos + 7..])?;

    fs::write("screenshot.png", &image).await?;

    Ok(())
}
