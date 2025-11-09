use anyhow::Result;
use obws::{
    common::{DeinterlaceFieldOrder, MonitorType},
    requests::inputs::{Create, SetSettings, Volume},
};
use serde_json::json;
use test_log::test;
use uuid::Uuid;

use crate::common::{
    self, INPUT_KIND_BROWSER, INPUT_KIND_VLC, TEST_BROWSER, TEST_BROWSER_RENAME, TEST_MEDIA,
    TEST_SCENE,
};

#[test(tokio::test)]
async fn inputs() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.inputs();

    server.expect("GetInputList", json!({}), json!({"inputs": []}));

    client.list(None).await?;

    server.expect(
        "GetInputKindList",
        json!({"unversioned": false}),
        json!({"inputKinds": []}),
    );

    client.list_kinds(false).await?;

    server.expect(
        "GetSpecialInputs",
        json!(null),
        json!({
            "desktop1": "audio1",
            "desktop2": "audio2",
            "mic1": "audio3",
            "mic2": "audio4",
            "mic3": "audio5",
            "mic4": "audio6",
        }),
    );

    client.specials().await?;

    server.expect(
        "GetInputDefaultSettings",
        json!({"inputKind": "browser_source"}),
        json!({"defaultInputSettings": {}}),
    );

    client
        .default_settings::<serde_json::Value>(INPUT_KIND_BROWSER)
        .await?;

    server.expect(
        "GetInputSettings",
        json!({"inputName": "OBWS-TEST-Browser"}),
        json!({
            "inputSettings": {},
            "inputKind": "browser_source",
        }),
    );

    let settings = client
        .settings::<serde_json::Value>(TEST_BROWSER)
        .await?
        .settings;

    server.expect(
        "SetInputSettings",
        json!({
            "inputName": "OBWS-TEST-Browser",
            "inputSettings": {},
            "overlay": false,
        }),
        json!(null),
    );

    client
        .set_settings(SetSettings {
            input: TEST_BROWSER,
            settings: &settings,
            overlay: Some(false),
        })
        .await?;

    server.expect(
        "GetInputMute",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"inputMuted": true}),
    );

    let muted = client.muted(TEST_MEDIA).await?;

    server.expect(
        "SetInputMute",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputMuted": false,
        }),
        json!(null),
    );

    client.set_muted(TEST_MEDIA, !muted).await?;

    server.expect(
        "ToggleInputMute",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"inputMuted": true}),
    );

    client.toggle_mute(TEST_MEDIA).await?;

    server.expect(
        "GetInputVolume",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({
            "inputVolumeMul": 1.0,
            "inputVolumeDb": 20.5,
        }),
    );

    let volume = client.volume(TEST_MEDIA).await?;

    server.expect(
        "SetInputVolume",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputVolumeMul": 0.5,
        }),
        json!(null),
    );

    client
        .set_volume(TEST_MEDIA, Volume::Mul(volume.mul / 2.0))
        .await?;

    server.expect(
        "CreateInput",
        json!({
            "sceneName": "OBWS-TEST-Scene",
            "inputName": "new-input",
            "inputKind": "vlc_source",
            "inputSettings": {},
            "sceneItemEnabled": true,
        }),
        json!({
            "inputUuid": Uuid::nil(),
            "sceneItemId": 1,
        }),
    );

    let scene_item_id = client
        .create(Create {
            scene: TEST_SCENE,
            input: "new-input",
            kind: INPUT_KIND_VLC,
            settings: Some(serde_json::Map::new()),
            enabled: Some(true),
        })
        .await?;

    server.expect(
        "RemoveInput",
        json!({"inputUuid": Uuid::nil()}),
        json!(null),
    );

    client.remove(scene_item_id.input_uuid.into()).await?;

    server.expect(
        "SetInputName",
        json!({
            "inputName": "OBWS-TEST-Browser",
            "newInputName": "OBWS-TEST-Browser-Renamed",
        }),
        json!(null),
    );

    client
        .set_name(TEST_BROWSER, TEST_BROWSER_RENAME.as_name().unwrap())
        .await?;

    server.expect(
        "GetInputAudioBalance",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"inputAudioBalance": 1.0}),
    );

    let balance = client.audio_balance(TEST_MEDIA).await?;

    server.expect(
        "SetInputAudioBalance",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputAudioBalance": 0.5,
        }),
        json!(null),
    );

    client.set_audio_balance(TEST_MEDIA, balance / 2.0).await?;

    server.expect(
        "GetInputAudioSyncOffset",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"inputAudioSyncOffset": 1000}),
    );

    let offset = client.audio_sync_offset(TEST_MEDIA).await?;

    server.expect(
        "SetInputAudioSyncOffset",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputAudioSyncOffset": 500,
        }),
        json!(null),
    );

    client.set_audio_sync_offset(TEST_MEDIA, offset / 2).await?;

    server.expect(
        "GetInputAudioMonitorType",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"monitorType": "OBS_MONITORING_TYPE_NONE"}),
    );

    client.audio_monitor_type(TEST_MEDIA).await?;

    server.expect(
        "SetInputAudioMonitorType",
        json!({
            "inputName": "OBWS-TEST-Media",
            "monitorType": "OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT",
        }),
        json!(null),
    );

    client
        .set_audio_monitor_type(TEST_MEDIA, MonitorType::MonitorAndOutput)
        .await?;

    server.expect(
        "GetInputAudioTracks",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({
            "inputAudioTracks": {
                "1": true,
                "2": false,
                "3": false,
                "4": false,
                "5": false,
                "6": false,
            },
        }),
    );

    let tracks = client.audio_tracks(TEST_MEDIA).await?;

    server.expect(
        "SetInputAudioTracks",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputAudioTracks": {
                "1": false,
            },
        }),
        json!(null),
    );

    client
        .set_audio_tracks(TEST_MEDIA, [Some(!tracks[0]), None, None, None, None, None])
        .await?;

    server.expect(
        "GetInputDeinterlaceMode",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"inputDeinterlaceMode": "OBS_DEINTERLACE_MODE_BLEND"}),
    );

    client.deinterlace_mode(TEST_MEDIA).await?;

    server.expect(
        "SetInputDeinterlaceMode",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputDeinterlaceMode": "OBS_DEINTERLACE_MODE_LINEAR_2X",
        }),
        json!(null),
    );

    client
        .set_deinterlace_mode(TEST_MEDIA, obws::common::DeinterlaceMode::Linear2X)
        .await?;

    server.expect(
        "GetInputDeinterlaceFieldOrder",
        json!({"inputName": "OBWS-TEST-Media"}),
        json!({"inputDeinterlaceFieldOrder": "OBS_DEINTERLACE_FIELD_ORDER_TOP"}),
    );

    client.deinterlace_field_order(TEST_MEDIA).await?;

    server.expect(
        "SetInputDeinterlaceFieldOrder",
        json!({
            "inputName": "OBWS-TEST-Media",
            "inputDeinterlaceFieldOrder": "OBS_DEINTERLACE_FIELD_ORDER_BOTTOM",
        }),
        json!(null),
    );

    client
        .set_deinterlace_field_order(TEST_MEDIA, DeinterlaceFieldOrder::Bottom)
        .await?;

    server.expect(
        "GetInputPropertiesListPropertyItems",
        json!({
            "inputName": "OBWS-TEST-Media",
            "propertyName": "prop",
        }),
        json!({
            "propertyItems": [{
                "itemName": "Option",
                "itemEnabled": true,
                "itemValue": "hello",
            }],
        }),
    );

    client
        .properties_list_property_items(TEST_MEDIA, "prop")
        .await?;

    server.expect(
        "PressInputPropertiesButton",
        json!({
            "inputName": "OBWS-TEST-Media",
            "propertyName": "prop",
        }),
        json!(null),
    );

    client.press_properties_button(TEST_MEDIA, "prop").await?;

    server.stop().await
}
