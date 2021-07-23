#![cfg(feature = "test-integration")]

use anyhow::Result;
use obws::{
    requests::SetProfileParameter,
    responses::{Profiles, SceneCollections},
};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.config();

    let SceneCollections {
        current_scene_collection_name,
        scene_collections,
    } = client.get_scene_collection_list().await?;
    let other = scene_collections
        .iter()
        .find(|sc| *sc != &current_scene_collection_name)
        .unwrap();
    client.set_current_scene_collection(&other).await?;
    client
        .set_current_scene_collection(&current_scene_collection_name)
        .await?;

    let Profiles {
        current_profile_name,
        profiles,
    } = client.get_profile_list().await?;
    let other = profiles
        .iter()
        .find(|p| *p != &current_profile_name)
        .unwrap();
    client.set_current_profile(&other).await?;
    client.set_current_profile(&current_profile_name).await?;

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

    Ok(())
}
