#![cfg(feature = "test-integration")]

use anyhow::Result;

use common::TEST_TRANSITION_2;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.transitions();

    client.get_transition_list().await?;
    let original = client.get_current_transition().await?.name;
    client.set_current_transition(TEST_TRANSITION_2).await?;
    client.set_current_transition(&original).await?;

    let original = client.get_transition_duration().await?;
    client.set_transition_duration(original * 2).await?;
    client.set_transition_duration(original).await?;

    Ok(())
}
