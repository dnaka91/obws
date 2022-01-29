use std::{env, time::Duration};

use anyhow::Result;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    tracing_subscriber::fmt::init();

    let client = Client::connect("localhost", 4455, env::var("OBS_PASSWORD").ok()).await?;

    let scene_list = client.scenes().get_scene_list().await?;

    for scene in scene_list.scenes.iter().cycle() {
        client
            .scenes()
            .set_current_program_scene(&scene.scene_name)
            .await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
