#![cfg(feature = "test-integration")]

use anyhow::Result;
use obws::{
    requests::{SetInputSettings, Volume},
    MonitorType,
};
use time::Duration;

use crate::common::{INPUT_KIND_BROWSER, TEST_BROWSER, TEST_BROWSER_RENAME, TEST_MEDIA};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.inputs();

    client.get_input_list(None).await?;
    client.get_input_kind_list(false).await?;
    client
        .get_input_default_settings::<serde_json::Value>(INPUT_KIND_BROWSER)
        .await?;

    let settings = client
        .get_input_settings::<serde_json::Value>(TEST_BROWSER)
        .await?
        .input_settings;
    client
        .set_input_settings(SetInputSettings {
            input_name: TEST_BROWSER,
            input_settings: &settings,
            overlay: false,
        })
        .await?;

    let muted = client.get_input_mute(TEST_MEDIA).await?;
    client.set_input_mute(TEST_MEDIA, !muted).await?;
    client.set_input_mute(TEST_MEDIA, muted).await?;
    client.toggle_input_mute(TEST_MEDIA).await?;
    client.toggle_input_mute(TEST_MEDIA).await?;

    let volume = client.get_input_volume(TEST_MEDIA).await?;
    client
        .set_input_volume(TEST_MEDIA, Volume::Mul(volume.input_volume_mul))
        .await?;

    client
        .set_input_name(TEST_BROWSER, TEST_BROWSER_RENAME)
        .await?;
    client
        .set_input_name(TEST_BROWSER_RENAME, TEST_BROWSER)
        .await?;

    let offset = client.get_input_audio_sync_offset(TEST_MEDIA).await?;
    client
        .set_input_audio_sync_offset(TEST_MEDIA, Duration::milliseconds(500))
        .await?;
    client
        .set_input_audio_sync_offset(TEST_MEDIA, offset)
        .await?;

    let monitor_type = client.get_input_audio_monitor_type(TEST_MEDIA).await?;
    client
        .set_input_audio_monitor_type(TEST_MEDIA, MonitorType::MonitorAndOutput)
        .await?;
    client
        .set_input_audio_monitor_type(TEST_MEDIA, monitor_type)
        .await?;

    Ok(())
}
