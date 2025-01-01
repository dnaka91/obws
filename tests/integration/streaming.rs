use anyhow::Result;
use serde_json::json;
use test_log::test;

use crate::common;

#[test(tokio::test)]
async fn streaming() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.streaming();

    server.expect(
        "GetStreamStatus",
        json!(null),
        json!({
            "outputActive": false,
            "outputReconnecting": false,
            "outputTimecode": "00:00:00.000",
            "outputDuration": 0,
            "outputCongestion": 0,
            "outputBytes": 0,
            "outputSkippedFrames": 0,
            "outputTotalFrames": 0,
        }),
    );

    client.status().await?;

    server.expect("StartStream", json!(null), json!(null));

    client.start().await?;

    server.expect("StopStream", json!(null), json!(null));

    client.stop().await?;

    server.expect("ToggleStream", json!(null), json!({"outputActive": true}));

    client.toggle().await?;

    server.expect(
        "SendStreamCaption",
        json!({"captionText": "test"}),
        json!(null),
    );

    client.send_caption("test").await?;

    server.stop().await
}
