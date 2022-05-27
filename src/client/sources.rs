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
    pub async fn active(&self, name: &str) -> Result<responses::SourceActive> {
        self.client
            .send_message(RequestType::GetSourceActive { name })
            .await
    }

    /// Gets a Base64-encoded screenshot of a source.
    ///
    /// The [`width`] and [`height`] parameters are treated as "scale to inner", meaning
    /// the smallest ratio will be used and the aspect ratio of the original resolution is kept. If
    /// [`width`] and [`height`] are not specified, the compressed image will use the
    /// full resolution of the source.
    ///
    /// [`width`]: GetSourceScreenshot::width
    /// [`height`]: GetSourceScreenshot::height
    pub async fn take_screenshot(&self, settings: GetSourceScreenshot<'_>) -> Result<String> {
        self.client
            .send_message::<responses::ImageData>(RequestType::GetSourceScreenshot(settings))
            .await
            .map(|id| id.image_data)
    }

    /// Saves a screenshot of a source to the file system.
    ///
    /// The [`width`] and [`height`] parameters are treated as "scale to inner", meaning
    /// the smallest ratio will be used and the aspect ratio of the original resolution is kept. If
    /// [`width`] and [`height`] are not specified, the compressed image will use the
    /// full resolution of the source.
    ///
    /// [`width`]: SaveSourceScreenshot::width
    /// [`height`]: SaveSourceScreenshot::height
    pub async fn save_screenshot(&self, settings: SaveSourceScreenshot<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SaveSourceScreenshot(settings))
            .await
    }
}
