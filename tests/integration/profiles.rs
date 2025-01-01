use anyhow::Result;
use obws::{requests::profiles::SetParameter, responses::profiles::Profiles};
use serde_json::json;
use test_log::test;

use crate::common;

#[test(tokio::test)]
async fn profiles() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.profiles();

    server.expect(
        "GetProfileList",
        json!(null),
        json!({
            "currentProfileName": "main",
            "profiles": ["main", "other"],
        }),
    );

    let Profiles { current, profiles } = client.list().await?;

    server.expect(
        "GetProfileList",
        json!(null),
        json!({
            "currentProfileName": "main",
            "profiles": ["main", "other"],
        }),
    );

    client.current().await?;
    let other = profiles.iter().find(|p| *p != &current).unwrap();

    server.expect(
        "SetCurrentProfile",
        json!({"profileName": "other"}),
        json!(null),
    );

    client.set_current(other).await?;

    server.expect(
        "CreateProfile",
        json!({"profileName": "OBWS-TEST-New-Profile"}),
        json!(null),
    );

    client.create("OBWS-TEST-New-Profile").await?;

    server.expect(
        "RemoveProfile",
        json!({"profileName": "OBWS-TEST-New-Profile"}),
        json!(null),
    );

    client.remove("OBWS-TEST-New-Profile").await?;

    server.expect(
        "GetProfileParameter",
        json!({
            "parameterCategory": "General",
            "parameterName": "Name",
        }),
        json!({
            "parameterValue": "Some",
            "defaultParameterValue": null,
        }),
    );

    client.parameter("General", "Name").await?;

    server.expect(
        "SetProfileParameter",
        json!({
            "parameterCategory": "OBWS",
            "parameterName": "Test",
            "parameterValue": "Value",
        }),
        json!(null),
    );

    client
        .set_parameter(SetParameter {
            category: "OBWS",
            name: "Test",
            value: Some("Value"),
        })
        .await?;

    server.stop().await
}
