#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use tokio::time;

use crate::common::TEST_PROFILE;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.profiles();

    client.list_profiles().await?;

    let original = client.get_current_profile().await?;
    client.set_current_profile(TEST_PROFILE).await?;

    // Give OBS some time to switch profiles
    time::sleep(Duration::from_millis(200)).await;

    client.set_current_profile(&original).await?;

    Ok(())
}
