use anyhow::Result;
use obws::requests::ui::{Location, OpenSourceProjector, OpenVideoMixProjector, VideoMixType};

use crate::common::{self, TEST_TEXT};

#[tokio::test]
async fn ui() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.ui();

    let enabled = client.studio_mode_enabled().await?;
    client.set_studio_mode_enabled(!enabled).await?;
    client.set_studio_mode_enabled(enabled).await?;

    client.list_monitors().await?;
    client
        .open_video_mix_projector(OpenVideoMixProjector {
            r#type: VideoMixType::Preview,
            location: None,
        })
        .await?;
    client
        .open_source_projector(OpenSourceProjector {
            source: TEST_TEXT,
            location: Some(Location::MonitorIndex(-1)),
        })
        .await?;

    Ok(())
}
