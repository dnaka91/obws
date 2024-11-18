//! Requests related to filters.

use serde::Serialize;
use serde_with::skip_serializing_none;

use super::sources::SourceId;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetSourceFilterKindList")]
    KindList,
    #[serde(rename = "GetSourceFilterList")]
    List {
        /// Identifier of the source.
        #[serde(flatten)]
        source: SourceId<'a>,
    },
    #[serde(rename = "GetSourceFilterDefaultSettings")]
    DefaultSettings {
        /// Filter kind to get the default settings for.
        #[serde(rename = "filterKind")]
        kind: &'a str,
    },
    #[serde(rename = "CreateSourceFilter")]
    Create(CreateInternal<'a>),
    #[serde(rename = "RemoveSourceFilter")]
    Remove {
        /// Identifier of the source the filter is on.
        #[serde(flatten)]
        source: SourceId<'a>,
        /// Name of the filter to remove.
        #[serde(rename = "filterName")]
        filter: &'a str,
    },
    #[serde(rename = "SetSourceFilterName")]
    SetName(SetName<'a>),
    #[serde(rename = "GetSourceFilter")]
    Get {
        /// Identifier of the source.
        #[serde(flatten)]
        source: SourceId<'a>,
        /// Name of the filter.
        #[serde(rename = "filterName")]
        filter: &'a str,
    },
    #[serde(rename = "SetSourceFilterIndex")]
    SetIndex(SetIndex<'a>),
    #[serde(rename = "SetSourceFilterSettings")]
    SetSettings(SetSettingsInternal<'a>),
    #[serde(rename = "SetSourceFilterEnabled")]
    SetEnabled(SetEnabled<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Filters(value)
    }
}

/// Request information for [`crate::client::Filters::create`].
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct Create<'a, T> {
    /// Identifier of the source to add the filter to.
    pub source: SourceId<'a>,
    /// Name of the new filter to be created.
    pub filter: &'a str,
    /// The kind of filter to be created.
    pub kind: &'a str,
    /// Settings object to initialize the filter with.
    pub settings: Option<T>,
}

/// Request information for [`crate::client::Filters::create_source_filter`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
pub(crate) struct CreateInternal<'a> {
    /// Identifier of the source to add the filter to.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Name of the new filter to be created.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// The kind of filter to be created.
    #[serde(rename = "filterKind")]
    pub kind: &'a str,
    /// Settings object to initialize the filter with.
    #[serde(rename = "filterSettings")]
    pub settings: Option<serde_json::Value>,
}

/// Request information for [`crate::client::Filters::set_name`].
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetName<'a> {
    /// Identifier of the source the filter is on.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Current name of the filter.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// New name for the filter.
    #[serde(rename = "newFilterName")]
    pub new_name: &'a str,
}

/// Request information for [`crate::client::Filters::set_index`].
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetIndex<'a> {
    /// Identifier of the source the filter is on.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Name of the filter.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// New index position of the filter.
    #[serde(rename = "filterIndex")]
    pub index: u32,
}

/// Request information for [`crate::client::Filters::set_settings`].
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetSettings<'a, T> {
    /// Identifier of the source the filter is on.
    pub source: SourceId<'a>,
    /// Name of the filter to set the settings of.
    pub filter: &'a str,
    /// Object of settings to apply.
    pub settings: T,
    /// Whether to overlay over the current settings or replace them.
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Filters::set_settings`].
#[derive(Default, Serialize)]
pub(crate) struct SetSettingsInternal<'a> {
    /// Identifier of the source the filter is on.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Name of the filter to set the settings of.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// Object of settings to apply.
    #[serde(rename = "filterSettings")]
    pub settings: serde_json::Value,
    /// Whether to overlay over the current settings or replace them.
    #[serde(rename = "overlay")]
    pub overlay: Option<bool>,
}

/// Request information for [`crate::client::Filters::set_enabled`].
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetEnabled<'a> {
    /// Identifier of the source the filter is on.
    #[serde(flatten)]
    pub source: SourceId<'a>,
    /// Name of the filter.
    #[serde(rename = "filterName")]
    pub filter: &'a str,
    /// New enable state of the filter.
    #[serde(rename = "filterEnabled")]
    pub enabled: bool,
}
