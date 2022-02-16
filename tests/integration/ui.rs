use anyhow::Result;

use crate::common;

#[tokio::test]
async fn ui() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.ui();

    let enabled = client.get_studio_mode_enabled().await?;
    client.set_studio_mode_enabled(!enabled).await?;
    client.set_studio_mode_enabled(enabled).await?;

    Ok(())
}
