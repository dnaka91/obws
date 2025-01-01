use anyhow::Result;
use obws::requests::hotkeys::KeyModifiers;
use serde_json::json;
use test_log::test;

use crate::common;

#[test(tokio::test)]
async fn hotkeys() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.hotkeys();

    server.expect("GetHotkeyList", json!(null), json!({"hotkeys": []}));

    client.list().await?;

    server.expect(
        "TriggerHotkeyByName",
        json!({
            "hotkeyName": "ReplayBuffer.Save",
            "contextName": null,
        }),
        json!(null),
    );

    client.trigger_by_name("ReplayBuffer.Save", None).await?;

    server.expect(
        "TriggerHotkeyByKeySequence",
        json!({
            "keyId": "OBS_KEY_P",
            "keyModifiers": {
                "shift": false,
                "control": false,
                "alt": false,
                "command": false,
            },
        }),
        json!(null),
    );

    client
        .trigger_by_sequence("OBS_KEY_P", KeyModifiers::default())
        .await?;

    server.stop().await
}
