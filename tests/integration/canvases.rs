use anyhow::Result;
use serde_json::json;
use test_log::test;
use uuid::Uuid;

use crate::common;

#[test(tokio::test)]
async fn canvases() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.canvases();

    server.expect(
        "GetCanvasList",
        json!(null),
        json!({
            "canvases": [
                {
                    "canvasName": "main",
                    "canvasUuid":  Uuid::new_v8([1; 16]),
                    "canvasFlags": {
                        "MAIN": true,
                        "ACTIVATE": true,
                        "MIX_AUDIO": true,
                        "SCENE_REF": true,
                    },
                    "canvasVideoSettings": {
                        "fpsNumerator": 1,
                        "fpsDenominator": 60,
                        "baseWidth": 1920,
                        "baseHeight": 1080,
                        "outputWidth": 1280,
                        "outputHeight": 720,
                    },
                },
            ],
        }),
    );

    client.list().await?;

    server.stop().await
}
