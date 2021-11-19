#![cfg(feature = "test-integration")]

use std::env;

use anyhow::Result;
use obws::requests::{GetSourceScreenshot, SaveSourceScreenshot};

use crate::common::{self, TEST_TEXT};

#[tokio::test]
async fn sources() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.sources();

    client.get_source_active(TEST_TEXT).await?;
    client
        .get_source_screenshot(GetSourceScreenshot {
            source_name: TEST_TEXT,
            image_width: Some(100),
            image_height: Some(100),
            image_compression_quality: Some(50),
            image_format: "jpg",
        })
        .await?;

    let file = env::temp_dir().join("obws-test-image.png");
    client
        .save_source_screenshot(SaveSourceScreenshot {
            source_name: TEST_TEXT,
            image_file_path: &file,
            image_width: None,
            image_height: None,
            image_compression_quality: None,
            image_format: "png",
        })
        .await?;

    Ok(())
}
