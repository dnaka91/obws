use anyhow::Result;
use serde_json::json;
use test_log::test;

use crate::common;

const OUTPUT_VIRTUALCAM: &str = "virtualcam_output";

#[test(tokio::test)]
async fn outputs() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.outputs();

    server.expect("GetOutputList", json!(null), json!({"outputs": []}));

    client.list().await?;

    server.expect(
        "GetOutputStatus",
        json!({"outputName": "virtualcam_output"}),
        json!({
            "outputActive": true,
            "outputReconnecting": false,
            "outputTimecode": "12:30:45.678",
            "outputDuration": 50_000,
            "outputCongestion": 0,
            "outputBytes": 1024,
            "outputSkippedFrames": 0,
            "outputTotalFrames": 250,
        }),
    );

    client.status(OUTPUT_VIRTUALCAM).await?;

    server.expect(
        "ToggleOutput",
        json!({"outputName": "virtualcam_output"}),
        json!({"outputActive": false}),
    );

    client.toggle(OUTPUT_VIRTUALCAM).await?;

    server.expect(
        "StartOutput",
        json!({"outputName": "virtualcam_output"}),
        json!(null),
    );

    client.start(OUTPUT_VIRTUALCAM).await?;

    server.expect(
        "StopOutput",
        json!({"outputName": "virtualcam_output"}),
        json!(null),
    );

    client.stop(OUTPUT_VIRTUALCAM).await?;

    server.expect(
        "GetOutputSettings",
        json!({"outputName": "virtualcam_output"}),
        json!({"outputSettings": {}}),
    );

    let settings = client
        .settings::<serde_json::Value>(OUTPUT_VIRTUALCAM)
        .await?;

    server.expect(
        "SetOutputSettings",
        json!({
            "outputName": "virtualcam_output",
            "outputSettings": {},
        }),
        json!(null),
    );

    client.set_settings(OUTPUT_VIRTUALCAM, &settings).await?;

    server.stop().await
}
