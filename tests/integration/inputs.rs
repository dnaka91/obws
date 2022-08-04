use anyhow::Result;
use obws::{
    common::MonitorType,
    requests::inputs::{SetSettings, Volume},
};
use time::Duration;

use crate::common::{self, INPUT_KIND_BROWSER, TEST_BROWSER, TEST_BROWSER_RENAME, TEST_MEDIA};

#[tokio::test]
async fn inputs() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.inputs();

    client.list(None).await?;
    client.list_kinds(false).await?;
    client.specials().await?;
    client
        .default_settings::<serde_json::Value>(INPUT_KIND_BROWSER)
        .await?;

    let settings = client
        .settings::<serde_json::Value>(TEST_BROWSER)
        .await?
        .settings;
    client
        .set_settings(SetSettings {
            input: TEST_BROWSER,
            settings: &settings,
            overlay: Some(false),
        })
        .await?;

    let muted = client.muted(TEST_MEDIA).await?;
    client.set_muted(TEST_MEDIA, !muted).await?;
    client.set_muted(TEST_MEDIA, muted).await?;
    client.toggle_mute(TEST_MEDIA).await?;
    client.toggle_mute(TEST_MEDIA).await?;

    let volume = client.volume(TEST_MEDIA).await?;
    client
        .set_volume(TEST_MEDIA, Volume::Mul(volume.mul))
        .await?;

    client.set_name(TEST_BROWSER, TEST_BROWSER_RENAME).await?;
    client.set_name(TEST_BROWSER_RENAME, TEST_BROWSER).await?;

    let balance = client.audio_balance(TEST_MEDIA).await?;
    client.set_audio_balance(TEST_MEDIA, balance / 2.0).await?;
    client.set_audio_balance(TEST_MEDIA, balance).await?;

    let offset = client.audio_sync_offset(TEST_MEDIA).await?;
    client
        .set_audio_sync_offset(TEST_MEDIA, Duration::milliseconds(500))
        .await?;
    client.set_audio_sync_offset(TEST_MEDIA, offset).await?;

    let monitor_type = client.audio_monitor_type(TEST_MEDIA).await?;
    client
        .set_audio_monitor_type(TEST_MEDIA, MonitorType::MonitorAndOutput)
        .await?;
    client
        .set_audio_monitor_type(TEST_MEDIA, monitor_type)
        .await?;

    let tracks = client.audio_tracks(TEST_MEDIA).await?;
    client
        .set_audio_tracks(TEST_MEDIA, [Some(!tracks[0]), None, None, None, None, None])
        .await?;
    client
        .set_audio_tracks(TEST_MEDIA, [Some(tracks[0]), None, None, None, None, None])
        .await?;

    Ok(())
}
