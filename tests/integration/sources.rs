use std::path::Path;

use anyhow::Result;
use obws::requests::sources::{SaveScreenshot, TakeScreenshot};
use serde_json::json;
use test_log::test;

use crate::common::{self, TEST_TEXT};

#[test(tokio::test)]
async fn sources() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.sources();

    server.expect(
        "GetSourceActive",
        json!({"sourceName": "OBWS-TEST-Text"}),
        json!({
            "videoActive": true,
            "videoShowing": true,
        }),
    );

    client.active(TEST_TEXT.as_source()).await?;

    server.expect(
        "GetSourceScreenshot",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "imageFormat": "jpg",
            "imageWidth": 100,
            "imageHeight": 100,
            "imageCompressionQuality": 50,
        }),
        json!({"imageData": ""}),
    );

    client
        .take_screenshot(TakeScreenshot {
            source: TEST_TEXT.as_source(),
            width: Some(100),
            height: Some(100),
            compression_quality: Some(50),
            format: "jpg",
        })
        .await?;

    server.expect(
        "SaveSourceScreenshot",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "imageFormat": "png",
            "imageFilePath": "/tmp/file.png",
        }),
        json!(null),
    );

    client
        .save_screenshot(SaveScreenshot {
            source: TEST_TEXT.as_source(),
            file_path: Path::new("/tmp/file.png"),
            width: None,
            height: None,
            compression_quality: None,
            format: "png",
        })
        .await?;

    server.stop().await
}
