use anyhow::Result;
use obws::events::{Event, OutputState};
use serde_json::json;
use test_log::test;

use crate::{common, wait_for};

#[test(tokio::test)]
async fn recording() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let mut events = client.events()?;
    let client = client.recording();

    server.expect(
        "GetRecordStatus",
        json!(null),
        json!({
            "outputActive": false,
            "outputPaused": false,
            "outputTimecode": "00:00:00.500",
            "outputDuration": 500,
            "outputBytes": 2048,
        }),
    );

    client.status().await?;

    server.expect("StartRecord", json!(null), json!(null));
    server.send_event(Event::RecordStateChanged {
        active: true,
        state: OutputState::Started,
        path: None,
    });

    client.start().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Started,
            ..
        }
    );

    server.expect("PauseRecord", json!(null), json!(null));
    server.send_event(Event::RecordStateChanged {
        active: true,
        state: OutputState::Paused,
        path: None,
    });

    client.pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Paused,
            ..
        }
    );

    server.expect("ResumeRecord", json!(null), json!(null));
    server.send_event(Event::RecordStateChanged {
        active: true,
        state: OutputState::Resumed,
        path: None,
    });

    client.resume().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Resumed,
            ..
        }
    );

    server.expect("StopRecord", json!(null), json!({"outputPath": "/tmp"}));
    server.send_event(Event::RecordStateChanged {
        active: false,
        state: OutputState::Stopped,
        path: None,
    });

    client.stop().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );

    server.expect("ToggleRecord", json!(null), json!({"outputActive": true}));
    server.send_event(Event::RecordStateChanged {
        active: true,
        state: OutputState::Started,
        path: None,
    });

    client.toggle().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Started,
            ..
        }
    );

    server.expect(
        "ToggleRecordPause",
        json!(null),
        json!({"outputPaused": true}),
    );
    server.send_event(Event::RecordStateChanged {
        active: true,
        state: OutputState::Paused,
        path: None,
    });

    client.toggle_pause().await?;
    wait_for!(
        events,
        Event::RecordStateChanged {
            state: OutputState::Paused,
            ..
        }
    );

    server.expect("SplitRecordFile", json!(null), json!(null));

    client.split_file().await?;

    server.expect(
        "CreateRecordChapter",
        json!({"chapterName": "one"}),
        json!(null),
    );

    client.create_chapter(Some("one")).await?;

    server.stop().await
}
