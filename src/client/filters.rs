use super::Client;
use crate::{requests::RequestType, responses, Result};

/// API functions related to filters.
pub struct Filters<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Filters<'a> {
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
}
