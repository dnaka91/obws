//! Requests related to recording.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetRecordStatus")]
    Status,
    #[serde(rename = "ToggleRecord")]
    Toggle,
    #[serde(rename = "StartRecord")]
    Start,
    #[serde(rename = "StopRecord")]
    Stop,
    #[serde(rename = "ToggleRecordPause")]
    TogglePause,
    #[serde(rename = "PauseRecord")]
    Pause,
    #[serde(rename = "ResumeRecord")]
    Resume,
    #[serde(rename = "SplitRecordFile")]
    SplitFile,
    #[serde(rename = "CreateRecordChapter")]
    CreateChapter {
        /// Name of the new chapter.
        #[serde(rename = "chapterName")]
        name: Option<&'a str>,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Recording(value)
    }
}
