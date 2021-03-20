#![cfg(feature = "test-integration")]

use anyhow::Result;
use obws::requests::{Projector, ProjectorType, QtGeometry, QtRect};
use serde_json::json;

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.general();

    client.get_version().await?;

    client.get_auth_required().await?;

    let original = client.get_filename_formatting().await?;
    client.set_filename_formatting("test").await?;
    client.set_filename_formatting(&original).await?;

    client.get_stats().await?;

    client
        .broadcast_custom_message("test", &json! {{"greeting":"hello"}})
        .await?;

    client.get_video_info().await?;

    // Currently no API function available to close the projector again.
    client
        .open_projector(Projector {
            ty: Some(ProjectorType::Multiview),
            geometry: Some(&QtGeometry::new(QtRect {
                left: 100,
                top: 100,
                right: 300,
                bottom: 300,
            })),
            ..Default::default()
        })
        .await?;

    client.trigger_hotkey_by_name("ReplayBuffer.Save").await?;
    client.trigger_hotkey_by_sequence("OBS_KEY_P", &[]).await?;

    Ok(())
}
