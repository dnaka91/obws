#![cfg(feature = "test-integration")]

use std::time::Duration;

use anyhow::Result;
use obws::{
    requests::{Realm, SetPersistentData, SetProfileParameter},
    responses::{Profiles, SceneCollections},
};
use tokio::time;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
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
        current_scene_collection_name,
        scene_collections,
    } = client.get_scene_collection_list().await?;
    let other = scene_collections
        .iter()
        .find(|sc| *sc != &current_scene_collection_name)
        .unwrap();
    client.set_current_scene_collection(other).await?;
    time::sleep(Duration::from_secs(1)).await;
    client
        .set_current_scene_collection(&current_scene_collection_name)
        .await?;
    time::sleep(Duration::from_secs(1)).await;

    let Profiles {
        current_profile_name,
        profiles,
    } = client.get_profile_list().await?;
    let other = profiles
        .iter()
        .find(|p| *p != &current_profile_name)
        .unwrap();
    client.set_current_profile(other).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.set_current_profile(&current_profile_name).await?;
    time::sleep(Duration::from_secs(1)).await;
    client.create_profile("OBWS-TEST-New-Profile").await?;
    client.remove_profile("OBWS-TEST-New-Profile").await?;

    client.get_profile_parameter("General", "Name").await?;
    client
        .set_profile_parameter(SetProfileParameter {
            parameter_category: "OBWS",
            parameter_name: "Test",
            parameter_value: Some("Value"),
        })
        .await?;
    client
        .set_profile_parameter(SetProfileParameter {
            parameter_category: "OBWS",
            parameter_name: "Test",
            parameter_value: None,
        })
        .await?;

    let settings = client.get_video_settings().await?;
    client.set_video_settings(settings.into()).await?;

    let settings = client
        .get_stream_service_settings::<serde_json::Value>()
        .await?;
    client
        .set_stream_service_settings(
            &settings.stream_service_type,
            &settings.stream_service_settings,
        )
        .await?;

    Ok(())
}
