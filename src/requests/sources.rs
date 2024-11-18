//! Requests related to sources.

use std::path::Path;

use serde::Serialize;
use serde_with::skip_serializing_none;

pub use super::ids::SourceId;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetSourceActive")]
    Active {
        /// Identifier of the source to get the active state of.
        #[serde(flatten)]
        source: SourceId<'a>,
    },
    #[serde(rename = "GetSourceScreenshot")]
    TakeScreenshot(TakeScreenshot<'a>),
    #[serde(rename = "SaveSourceScreenshot")]
    SaveScreenshot(SaveScreenshot<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Sources(value)
    }
}

/// Request information for [`crate::client::Sources::take_screenshot`].
#[skip_serializing_none]
#[derive(Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct TakeScreenshot<'a> {
    /// Identifier of the source to take a screenshot of.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Image compression format to use. Use [`crate::client::General::version`] to get compatible
    /// image formats.
    #[serde(rename = "imageFormat")]
    pub format: &'a str,
    /// Width to scale the screenshot to.
    #[serde(rename = "imageWidth")]
    pub width: Option<u32>,
    /// Height to scale the screenshot to.
    #[serde(rename = "imageHeight")]
    pub height: Option<u32>,
    /// Compression quality to use. 0 for high compression, 100 for uncompressed. -1 to use
    /// "default".
    #[serde(rename = "imageCompressionQuality")]
    pub compression_quality: Option<i32>,
}

/// Request information for [`crate::client::Sources::save_screenshot`].
#[skip_serializing_none]
#[derive(Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SaveScreenshot<'a> {
    /// Identifier of the source to take a screenshot of.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Image compression format to use. Use [`crate::client::General::version`] to get compatible
    /// image formats.
    #[serde(rename = "imageFormat")]
    pub format: &'a str,
    /// Width to scale the screenshot to.
    #[serde(rename = "imageWidth")]
    pub width: Option<u32>,
    /// Height to scale the screenshot to.
    #[serde(rename = "imageHeight")]
    pub height: Option<u32>,
    /// Compression quality to use. 0 for high compression, 100 for uncompressed. -1 to use
    /// "default".
    #[serde(rename = "imageCompressionQuality")]
    pub compression_quality: Option<i32>,
    /// Path to save the screenshot file to. For example `C:\Users\user\Desktop\screenshot.png`.
    #[serde(rename = "imageFilePath")]
    pub file_path: &'a Path,
}
