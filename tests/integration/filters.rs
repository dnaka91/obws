use anyhow::Result;
use obws::requests::{
    CreateSourceFilter, SetSourceFilterEnabled, SetSourceFilterIndex, SetSourceFilterName,
    SetSourceFilterSettings,
};

use crate::common::{
    self, FILTER_COLOR, TEST_FILTER, TEST_FILTER_2, TEST_FILTER_RENAME, TEST_TEXT,
};

#[tokio::test]
async fn filters() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.filters();

    client.get_source_filter_list(TEST_TEXT).await?;

    client
        .get_source_filter_default_settings::<serde_json::Value>(FILTER_COLOR)
        .await?;
    client
        .create_source_filter(CreateSourceFilter {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER_2,
            filter_kind: FILTER_COLOR,
            filter_settings: Some(serde_json::Map::new()),
        })
        .await?;
    client
        .remove_source_filter(TEST_TEXT, TEST_FILTER_2)
        .await?;

    client
        .set_source_filter_name(SetSourceFilterName {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER,
            new_filter_name: TEST_FILTER_RENAME,
        })
        .await?;
    client
        .set_source_filter_name(SetSourceFilterName {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER_RENAME,
            new_filter_name: TEST_FILTER,
        })
        .await?;

    client.get_source_filter(TEST_TEXT, TEST_FILTER).await?;

    client
        .set_source_filter_index(SetSourceFilterIndex {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER,
            filter_index: 0,
        })
        .await?;
    client
        .set_source_filter_settings(SetSourceFilterSettings {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER,
            filter_settings: serde_json::Map::new(),
            overlay: Some(true),
        })
        .await?;
    client
        .set_source_filter_enabled(SetSourceFilterEnabled {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER,
            filter_enabled: false,
        })
        .await?;
    client
        .set_source_filter_enabled(SetSourceFilterEnabled {
            source_name: TEST_TEXT,
            filter_name: TEST_FILTER,
            filter_enabled: true,
        })
        .await?;

    Ok(())
}
