use super::Client;
use crate::{
    requests::{GetSourceScreenshot, RequestType, SaveSourceScreenshot},
    responses, Result,
};

/// API functions related to sources.
pub struct Sources<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Sources<'a> {
    pub async fn get_source_active(&self, source_name: &str) -> Result<responses::SourceActive> {
        self.client
            .send_message(RequestType::GetSourceActive { source_name })
            .await
    }

    pub async fn get_source_screenshot(&self, settings: GetSourceScreenshot<'_>) -> Result<String> {
        self.client
            .send_message::<responses::ImageData>(RequestType::GetSourceScreenshot(settings))
            .await
            .map(|id| id.image_data)
    }

    pub async fn save_source_screenshot(&self, settings: SaveSourceScreenshot<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SaveSourceScreenshot(settings))
            .await
    }
}
