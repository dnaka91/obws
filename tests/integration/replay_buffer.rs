use anyhow::Result;
use obws::events::{Event, OutputState};
use serde_json::json;
use test_log::test;

use crate::{common, wait_for};

#[test(tokio::test)]
async fn replay_buffer() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let events = client.events()?;
    let client = client.replay_buffer();

    tokio::pin!(events);

    server.expect(
        "GetReplayBufferStatus",
        json!(null),
        json!({"outputActive": false}),
    );

    client.status().await?;

    server.expect(
        "ToggleReplayBuffer",
        json!(null),
        json!({"outputActive": false}),
    );
    server.send_event(Event::ReplayBufferStateChanged {
        active: true,
        state: OutputState::Started,
    });

    client.toggle().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Started,
            ..
        }
    );

    server.expect("StartReplayBuffer", json!(null), json!(null));
    server.send_event(Event::ReplayBufferStateChanged {
        active: true,
        state: OutputState::Started,
    });

    client.start().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Started,
            ..
        }
    );

    server.expect("SaveReplayBuffer", json!(null), json!(null));

    client.save().await?;

    server.expect(
        "GetLastReplayBufferReplay",
        json!(null),
        json!({"savedReplayPath": "/tmp"}),
    );

    client.last_replay().await?;

    server.expect("StopReplayBuffer", json!(null), json!(null));
    server.send_event(Event::ReplayBufferStateChanged {
        active: true,
        state: OutputState::Stopped,
    });

    client.stop().await?;
    wait_for!(
        events,
        Event::ReplayBufferStateChanged {
            state: OutputState::Stopped,
            ..
        }
    );

    server.stop().await
}
