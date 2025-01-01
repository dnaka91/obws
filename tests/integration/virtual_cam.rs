use anyhow::Result;
use obws::events::{Event, OutputState};
use serde_json::json;
use test_log::test;

use crate::{common, wait_for};

#[test(tokio::test)]
async fn virtual_cam() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let events = client.events()?;
    let client = client.virtual_cam();

    tokio::pin!(events);

    server.expect(
        "GetVirtualCamStatus",
        json!(null),
        json!({"outputActive": false}),
    );

    client.status().await?;

    server.expect(
        "ToggleVirtualCam",
        json!(null),
        json!({"outputActive": true}),
    );
    server.send_event(Event::VirtualcamStateChanged {
        active: true,
        state: OutputState::Started,
    });

    client.toggle().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Started,
            ..
        }
    );

    server.expect("StartVirtualCam", json!(null), json!(null));
    server.send_event(Event::VirtualcamStateChanged {
        active: true,
        state: OutputState::Started,
    });

    client.start().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Started,
            ..
        }
    );

    server.expect("StopVirtualCam", json!(null), json!(null));
    server.send_event(Event::VirtualcamStateChanged {
        active: false,
        state: OutputState::Stopped,
    });

    client.stop().await?;
    wait_for!(
        events,
        Event::VirtualcamStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );

    server.stop().await
}
