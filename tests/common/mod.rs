use std::sync::Once;
use std::time::Duration;

use anyhow::{ensure, Result};
use obws::{
    responses::{Output, Scene, SceneCollection, SourceListItem},
    Client,
};
use tokio::time;

pub const TEST_OUTPUT: &str = "virtualcam_output";
pub const TEST_COLLECTION: &str = "OBWS-TEST";
pub const TEST_SCENE: &str = "OBWS-TEST-Scene";
pub const TEXT_SOURCE: &str = "OBWS-TEST-Text";

static INIT: Once = Once::new();

pub async fn new_client() -> Result<Client> {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        pretty_env_logger::init();
    });

    let client = Client::connect("localhost", 4444).await?;
    client.login(std::env::var("OBS_PASSWORD").ok()).await?;

    let collections = client.scene_collections().list_scene_collections().await?;
    ensure!(
        collections.iter().any(is_required_scene_collection),
        "scene collection `{}` not found, required for all tests",
        TEST_COLLECTION
    );

    client
        .scene_collections()
        .set_current_scene_collection("OBWS-TEST")
        .await?;

    // Give OBS some time to load the scene collection
    time::sleep(Duration::from_millis(500)).await;

    ensure_obs_setup(&client).await?;

    Ok(client)
}

async fn ensure_obs_setup(client: &Client) -> Result<()> {
    let outputs = client.outputs().list_outputs().await?;
    ensure!(
        outputs.iter().any(is_required_output),
        "output `{}` not found, required for output tests",
        TEST_OUTPUT
    );

    let scenes = client.scenes().get_scene_list().await?;
    ensure!(
        scenes.scenes.iter().any(is_required_scene),
        "scene `{}` not found, required for scenes tests",
        TEST_SCENE
    );

    let sources = client.sources().get_sources_list().await?;
    ensure!(
        sources.iter().any(is_required_source),
        "text source `{}` not found, required for sources tests",
        TEXT_SOURCE
    );

    let special_sources = client.sources().get_special_sources().await?;
    ensure!(
        special_sources.desktop_1.is_some(),
        "desktop audio device required for sources tests"
    );

    Ok(())
}

fn is_required_output(output: &Output) -> bool {
    output.name == TEST_OUTPUT
}

fn is_required_scene_collection(output: &SceneCollection) -> bool {
    output.sc_name == TEST_COLLECTION
}

fn is_required_scene(scene: &Scene) -> bool {
    scene.name == TEST_SCENE
}

fn is_required_source(source: &SourceListItem) -> bool {
    source.name == TEXT_SOURCE && source.ty == "input" && source.type_id == "text_ft2_source_v2"
}
