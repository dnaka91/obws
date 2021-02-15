use chrono::Duration;
use serde::de::DeserializeOwned;

use super::Client;
use crate::common::MonitorType;
use crate::requests::{
    AddFilter, CreateSource, MoveFilter, ReorderFilter, RequestType, SourceFilterSettings,
    SourceFilterVisibility, SourceScreenshot, SourceSettings, TextFreetype2Properties,
    TextGdiPlusProperties, Volume,
};
use crate::responses;
use crate::Result;

/// API functions related to sources.
pub struct Sources<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Sources<'a> {
    /// List the media state of all media sources (vlc and media source).
    pub async fn get_media_sources_list(&self) -> Result<Vec<responses::MediaSource>> {
        self.client
            .send_message::<responses::MediaSourcesList>(RequestType::GetMediaSourcesList)
            .await
            .map(|ms| ms.media_sources)
    }

    /// Create a source and add it as a scene item to a scene.
    pub async fn create_source(&self, source: CreateSource<'_>) -> Result<i64> {
        self.client
            .send_message::<responses::SourceItemId>(RequestType::CreateSource(source))
            .await
            .map(|sii| sii.item_id)
    }

    /// List all sources available in the running OBS instance.
    pub async fn get_sources_list(&self) -> Result<Vec<responses::SourceListItem>> {
        self.client
            .send_message::<responses::SourcesList>(RequestType::GetSourcesList)
            .await
            .map(|sl| sl.sources)
    }

    /// Get a list of all available sources types.
    pub async fn get_sources_types_list(&self) -> Result<Vec<responses::SourceTypeItem>> {
        self.client
            .send_message::<responses::SourceTypesList>(RequestType::GetSourceTypesList)
            .await
            .map(|stl| stl.types)
    }

    /// Get the volume of the specified source. Default response uses mul format, NOT SLIDER
    /// PERCENTAGE.
    ///
    /// - `source`: Source name.
    /// - `use_decibel`: Output volume in decibels of attenuation instead of amplitude/mul.
    pub async fn get_volume(
        &self,
        source: &str,
        use_decibel: Option<bool>,
    ) -> Result<responses::Volume> {
        self.client
            .send_message(RequestType::GetVolume {
                source,
                use_decibel,
            })
            .await
    }

    /// Set the volume of the specified source. Default request format uses mul, NOT SLIDER
    /// PERCENTAGE.
    pub async fn set_volume(&self, volume: Volume<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetVolume(volume))
            .await
    }

    /// Get the mute status of a specified source.
    ///
    /// - `source`: Source name.
    pub async fn get_mute(&self, source: &str) -> Result<responses::Mute> {
        self.client
            .send_message(RequestType::GetMute { source })
            .await
    }

    /// Sets the mute status of a specified source.
    ///
    /// - `source`: Source name.
    /// - `mute`: Desired mute status.
    pub async fn set_mute(&self, source: &str, mute: bool) -> Result<()> {
        self.client
            .send_message(RequestType::SetMute { source, mute })
            .await
    }

    /// Inverts the mute status of a specified source.
    ///
    /// - `source`: Source name.
    pub async fn toggle_mute(&self, source: &str) -> Result<()> {
        self.client
            .send_message(RequestType::ToggleMute { source })
            .await
    }

    /// Get the audio's active status of a specified source.
    ///
    /// - `source_name`: Source name.
    pub async fn get_audio_active(&self, source_name: &str) -> Result<bool> {
        self.client
            .send_message::<responses::AudioActive>(RequestType::GetAudioActive { source_name })
            .await
            .map(|aa| aa.audio_active)
    }

    /// Rename an existing source.
    ///
    /// Note: If the new name already exists as a source, obs-websocket will return an error.
    ///
    /// - `source_name`: Source name.
    /// - `new_name`: New source name.
    pub async fn set_source_name(&self, source_name: &str, new_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceName {
                source_name,
                new_name,
            })
            .await
    }

    /// Set the audio sync offset of a specified source.
    ///
    /// - `source`: Source name.
    /// - `offset`: The desired audio sync offset (in nanoseconds).
    pub async fn set_sync_offset(&self, source: &str, offset: Duration) -> Result<()> {
        self.client
            .send_message(RequestType::SetSyncOffset { source, offset })
            .await
    }

    /// Get the audio sync offset of a specified source.
    ///
    /// - `source`: Source name.
    pub async fn get_sync_offset(&self, source: &str) -> Result<responses::SyncOffset> {
        self.client
            .send_message(RequestType::GetSyncOffset { source })
            .await
    }

    /// Get settings of the specified source.
    ///
    /// - `source_name`: Source name.
    /// - `source_type`: Type of the specified source. Useful for type-checking if you expect a
    ///   specific settings schema.
    pub async fn get_source_settings<T>(
        &self,
        source_name: &str,
        source_type: Option<&str>,
    ) -> Result<responses::SourceSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message(RequestType::GetSourceSettings {
                source_name,
                source_type,
            })
            .await
    }

    /// Set settings of the specified source.
    pub async fn set_source_settings<T>(
        &self,
        source_settings: SourceSettings<'_>,
    ) -> Result<responses::SourceSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message(RequestType::SetSourceSettings(source_settings))
            .await
    }

    /// Get the current properties of a Text GDI Plus source.
    ///
    /// - `source`: Source name.
    pub async fn get_text_gdi_plus_properties(
        &self,
        source: &str,
    ) -> Result<responses::TextGdiPlusProperties> {
        self.client
            .send_message(RequestType::GetTextGDIPlusProperties { source })
            .await
    }

    /// Set the current properties of a Text GDI Plus source.
    pub async fn set_text_gdi_plus_properties(
        &self,
        properties: TextGdiPlusProperties<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetTextGDIPlusProperties(Box::new(properties)))
            .await
    }

    /// Get the current properties of a Text Freetype 2 source.
    ///
    /// - `source`: Source name.
    pub async fn get_text_freetype2_properties(
        &self,
        source: &str,
    ) -> Result<responses::TextFreetype2Properties> {
        self.client
            .send_message(RequestType::GetTextFreetype2Properties { source })
            .await
    }

    /// Set the current properties of a Text Freetype 2 source.
    pub async fn set_text_freetype2_properties(
        &self,
        properties: TextFreetype2Properties<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetTextFreetype2Properties(properties))
            .await
    }

    /// Get configured special sources like Desktop Audio and Mic/Aux sources.
    pub async fn get_special_sources(&self) -> Result<responses::SpecialSources> {
        self.client
            .send_message(RequestType::GetSpecialSources)
            .await
    }

    /// List filters applied to a source
    ///
    /// - `source_name`: Source name.
    pub async fn get_source_filters(
        &self,
        source_name: &str,
    ) -> Result<Vec<responses::SourceFilter>> {
        self.client
            .send_message::<responses::SourceFilters>(RequestType::GetSourceFilters { source_name })
            .await
            .map(|sf| sf.filters)
    }

    /// List filters applied to a source.
    ///
    /// - `source_name`: Source name.
    /// - `filter_name`: Source filter name.
    pub async fn get_source_filter_info<T>(
        &self,
        source_name: &str,
        filter_name: &str,
    ) -> Result<responses::SourceFilterInfo<T>>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message(RequestType::GetSourceFilterInfo {
                source_name,
                filter_name,
            })
            .await
    }

    /// Add a new filter to a source. Available source types along with their settings properties
    /// are available from [`get_sources_types_list`](Self::get_sources_types_list).
    pub async fn add_filter_to_source(&self, add_filter: AddFilter<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::AddFilterToSource(add_filter))
            .await
    }

    /// Remove a filter from a source.
    ///
    /// - `source_name`: Name of the source from which the specified filter is removed.
    /// - `filter_name`: Name of the filter to remove.
    pub async fn remove_filter_from_source(
        &self,
        source_name: &str,
        filter_name: &str,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveFilterFromSource {
                source_name,
                filter_name,
            })
            .await
    }

    /// Move a filter in the chain (absolute index positioning).
    pub async fn reorder_source_filter(&self, reorder_filter: ReorderFilter<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::ReorderSourceFilter(reorder_filter))
            .await
    }

    /// Move a filter in the chain (relative positioning).
    pub async fn move_source_filter(&self, move_filter: MoveFilter<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::MoveSourceFilter(move_filter))
            .await
    }

    /// Update settings of a filter.
    pub async fn set_source_filter_settings(
        &self,
        settings: SourceFilterSettings<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterSettings(settings))
            .await
    }

    /// Change the visibility/enabled state of a filter.
    pub async fn set_source_filter_visibility(
        &self,
        visibility: SourceFilterVisibility<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterVisibility(visibility))
            .await
    }

    /// Get the audio monitoring type of the specified source.
    ///
    /// - `source_name`: Source name.
    pub async fn get_audio_monitor_type(&self, source_name: &str) -> Result<MonitorType> {
        self.client
            .send_message::<responses::AudioMonitorType>(RequestType::GetAudioMonitorType {
                source_name,
            })
            .await
            .map(|amt| amt.monitor_type)
    }

    /// Set the audio monitoring type of the specified source.
    ///
    /// - `source_name`: Source name.
    /// - `monitor_type`: The monitor type to use. Options: `none`, `monitorOnly`,
    ///   `monitorAndOutput`.
    pub async fn set_audio_monitor_type(
        &self,
        source_name: &str,
        monitor_type: MonitorType,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetAudioMonitorType {
                source_name,
                monitor_type,
            })
            .await
    }

    /// Get the default settings for a given source type.
    ///
    /// - `source_kind`: Source kind. Also called "source id" in libobs terminology.
    pub async fn get_source_default_settings(
        &self,
        source_kind: &str,
    ) -> Result<responses::SourceDefaultSettings> {
        self.client
            .send_message(RequestType::GetSourceDefaultSettings { source_kind })
            .await
    }

    /// At least [`embed_picture_format`](SourceScreenshot::embed_picture_format) or
    /// [`save_to_file_path`](SourceScreenshot::save_to_file_path) must be specified.
    ///
    /// Clients can specify [`width`](SourceScreenshot::width) and
    /// [`height`](SourceScreenshot::height) parameters to receive scaled pictures. Aspect ratio is
    /// preserved if only one of these two parameters is specified.
    pub async fn take_source_screenshot(
        &self,
        source_screenshot: SourceScreenshot<'_>,
    ) -> Result<responses::SourceScreenshot> {
        self.client
            .send_message(RequestType::TakeSourceScreenshot(source_screenshot))
            .await
    }

    /// Refreshes the specified browser source.
    ///
    /// - `source_name`: Source name.
    pub async fn refresh_browser_source(&self, source_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RefreshBrowserSource { source_name })
            .await
    }
}
