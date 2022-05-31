use std::env;

use anyhow::Result;
use obws::requests::sources::{TakeScreenshot, SaveScreenshot};

use crate::common::{self, TEST_TEXT};

#[tokio::test]
async fn sources() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.sources();

    client.active(TEST_TEXT).await?;
    client
        .take_screenshot(TakeScreenshot {
            source: TEST_TEXT,
            width: Some(100),
            height: Some(100),
            compression_quality: Some(50),
            format: "jpg",
        })
        .await?;

    let file = env::temp_dir().join("obws-test-image.png");
    client
        .save_screenshot(SaveScreenshot {
            source: TEST_TEXT,
            file_path: &file,
            width: None,
            height: None,
            compression_quality: None,
            format: "png",
        })
        .await?;

    Ok(())
}
