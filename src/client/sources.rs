use super::Client;
use crate::{
    requests::sources::{Request, SaveScreenshot, TakeScreenshot},
    responses::sources as responses,
    Result,
};

/// API functions related to sources.
pub struct Sources<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Sources<'a> {
    /// Gets the active and show state of a source.
    pub async fn active(&self, name: &str) -> Result<responses::SourceActive> {
        self.client.send_message(Request::Active { name }).await
    }

    /// Gets a Base64-encoded screenshot of a source.
    ///
    /// The [`TakeScreenshot::width`] and [`TakeScreenshot::height`] parameters are treated as
    /// "scale to inner", meaning the smallest ratio will be used and the aspect ratio of the
    /// original resolution is kept. If [`TakeScreenshot::width`] and [`TakeScreenshot::height`] are
    /// not specified, the compressed image will use the full resolution of the source.
    pub async fn take_screenshot(&self, settings: TakeScreenshot<'_>) -> Result<String> {
        self.client
            .send_message::<_, responses::ImageData>(Request::TakeScreenshot(settings))
            .await
            .map(|id| id.image_data)
    }

    /// Saves a screenshot of a source to the file system.
    ///
    /// The [`SaveScreenshot::width`] and [`SaveScreenshot::height`] parameters are treated as
    /// "scale to inner", meaning the smallest ratio will be used and the aspect ratio of the
    /// original resolution is kept. If [`SaveScreenshot::width`] and [`SaveScreenshot::height`] are
    /// not specified, the compressed image will use the full resolution of the source.
    pub async fn save_screenshot(&self, settings: SaveScreenshot<'_>) -> Result<()> {
        self.client
            .send_message(Request::SaveScreenshot(settings))
            .await
    }
}
