use std::time::Duration;

use anyhow::Result;
use obws::{
    requests::{Realm, SetPersistentData, SetProfileParameter},
    responses::{Profiles, SceneCollections},
};
use tokio::time;

use crate::common;

#[tokio::test]
async fn config() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.config();

    client
        .set_persistent_data(SetPersistentData {
            realm: Realm::Profile,
            slot_name: "obws-test",
            slot_value: &true.into(),
        })
        .await?;
    client
        .get_persistent_data(Realm::Profile, "obws-test")
        .await?;

    let SceneCollections {
        current,
        collections,
    } = client.list_scene_collections().await?;
    let other = collections.iter().find(|sc| *sc != &current).unwrap();
    client.set_current_scene_collection(other).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.set_current_scene_collection(&current).await?;
    time::sleep(Duration::from_secs(1)).await;

    let Profiles { current, profiles } = client.list_profiles().await?;
    let other = profiles.iter().find(|p| *p != &current).unwrap();
    client.set_current_profile(other).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.set_current_profile(&current).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.create_profile("OBWS-TEST-New-Profile").await?;
    client.remove_profile("OBWS-TEST-New-Profile").await?;

    client.get_profile_parameter("General", "Name").await?;
    client
        .set_profile_parameter(SetProfileParameter {
            category: "OBWS",
            name: "Test",
            value: Some("Value"),
        })
        .await?;
    client
        .set_profile_parameter(SetProfileParameter {
            category: "OBWS",
            name: "Test",
            value: None,
        })
        .await?;

    let settings = client.video_settings().await?;
    client.set_video_settings(settings.into()).await?;

    let settings = client
        .stream_service_settings::<serde_json::Value>()
        .await?;
    client
        .set_stream_service_settings(&settings.r#type, &settings.settings)
        .await?;

    client.record_directory().await?;

    Ok(())
}
