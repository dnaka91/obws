use anyhow::Result;

use crate::common::{self, TEST_TRANSITION};

#[tokio::test]
async fn transitions() -> Result<()> {
    let client = common::new_client().await?;
    let ui = client.ui();
    let client = client.transitions();

    client.list_kinds().await?;
    client.list().await?;

    client.set_current(TEST_TRANSITION).await?;
    let transition = client.current().await?;

    client
        .set_current_duration(transition.duration.unwrap())
        .await?;
    client
        .set_current_settings(transition.settings.unwrap(), None)
        .await?;
    client.current_cursor().await?;

    ui.set_studio_mode_enabled(true).await?;

    client.trigger().await?;
    client.set_tbar_position(0.5, None).await?;

    ui.set_studio_mode_enabled(false).await?;

    Ok(())
}
