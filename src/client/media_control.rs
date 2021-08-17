use chrono::Duration;

use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to media control.
pub struct MediaControl<'a> {
    pub(super) client: &'a Client,
}

impl<'a> MediaControl<'a> {
    /// Pause or play a media source. Supports FFmpeg and VLC media sources (as of OBS v25.0.8).
    ///
    /// - `source_name`: Source name.
    /// - `play_pause`: Whether to pause or play the source. `false` for play, `true` for pause.
    pub async fn play_pause_media(
        &self,
        source_name: &str,
        play_pause: Option<bool>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::PlayPauseMedia {
                source_name,
                play_pause,
            })
            .await
    }

    /// Restart a media source. Supports FFmpeg and VLC media sources (as of OBS v25.0.8).
    ///
    /// - `source_name`: Source name.
    pub async fn restart_media(&self, source_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RestartMedia { source_name })
            .await
    }

    /// Stop a media source. Supports FFmpeg and VLC media sources (as of OBS v25.0.8).
    ///
    /// - `source_name`: Source name.
    pub async fn stop_media(&self, source_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::StopMedia { source_name })
            .await
    }

    /// Skip to the next media item in the play-list. Supports only VLC media source (as of OBS
    /// v25.0.8).
    ///
    /// - `source_name`: Source name.
    pub async fn next_media(&self, source_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::NextMedia { source_name })
            .await
    }

    /// Go to the previous media item in the play-list. Supports only VLC media source (as of OBS
    /// v25.0.8).
    ///
    /// - `source_name`: Source name.
    pub async fn previous_media(&self, source_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::PreviousMedia { source_name })
            .await
    }

    /// Get the length of media in milliseconds. Supports FFmpeg and VLC media sources (as of OBS
    /// v25.0.8).
    ///
    /// Note: For some reason, for the first 5 or so seconds that the media is playing, the total
    /// duration can be off by upwards of `50ms`.
    ///
    /// - `source_name`: Source name.
    pub async fn get_media_duration(&self, source_name: &str) -> Result<Duration> {
        self.client
            .send_message::<responses::MediaDuration>(RequestType::GetMediaDuration { source_name })
            .await
            .map(|md| md.media_duration)
    }

    /// Get the current timestamp of media in milliseconds. Supports FFmpeg and VLC media sources
    /// (as of OBS v25.0.8).
    ///
    /// - `source_name`: Source name.
    pub async fn get_media_time(&self, source_name: &str) -> Result<Duration> {
        self.client
            .send_message::<responses::MediaTime>(RequestType::GetMediaTime { source_name })
            .await
            .map(|mt| mt.timestamp)
    }

    /// Set the timestamp of a media source. Supports FFmpeg and VLC media sources (as of OBS
    /// v25.0.8).
    ///
    /// - `source_name`: Source name.
    /// - `timestamp`: Milliseconds to set the timestamp to.
    pub async fn set_media_time(&self, source_name: &str, timestamp: Duration) -> Result<()> {
        self.client
            .send_message(RequestType::SetMediaTime {
                source_name,
                timestamp,
            })
            .await
    }

    /// Scrub media using a supplied offset. Supports FFmpeg and VLC media sources (as of OBS
    /// v25.0.8).
    ///
    /// Note: Due to processing/network delays, this request is not perfect. The processing rate of
    /// this request has also not been tested.
    ///
    /// - `source_name`: Source name.
    /// - `time_offset`: Millisecond offset (positive or negative) to offset the current media
    ///   position.
    pub async fn scrub_media(&self, source_name: &str, time_offset: Duration) -> Result<()> {
        self.client
            .send_message(RequestType::ScrubMedia {
                source_name,
                time_offset,
            })
            .await
    }

    /// Get the current playing state of a media source. Supports FFmpeg and VLC media sources (as
    /// of OBS v25.0.8).
    ///
    /// - `source_name`: Source name.
    pub async fn get_media_state(&self, source_name: &str) -> Result<responses::MediaState> {
        self.client
            .send_message::<responses::GetMediaState>(RequestType::GetMediaState { source_name })
            .await
            .map(|msr| msr.media_state)
    }
}
