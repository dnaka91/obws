use std::{env, time::Duration};

use anyhow::Result;

use obws::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    pretty_env_logger::init();

    let client = Client::connect("localhost", 4444).await?;

    client.login(env::var("OBS_PASSWORD").ok()).await?;

    let scene_list = client.scenes().get_scene_list().await?;

    for scene in scene_list.scenes.iter().cycle() {
        client
            .scenes()
            .set_current_scene(scene.name.clone())
            .await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
