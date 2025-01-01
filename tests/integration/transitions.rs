use anyhow::Result;
use serde_json::json;
use test_log::test;
use uuid::Uuid;

use crate::common::{self, TEST_TRANSITION};

#[test(tokio::test)]
async fn transitions() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let ui = client.ui();
    let client = client.transitions();

    server.expect(
        "GetTransitionKindList",
        json!(null),
        json!({"transitionKinds": ["fade"]}),
    );

    client.list_kinds().await?;

    server.expect(
        "GetSceneTransitionList",
        json!(null),
        json!({
            "currentSceneTransitionName": "main",
            "currentSceneTransitionUuid": Uuid::new_v8([1; 16]),
            "currentSceneTransitionKind": "fade",
            "transitions": [{
                "transitionName": "main",
                "transitionUuid": Uuid::new_v8([1; 16]),
                "transitionKind": "fade",
                "transitionFixed": false,
                "transitionConfigurable": false,
            }],
        }),
    );

    client.list().await?;

    server.expect(
        "SetCurrentSceneTransition",
        json!({"transitionName": "OBWS-TEST-Transition"}),
        json!(null),
    );

    client.set_current(TEST_TRANSITION).await?;

    server.expect(
        "GetCurrentSceneTransition",
        json!(null),
        json!({
            "transitionName": "OBWS-TEST-Transition",
            "transitionUuid": Uuid::new_v8([1; 16]),
            "transitionKind": "fade",
            "transitionFixed": false,
            "transitionDuration": 1000,
            "transitionConfigurable": true,
            "transitionSettings": {},
        }),
    );

    let transition = client.current().await?;

    server.expect(
        "SetCurrentSceneTransitionDuration",
        json!({"transitionDuration": 500}),
        json!(null),
    );

    client
        .set_current_duration(transition.duration.unwrap() / 2)
        .await?;

    server.expect(
        "SetCurrentSceneTransitionSettings",
        json!({"transitionSettings": {}}),
        json!(null),
    );

    client
        .set_current_settings(transition.settings.unwrap(), None)
        .await?;

    server.expect(
        "GetCurrentSceneTransitionCursor",
        json!(null),
        json!({"transitionCursor": 0.1}),
    );

    client.current_cursor().await?;

    server.expect(
        "SetStudioModeEnabled",
        json!({"studioModeEnabled": true}),
        json!(null),
    );

    ui.set_studio_mode_enabled(true).await?;

    server.expect("TriggerStudioModeTransition", json!(null), json!(null));

    client.trigger().await?;

    server.expect("SetTBarPosition", json!({"position": 0.5}), json!(null));

    client.set_tbar_position(0.5, None).await?;

    server.expect(
        "SetStudioModeEnabled",
        json!({"studioModeEnabled": false}),
        json!(null),
    );

    ui.set_studio_mode_enabled(false).await?;

    server.stop().await
}
