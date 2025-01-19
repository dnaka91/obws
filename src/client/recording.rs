use super::Client;
use crate::{error::Result, requests::recording::Request, responses::recording as responses};

/// API functions related to recording.
pub struct Recording<'a> {
    pub(super) client: &'a Client,
}

impl Recording<'_> {
    /// Gets the status of the record output.
    #[doc(alias = "GetRecordStatus")]
    pub async fn status(&self) -> Result<responses::RecordStatus> {
        self.client.send_message(Request::Status).await
    }

    /// Toggles the status of the record output.
    #[doc(alias = "ToggleRecord")]
    pub async fn toggle(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Toggle)
            .await
            .map(|oa| oa.active)
    }

    /// Starts the record output.
    #[doc(alias = "StartRecord")]
    pub async fn start(&self) -> Result<()> {
        self.client.send_message(Request::Start).await
    }

    /// Stops the record output.
    #[doc(alias = "StopRecord")]
    pub async fn stop(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::OutputStopped>(Request::Stop)
            .await
            .map(|os| os.path)
    }

    /// Toggles pause on the record output.
    #[doc(alias = "ToggleRecordPause")]
    pub async fn toggle_pause(&self) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputPaused>(Request::TogglePause)
            .await
            .map(|op| op.paused)
    }

    /// Pauses the record output.
    #[doc(alias = "PauseRecord")]
    pub async fn pause(&self) -> Result<()> {
        self.client.send_message(Request::Pause).await
    }

    /// Resumes the record output.
    #[doc(alias = "ResumeRecord")]
    pub async fn resume(&self) -> Result<()> {
        self.client.send_message(Request::Resume).await
    }

    /// Splits the current file being recorded into a new file.
    #[doc(alias = "SplitRecordFile")]
    pub async fn split_file(&self) -> Result<()> {
        self.client.send_message(Request::SplitFile).await
    }

    /// Adds a new chapter marker to the file currently being recorded.
    ///
    /// **Note:** As of OBS 30.2.0, the only file format supporting this feature is Hybrid MP4.
    #[doc(alias = "CreateRecordChapter")]
    pub async fn create_chapter(&self, name: Option<&str>) -> Result<()> {
        self.client
            .send_message(Request::CreateChapter { name })
            .await
    }
}
