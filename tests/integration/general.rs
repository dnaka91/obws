use anyhow::Result;
use obws::{events::Event, requests::general::CallVendorRequest};
use serde::Serialize;
use serde_json::json;
use test_log::test;

use crate::{common, wait_for};

#[test(tokio::test)]
async fn general() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let mut events = client.events()?;
    let client = client.general();

    server.expect(
        "GetVersion",
        json!(null),
        json!({
            "obsVersion": "31.0.0",
            "obsWebSocketVersion": "5.5.0",
            "rpcVersion": 1,
            "availableRequests": [],
            "supportedImageFormats": [],
            "platform": "mock",
            "platformDescription": "",
        }),
    );

    client.version().await?;

    server.expect(
        "GetStats",
        json!(null),
        json!({
            "cpuUsage": 0.5,
            "memoryUsage": 200,
            "availableDiskSpace": 30_000_000,
            "activeFps": 59.99,
            "averageFrameRenderTime": 5,
            "renderSkippedFrames": 0,
            "renderTotalFrames": 10_000,
            "outputSkippedFrames": 0,
            "outputTotalFrames": 8_000,
            "webSocketSessionIncomingMessages": 10,
            "webSocketSessionOutgoingMessages": 10,
        }),
    );

    client.stats().await?;

    server.expect(
        "BroadcastCustomEvent",
        json!({
            "eventData": {
                "hello": "world!",
            },
        }),
        json!(null),
    );

    client
        .broadcast_custom_event(&CustomEvent { hello: "world!" })
        .await?;

    server.send_event(Event::CustomEvent(json!({"hello": "world!"})));
    wait_for!(events, Event::CustomEvent(_));

    server.expect(
        "CallVendorRequest",
        json!({
            "vendorName": "mock",
            "requestType": "call",
            "requestData": 1,
        }),
        json!({
            "vendorName": "mock",
            "requestType": "call",
            "responseData": true,
        }),
    );

    client
        .call_vendor_request::<_, bool>(CallVendorRequest {
            vendor_name: "mock",
            request_type: "call",
            request_data: &1,
        })
        .await?;

    server.stop().await
}

#[derive(Serialize)]
struct CustomEvent<'a> {
    hello: &'a str,
}
