//! Responses related to filters.

use serde::{Deserialize, Serialize};

/// Response value for [`crate::client::Filters::get_source_filter_kind_list`].
#[derive(Debug, Deserialize)]
pub(crate) struct FilterKinds {
    /// Array of source filter kinds.
    #[serde(rename = "sourceFilterKinds")]
    pub kinds: Vec<String>,
}

/// Response value for [`crate::client::Filters::get_source_filter_list`].
#[derive(Debug, Deserialize)]
pub(crate) struct Filters {
    /// Array of filters.
    #[serde(rename = "filters")]
    pub filters: Vec<SourceFilter>,
}

/// Response value for [`crate::client::Filters::list`] and [`crate::client::Filters::get`].
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceFilter {
    /// Whether the filter is enabled.
    #[serde(rename = "filterEnabled")]
    pub enabled: bool,
    /// Index of the filter in the list, beginning at 0.
    #[serde(rename = "filterIndex")]
    pub index: u32,
    /// The kind of filter.
    #[serde(rename = "filterKind")]
    pub kind: String,
    /// name of the filter.
    #[serde(rename = "filterName", default)]
    pub name: String,
    /// Settings object associated with the filter.
    #[serde(rename = "filterSettings")]
    pub settings: serde_json::Value,
}

/// Response value for
/// [`crate::client::Filters::get_source_filter_default_settings`].
#[derive(Debug, Deserialize)]
pub(crate) struct DefaultFilterSettings<T> {
    /// Object of default settings for the filter kind.
    #[serde(rename = "defaultFilterSettings")]
    pub default_filter_settings: T,
}
