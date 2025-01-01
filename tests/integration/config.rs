use anyhow::Result;
use obws::requests::config::{Realm, SetPersistentData};
use serde_json::json;
use test_log::test;

use crate::common;

#[test(tokio::test)]
async fn config() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.config();

    server.expect(
        "SetPersistentData",
        json!({
            "realm": "OBS_WEBSOCKET_DATA_REALM_PROFILE",
            "slotName": "obws-test",
            "slotValue": true,
        }),
        json!(null),
    );

    client
        .set_persistent_data(SetPersistentData {
            realm: Realm::Profile,
            slot_name: "obws-test",
            slot_value: &true.into(),
        })
        .await?;

    server.expect(
        "GetPersistentData",
        json!({
            "realm": "OBS_WEBSOCKET_DATA_REALM_PROFILE",
            "slotName": "obws-test",
        }),
        json!({"slotValue": true}),
    );

    client
        .get_persistent_data(Realm::Profile, "obws-test")
        .await?;

    server.expect(
        "GetVideoSettings",
        json!(null),
        json!({
            "fpsNumerator": 1,
            "fpsDenominator": 60,
            "baseWidth": 1920,
            "baseHeight": 1080,
            "outputWidth": 1280,
            "outputHeight": 720,
        }),
    );

    let settings = client.video_settings().await?;

    server.expect(
        "SetVideoSettings",
        json!({
            "fpsNumerator": 1,
            "fpsDenominator": 60,
            "baseWidth": 1920,
            "baseHeight": 1080,
            "outputWidth": 1280,
            "outputHeight": 720,
        }),
        json!(null),
    );

    client.set_video_settings(settings.into()).await?;

    server.expect(
        "GetStreamServiceSettings",
        json!(null),
        json!({
            "streamServiceType": "rtmp_common",
            "streamServiceSettings": {},
        }),
    );

    let settings = client
        .stream_service_settings::<serde_json::Value>()
        .await?;

    server.expect(
        "SetStreamServiceSettings",
        json!({
            "streamServiceType": "rtmp_common",
            "streamServiceSettings": {},
        }),
        json!(null),
    );

    client
        .set_stream_service_settings(&settings.r#type, &settings.settings)
        .await?;

    server.expect(
        "GetRecordDirectory",
        json!(null),
        json!({"recordDirectory": "/tmp"}),
    );

    let directory = client.record_directory().await?;

    server.expect(
        "SetRecordDirectory",
        json!({"recordDirectory": "/tmp"}),
        json!(null),
    );

    client.set_record_directory(&directory).await?;

    server.stop().await
}
