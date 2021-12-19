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
    /// Gets the active and show state of a source.
    ///
    /// - `source_name`: Name of the source to get the active state of.
    pub async fn get_source_active(&self, source_name: &str) -> Result<responses::SourceActive> {
        self.client
            .send_message(RequestType::GetSourceActive { source_name })
            .await
    }

    /// Gets a Base64-encoded screenshot of a source.
    ///
    /// The [`image_width`] and [`image_height`] parameters are treated as "scale to inner", meaning
    /// the smallest ratio will be used and the aspect ratio of the original resolution is kept. If
    /// [`image_width`] and [`image_height`] are not specified, the compressed image will use the
    /// full resolution of the source.
    ///
    /// [`image_width`]: GetSourceScreenshot::image_width
    /// [`image_height`]: GetSourceScreenshot::image_height
    pub async fn get_source_screenshot(&self, settings: GetSourceScreenshot<'_>) -> Result<String> {
        self.client
            .send_message::<responses::ImageData>(RequestType::GetSourceScreenshot(settings))
            .await
            .map(|id| id.image_data)
    }

    /// Saves a screenshot of a source to the filesystem.
    ///
    /// The [`image_width`] and [`image_height`] parameters are treated as "scale to inner", meaning
    /// the smallest ratio will be used and the aspect ratio of the original resolution is kept. If
    /// [`image_width`] and [`image_height`] are not specified, the compressed image will use the
    /// full resolution of the source.
    ///
    /// [`image_width`]: SaveSourceScreenshot::image_width
    /// [`image_height`]: SaveSourceScreenshot::image_height
    pub async fn save_source_screenshot(&self, settings: SaveSourceScreenshot<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SaveSourceScreenshot(settings))
            .await
    }
}
