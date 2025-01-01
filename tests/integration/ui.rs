use anyhow::Result;
use obws::requests::ui::{
    Location, OpenSourceProjector, OpenVideoMixProjector, QtGeometry, QtRect, VideoMixType,
};
use serde_json::json;
use test_log::test;

use crate::common::{self, TEST_TEXT};

#[test(tokio::test)]
async fn ui() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.ui();

    server.expect(
        "GetStudioModeEnabled",
        json!(null),
        json!({"studioModeEnabled": false}),
    );

    let enabled = client.studio_mode_enabled().await?;

    server.expect(
        "SetStudioModeEnabled",
        json!({"studioModeEnabled": true}),
        json!(null),
    );

    client.set_studio_mode_enabled(!enabled).await?;

    server.expect(
        "OpenInputPropertiesDialog",
        json!({"inputName": "OBWS-TEST-Text"}),
        json!(null),
    );

    client.open_properties_dialog(TEST_TEXT).await?;

    server.expect(
        "OpenInputFiltersDialog",
        json!({"inputName": "OBWS-TEST-Text"}),
        json!(null),
    );

    client.open_filters_dialog(TEST_TEXT).await?;

    server.expect(
        "OpenInputInteractDialog",
        json!({"inputName": "OBWS-TEST-Text"}),
        json!(null),
    );

    client.open_interact_dialog(TEST_TEXT).await?;

    server.expect(
        "GetMonitorList",
        json!(null),
        json!({
            "monitors": [{
                "monitorName": "sample",
                "monitorIndex": 0,
                "monitorWidth": 640,
                "monitorHeight": 480,
                "monitorPositionX": 5,
                "monitorPositionY": 10,
            }],
        }),
    );

    client.list_monitors().await?;

    let geometry = QtGeometry {
        rect: QtRect {
            left: 50,
            top: 150,
            right: 250,
            bottom: 350,
        },
        ..QtGeometry::default()
    };

    server.expect(
        "OpenVideoMixProjector",
        json!({
            "videoMixType": "OBS_WEBSOCKET_VIDEO_MIX_TYPE_PREVIEW",
            "projectorGeometry": geometry.to_string(),
        }),
        json!(null),
    );

    client
        .open_video_mix_projector(OpenVideoMixProjector {
            r#type: VideoMixType::Preview,
            location: Some(Location::ProjectorGeometry(geometry)),
        })
        .await?;

    server.expect(
        "OpenSourceProjector",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "monitorIndex": -1,
        }),
        json!(null),
    );

    client
        .open_source_projector(OpenSourceProjector {
            source: TEST_TEXT.as_source(),
            location: Some(Location::MonitorIndex(-1)),
        })
        .await?;

    server.stop().await
}
