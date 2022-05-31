use std::time::Duration;

use anyhow::Result;
use obws::{requests::profiles::SetParameter, responses::Profiles};
use tokio::time;

use crate::common;

#[tokio::test]
async fn profiles() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.profiles();

    let Profiles { current, profiles } = client.list().await?;
    client.current().await?;
    let other = profiles.iter().find(|p| *p != &current).unwrap();
    client.set_current(other).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.set_current(&current).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.create("OBWS-TEST-New-Profile").await?;
    client.remove("OBWS-TEST-New-Profile").await?;

    client.parameter("General", "Name").await?;
    client
        .set_parameter(SetParameter {
            category: "OBWS",
            name: "Test",
            value: Some("Value"),
        })
        .await?;
    client
        .set_parameter(SetParameter {
            category: "OBWS",
            name: "Test",
            value: None,
        })
        .await?;

    Ok(())
}
