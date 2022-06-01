use anyhow::Result;
use obws::requests::config::{Realm, SetPersistentData};

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
