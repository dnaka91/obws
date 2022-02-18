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
    ///
    /// - `source_name`: Name of the source.
    pub async fn get_source_filter_list(
        &self,
        source_name: &str,
    ) -> Result<Vec<responses::SourceFilter>> {
        self.client
            .send_message::<responses::Filters>(RequestType::GetSourceFilterList { source_name })
            .await
            .map(|f| f.filters)
    }

    /// Gets the default settings for a filter kind.
    ///
    /// - `filter_kind`: Filter kind to get the default settings for.
    pub async fn get_source_filter_default_settings<T>(&self, filter_kind: &str) -> Result<T>
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
    pub async fn create_source_filter<T>(&self, filter: CreateSourceFilter<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::CreateSourceFilter(
                CreateSourceFilterInternal {
                    source_name: filter.source_name,
                    filter_name: filter.filter_name,
                    filter_kind: filter.filter_kind,
                    filter_settings: filter
                        .filter_settings
                        .map(|settings| serde_json::to_value(&settings))
                        .transpose()
                        .map_err(Error::SerializeCustomData)?,
                },
            ))
            .await
    }

    /// Removes a filter from a source.
    ///
    /// - `source_name`: Name of the source the filter is on.
    /// - `filter_name`: Name of the filter to remove.
    pub async fn remove_source_filter(&self, source_name: &str, filter_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveSourceFilter {
                source_name,
                filter_name,
            })
            .await
    }

    /// Sets the name of a source filter (rename).
    pub async fn set_source_filter_name(&self, name: SetSourceFilterName<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterName(name))
            .await
    }

    /// Gets the info for a specific source filter.
    ///
    /// - `source_name`: Name of the source.
    /// - `filter_name`: Name of the filter.
    pub async fn get_source_filter(
        &self,
        source_name: &str,
        filter_name: &str,
    ) -> Result<responses::SourceFilter> {
        self.client
            .send_message(RequestType::GetSourceFilter {
                source_name,
                filter_name,
            })
            .await
    }

    /// Sets the index position of a filter on a source.
    pub async fn set_source_filter_index(&self, index: SetSourceFilterIndex<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterIndex(index))
            .await
    }

    /// Sets the settings of a source filter.
    pub async fn set_source_filter_settings<T>(
        &self,
        settings: SetSourceFilterSettings<'_, T>,
    ) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetSourceFilterSettings(
                SetSourceFilterSettingsInternal {
                    source_name: settings.source_name,
                    filter_name: settings.filter_name,
                    filter_settings: serde_json::to_value(&settings.filter_settings)
                        .map_err(Error::SerializeCustomData)?,
                    overlay: settings.overlay,
                },
            ))
            .await
    }

    /// Sets the enable state of a source filter.
    pub async fn set_source_filter_enabled(
        &self,
        enabled: SetSourceFilterEnabled<'_>,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetSourceFilterEnabled(enabled))
            .await
    }
}
