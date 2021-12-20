use std::{env, sync::Once, time::Duration};

use anyhow::{ensure, Result};
use obws::{
    requests::SceneItem,
    responses::{Output, Profile, Scene, SceneCollection, SourceListItem, Transition},
    Client,
};
use tokio::time;

pub const TEST_OUTPUT: &str = "virtualcam_output";
pub const TEST_COLLECTION: &str = "OBWS-TEST";
pub const TEST_PROFILE: &str = "OBWS-TEST";
pub const TEST_SCENE: &str = "OBWS-TEST-Scene";
pub const TEST_SCENE_2: &str = "OBWS-TEST-Scene2";
pub const TEXT_SOURCE: &str = "OBWS-TEST-Text";
pub const TEXT_SOURCE_2: &str = "OBWS-TEST-Text2";
pub const TEST_TRANSITION: &str = "OBWS-TEST-Transition";
pub const TEST_TRANSITION_2: &str = "OBWS-TEST-Transition2";
pub const TEST_BROWSER: &str = "OBWS-TEST-Browser";
pub const TEST_MEDIA: &str = "OBWS-TEST-Media";
pub const SOURCE_KIND_TEXT_FT2: &str = "text_ft2_source_v2";
pub const SOURCE_KIND_BROWSER: &str = "browser_source";
pub const SOURCE_KIND_VLC: &str = "vlc_source";

const SCENE_ORDER: &[SceneItem] = &[
    SceneItem {
        id: None,
        name: Some(TEXT_SOURCE),
    },
    SceneItem {
        id: None,
        name: Some(TEXT_SOURCE_2),
    },
    SceneItem {
        id: None,
        name: Some(TEST_BROWSER),
    },
    SceneItem {
        id: None,
        name: Some(TEST_MEDIA),
    },
];

static INIT: Once = Once::new();

pub async fn new_client() -> Result<Client> {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        pretty_env_logger::init();
    });

    let host = env::var("OBS_HOST").unwrap_or_else(|_| "localhost".to_owned());
    let client = Client::connect(host, 4444).await?;
    client.login(env::var("OBS_PASSWORD").ok()).await?;

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
    time::sleep(Duration::from_secs(1)).await;

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
    ensure!(
        scenes.scenes.iter().any(is_required_scene_2),
        "scene `{}` not found, required for scenes tests",
        TEST_SCENE
    );

    let sources = client.sources().get_sources_list().await?;
    ensure!(
        sources.iter().any(is_required_source),
        "text source `{}` not found, required for sources tests",
        TEXT_SOURCE
    );
    ensure!(
        sources.iter().any(is_required_source_2),
        "text source `{}` not found, required for sources tests",
        TEXT_SOURCE_2
    );
    ensure!(
        sources.iter().any(is_required_browser_source),
        "media source `{}` not found, required for sources tests",
        TEST_BROWSER
    );
    ensure!(
        sources.iter().any(is_required_media_source),
        "media source `{}` not found, required for media control tests",
        TEST_MEDIA
    );

    let special_sources = client.sources().get_special_sources().await?;
    ensure!(
        special_sources.desktop_1.is_some(),
        "desktop audio device required for sources tests"
    );

    let profiles = client.profiles().list_profiles().await?;
    ensure!(
        profiles.iter().any(is_required_profile),
        "profile `{}` not found, required for profiles tests",
        TEST_PROFILE
    );

    let studio_mode_enabled = client.studio_mode().get_studio_mode_status().await?;
    ensure!(
        !studio_mode_enabled,
        "studio mode enabled, required to be disabled for studio mode tests"
    );

    let transitions = client.transitions().get_transition_list().await?;
    ensure!(
        transitions.transitions.iter().any(is_required_transition),
        "transition `{}` not found, required for transitions tests",
        TEST_TRANSITION
    );
    ensure!(
        transitions.transitions.iter().any(is_required_transition_2),
        "transition `{}` not found, required for transitions tests",
        TEST_TRANSITION
    );

    client.scenes().set_current_scene(TEST_SCENE).await?;

    let sources = client.scenes().get_current_scene().await?.sources;
    ensure!(
        sources.len() == 4,
        "scene `{}` must have exactly 4 scene items",
        TEST_SCENE
    );

    if sources[0].name != TEXT_SOURCE
        || sources[1].name != TEXT_SOURCE_2
        || sources[2].name != TEST_BROWSER
        || sources[3].name != TEST_MEDIA
    {
        client
            .scenes()
            .reorder_scene_items(Some(TEST_SCENE), SCENE_ORDER)
            .await?;
    }

    client
        .transitions()
        .set_current_transition(TEST_TRANSITION)
        .await?;

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

fn is_required_scene_2(scene: &Scene) -> bool {
    scene.name == TEST_SCENE_2
}

fn is_required_source(source: &SourceListItem) -> bool {
    source.name == TEXT_SOURCE && is_text_input_source(source)
}

fn is_required_source_2(source: &SourceListItem) -> bool {
    source.name == TEXT_SOURCE_2 && is_text_input_source(source)
}

fn is_required_browser_source(source: &SourceListItem) -> bool {
    source.name == TEST_BROWSER && is_browser_input_source(source)
}

fn is_required_media_source(source: &SourceListItem) -> bool {
    source.name == TEST_MEDIA && is_media_input_source(source)
}

fn is_text_input_source(source: &SourceListItem) -> bool {
    source.ty == "input" && source.type_id == SOURCE_KIND_TEXT_FT2
}

fn is_browser_input_source(source: &SourceListItem) -> bool {
    source.ty == "input" && source.type_id == SOURCE_KIND_BROWSER
}

fn is_media_input_source(source: &SourceListItem) -> bool {
    source.ty == "input" && source.type_id == SOURCE_KIND_VLC
}

fn is_required_profile(profile: &Profile) -> bool {
    profile.profile_name == TEST_PROFILE
}

fn is_required_transition(transition: &Transition) -> bool {
    transition.name == TEST_TRANSITION
}

fn is_required_transition_2(transition: &Transition) -> bool {
    transition.name == TEST_TRANSITION_2
}

#[allow(unused_macros)]
macro_rules! wait_for {
    ($expression:expr, $pattern:pat) => {
        while let Some(Event { ty, .. }) = $expression.next().await {
            if matches!(ty, $pattern) {
                break;
            }
        }
    };
}
