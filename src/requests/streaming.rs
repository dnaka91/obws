//! Requests related to streaming.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetStreamStatus")]
    GetStreamStatus,
    #[serde(rename = "ToggleStream")]
    ToggleStream,
    #[serde(rename = "StartStream")]
    StartStream,
    #[serde(rename = "StopStream")]
    StopStream,
    #[serde(rename = "SendStreamCaption")]
    SendStreamCaption {
        /// Caption text.
        #[serde(rename = "captionText")]
        caption_text: &'a str,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Streaming(value)
    }
}
