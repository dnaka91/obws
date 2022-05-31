use std::{env, sync::Once};

use anyhow::{ensure, Result};
use obws::{
    responses::{Input, Scene, SourceFilter},
    Client,
};

pub const TEST_PROFILE: &str = "OBWS-TEST";
pub const TEST_SCENE: &str = "OBWS-TEST-Scene";
pub const TEST_SCENE_2: &str = "OBWS-TEST-Scene2";
pub const TEST_SCENE_RENAME: &str = "OBWS-TEST-Scene-Renamed";
pub const TEST_SCENE_CREATE: &str = "OBWS-TEST-Scene-Created";
pub const TEST_TEXT: &str = "OBWS-TEST-Text";
pub const TEST_TEXT_2: &str = "OBWS-TEST-Text2";
pub const TEST_BROWSER: &str = "OBWS-TEST-Browser";
pub const TEST_BROWSER_RENAME: &str = "OBWS-TEST-Browser-Renamed";
pub const TEST_MEDIA: &str = "OBWS-TEST-Media";
pub const TEST_GROUP: &str = "OBWS-TEST-Group";
pub const TEST_TRANSITION: &str = "OBWS-TEST-Transition";
pub const TEST_FILTER: &str = "OBWS-TEST-Filter";
pub const TEST_FILTER_2: &str = "OBWS-TEST-Filter2";
pub const TEST_FILTER_RENAME: &str = "OBWS-TEST-Filter-Renamed";
pub const INPUT_KIND_TEXT_FT2: &str = "text_ft2_source_v2";
pub const INPUT_KIND_BROWSER: &str = "browser_source";
pub const INPUT_KIND_VLC: &str = "vlc_source";
pub const FILTER_COLOR: &str = "color_filter";

static INIT: Once = Once::new();

pub async fn new_client() -> Result<Client> {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
        tracing_subscriber::fmt::init();
    });

    let host = env::var("OBS_HOST").unwrap_or_else(|_| "localhost".to_owned());
    let port = env::var("OBS_PORT")
        .map(|p| p.parse())
        .unwrap_or(Ok(4455))?;
    let client = Client::connect(host, port, env::var("OBS_PASSWORD").ok()).await?;

    ensure_obs_setup(&client).await?;

    Ok(client)
}

async fn ensure_obs_setup(client: &Client) -> Result<()> {
    let scenes = client.scenes().list().await?;
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
    ensure!(
        !scenes.scenes.iter().any(is_renamed_scene),
        "scene `{}` found, must NOT be present for scenes tests",
        TEST_SCENE_RENAME
    );
    ensure!(
        !scenes.scenes.iter().any(is_created_scene),
        "scene `{}` found, must NOT be present for scenes tests",
        TEST_SCENE_CREATE
    );

    let groups = client.scenes().list_groups().await?;
    ensure!(
        groups.iter().map(String::as_str).any(is_required_group),
        "group `{}` not found, required for scenes and scene items tests",
        TEST_GROUP
    );

    let inputs = client.inputs().list(None).await?;
    ensure!(
        inputs.iter().any(is_required_text_input),
        "text input `{}` not found, required for inputs tests",
        TEST_TEXT
    );
    ensure!(
        inputs.iter().any(is_required_text_2_input),
        "text input `{}` not found, required for inputs tests",
        TEST_TEXT
    );
    ensure!(
        inputs.iter().any(is_required_browser_input),
        "media input `{}` not found, required for inputs tests",
        TEST_BROWSER
    );
    ensure!(
        inputs.iter().any(is_required_media_input),
        "media input `{}` not found, required for inputs tests",
        TEST_MEDIA
    );
    ensure!(
        !inputs.iter().any(is_renamed_input),
        "browser input `{}` found, must NOT be present for inputs tests",
        TEST_BROWSER_RENAME
    );

    let filters = client.filters().list(TEST_TEXT).await?;
    ensure!(
        filters.iter().any(is_required_filter),
        "filter `{}` not found, required for filters tests",
        TEST_FILTER
    );
    ensure!(
        !filters.iter().any(is_filter_2),
        "filter `{}` found, must NOT be present for filters tests",
        TEST_FILTER_2
    );
    ensure!(
        !filters.iter().any(is_renamed_filter),
        "filter `{}` found, must NOT be present for filters tests",
        TEST_FILTER_RENAME
    );

    let profiles = client.profiles().list().await?.profiles;
    ensure!(
        profiles.iter().map(String::as_str).any(is_required_profile),
        "profile `{}` not found, required for profiles tests",
        TEST_PROFILE
    );

    let studio_mode_enabled = client.ui().studio_mode_enabled().await?;
    ensure!(
        !studio_mode_enabled,
        "studio mode enabled, required to be disabled for studio mode tests"
    );

    let recording_active = client.recording().status().await?.active;
    ensure!(
        !recording_active,
        "recording active, required to be stopped for recording tests"
    );

    let virtual_cam_active = client.outputs().virtual_cam_status().await?;
    ensure!(
        !virtual_cam_active,
        "virtual cam active, required to be stopped for outputs tests"
    );

    let replay_buffer_active = client.outputs().replay_buffer_status().await?;
    ensure!(
        !replay_buffer_active,
        "replay buffer active, required to be stopped for outputs tests"
    );

    client
        .scenes()
        .set_current_program_scene(TEST_SCENE)
        .await?;

    Ok(())
}

fn is_required_scene(scene: &Scene) -> bool {
    scene.name == TEST_SCENE
}

fn is_required_scene_2(scene: &Scene) -> bool {
    scene.name == TEST_SCENE_2
}

fn is_renamed_scene(scene: &Scene) -> bool {
    scene.name == TEST_SCENE_RENAME
}

fn is_created_scene(scene: &Scene) -> bool {
    scene.name == TEST_SCENE_CREATE
}

fn is_required_group(group: &str) -> bool {
    group == TEST_GROUP
}

fn is_required_text_input(input: &Input) -> bool {
    input.name == TEST_TEXT && is_text_input(input)
}

fn is_required_text_2_input(input: &Input) -> bool {
    input.name == TEST_TEXT_2 && is_text_input(input)
}

fn is_required_browser_input(input: &Input) -> bool {
    input.name == TEST_BROWSER && is_browser_input(input)
}

fn is_required_media_input(input: &Input) -> bool {
    input.name == TEST_MEDIA && is_media_input(input)
}

fn is_renamed_input(input: &Input) -> bool {
    input.name == TEST_BROWSER_RENAME
}

fn is_text_input(input: &Input) -> bool {
    input.kind == INPUT_KIND_TEXT_FT2
}

fn is_browser_input(input: &Input) -> bool {
    input.kind == INPUT_KIND_BROWSER
}

fn is_media_input(input: &Input) -> bool {
    input.kind == INPUT_KIND_VLC
}

fn is_required_filter(filter: &SourceFilter) -> bool {
    filter.name == TEST_FILTER
}

fn is_filter_2(filter: &SourceFilter) -> bool {
    filter.name == TEST_FILTER_2
}

fn is_renamed_filter(filter: &SourceFilter) -> bool {
    filter.name == TEST_FILTER_RENAME
}

fn is_required_profile(profile: &str) -> bool {
    profile == TEST_PROFILE
}

#[macro_export]
macro_rules! wait_for {
    ($expression:expr, $pattern:pat) => {{
        use futures_util::stream::StreamExt;

        while let Some(event) = $expression.next().await {
            if matches!(event, $pattern) {
                break;
            }
        }
    }};
}
