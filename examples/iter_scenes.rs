use std::{env, time::Duration};

use anyhow::Result;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::connect("127.0.0.1", 4455, env::var("OBS_PASSWORD").ok()).await?;

    let scene_list = client.scenes().list().await?;

    for scene in scene_list.scenes.iter().cycle() {
        client.scenes().set_current_program_scene(&scene.id).await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
