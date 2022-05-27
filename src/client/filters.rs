use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{
    requests::{
        CreateSourceFilter, CreateSourceFilterInternal, RequestType, SetSourceFilterEnabled,
        SetSourceFilterIndex, SetSourceFilterName, SetSourceFilterSettings,
        SetSourceFilterSettingsInternal,
    },
    responses, Error, Result,
};

/// API functions related to filters.
pub struct Filters<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Filters<'a> {
    /// Gets an array of all of a source's filters.
    pub async fn list(&self, source_name: &str) -> Result<Vec<responses::SourceFilter>> {
        self.client
            .send_message::<responses::Filters>(RequestType::GetSourceFilterList { source_name })
            .await
            .map(|f| f.filters)
    }

    /// Gets the default settings for a filter kind.
    pub async fn default_settings<T>(&self, filter_kind: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message::<responses::DefaultFilterSettings<T>>(
                RequestType::GetSourceFilterDefaultSettings { filter_kind },
            )
            .await
            .map(|dfs| dfs.default_filter_settings)
    }

    /// Creates a new filter, adding it to the specified source.
    pub async fn create<T>(&self, filter: CreateSourceFilter<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::CreateSourceFilter(
                CreateSourceFilterInternal {
                    source: filter.source,
                    filter: filter.filter,
                    kind: filter.kind,
                    settings: filter
                        .settings
                        .map(|settings| serde_json::to_value(&settings))
                        .transpose()
                        .map_err(Error::SerializeCustomData)?,
                },
            ))
            .await
    }

    /// Removes a filter from a source.
    pub async fn remove(&self, source: &str, filter: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveSourceFilter { source, filter })
            .await
    }

    /// Sets the name of a source filter (rename).
    pub async fn set_name(&self, name: SetSourceFilterName<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterName(name))
            .await
    }

    /// Gets the info for a specific source filter.
    pub async fn get(&self, source: &str, filter: &str) -> Result<responses::SourceFilter> {
        self.client
            .send_message(RequestType::GetSourceFilter { source, filter })
            .await
    }

    /// Sets the index position of a filter on a source.
    pub async fn set_index(&self, index: SetSourceFilterIndex<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterIndex(index))
            .await
    }

    /// Sets the settings of a source filter.
    pub async fn set_settings<T>(&self, settings: SetSourceFilterSettings<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetSourceFilterSettings(
                SetSourceFilterSettingsInternal {
                    source: settings.source,
                    filter: settings.filter,
                    settings: serde_json::to_value(&settings.settings)
                        .map_err(Error::SerializeCustomData)?,
                    overlay: settings.overlay,
                },
            ))
            .await
    }

    /// Sets the enable state of a source filter.
    pub async fn set_enabled(&self, enabled: SetSourceFilterEnabled<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterEnabled(enabled))
            .await
    }
}
