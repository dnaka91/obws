use std::path::{Path, PathBuf};

use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to recording.
pub struct Recording<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Recording<'a> {
    /// Get current recording status.
    pub async fn get_recording_status(&self) -> Result<responses::RecordingStatus> {
        self.client
            .send_message(RequestType::GetRecordingStatus)
            .await
    }

    /// Toggle recording on or off (depending on the current recording state).
    pub async fn start_stop_recording(&self) -> Result<()> {
        self.client
            .send_message(RequestType::StartStopRecording)
            .await
    }

    /// Start recording. Will return an `error` if recording is already active.
    pub async fn start_recording(&self) -> Result<()> {
        self.client.send_message(RequestType::StartRecording).await
    }

    /// Stop recording. Will return an `error` if recording is not active.
    pub async fn stop_recording(&self) -> Result<()> {
        self.client.send_message(RequestType::StopRecording).await
    }

    /// Pause the current recording. Returns an `error` if recording is not active or already
    /// paused.
    pub async fn pause_recording(&self) -> Result<()> {
        self.client.send_message(RequestType::PauseRecording).await
    }

    /// Resume/un-pause the current recording (if paused). Returns an error if recording is not
    /// active or not paused.
    pub async fn resume_recording(&self) -> Result<()> {
        self.client.send_message(RequestType::ResumeRecording).await
    }

    /// Please note: if this is called while a recording is in progress, the change won't be applied
    /// immediately and will be effective on the next recording.
    ///
    /// - `rec_folder`: Path of the recording folder.
    pub async fn set_recording_folder(&self, rec_folder: &Path) -> Result<()> {
        self.client
            .send_message(RequestType::SetRecordingFolder { rec_folder })
            .await
    }

    /// Get the path of the current recording folder.
    pub async fn get_recording_folder(&self) -> Result<PathBuf> {
        self.client
            .send_message::<responses::RecordingFolder>(RequestType::GetRecordingFolder)
            .await
            .map(|rf| rf.rec_folder)
    }
}
