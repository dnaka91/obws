use std::env;

use anyhow::Result;

use obws::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=debug");
    pretty_env_logger::init();

    let client = Client::connect("localhost", 4444).await?;

    let version = client.general().get_version().await?;
    println!("{:#?}", version);

    client.login(env::var("OBS_PASSWORD").ok()).await?;

    let scene_list = client.scenes().get_scene_list().await?;
    println!("{:#?}", scene_list);

    Ok(())
}
