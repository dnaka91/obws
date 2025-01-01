use anyhow::Result;
use obws::common::MediaAction;
use serde_json::json;
use test_log::test;
use time::Duration;

use crate::common::{self, TEST_MEDIA};

#[test(tokio::test)]
async fn media_inputs() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.media_inputs();

    server.expect(
        "GetMediaInputStatus",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({
            "mediaState": "OBS_MEDIA_STATE_PLAYING",
            "mediaDuration": 12_500,
            "mediaCursor": 100,
        }),
    );

    let status = client.status(TEST_MEDIA).await?;

    server.expect(
        "SetMediaInputCursor",
        json!({
            "inputName": "OBWS-TEST-Media",
            "mediaCursor": 50,
        }),
        json!(null),
    );

    client
        .set_cursor(TEST_MEDIA, status.cursor.unwrap() / 2)
        .await?;

    server.expect(
        "OffsetMediaInputCursor",
        json!({
            "inputName": "OBWS-TEST-Media",
            "mediaCursorOffset": 1000,
        }),
        json!(null),
    );

    client
        .offset_cursor(TEST_MEDIA, Duration::seconds(1))
        .await?;

    server.expect(
        "TriggerMediaInputAction",
        json!({
            "inputName": "OBWS-TEST-Media",
            "mediaAction": "OBS_WEBSOCKET_MEDIA_INPUT_ACTION_NEXT",
        }),
        json!(null),
    );

    client.trigger_action(TEST_MEDIA, MediaAction::Next).await?;

    server.stop().await
}
