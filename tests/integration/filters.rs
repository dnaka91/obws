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

    client.list(TEST_TEXT).await?;

    client
        .default_settings::<serde_json::Value>(FILTER_COLOR)
        .await?;
    client
        .create(CreateSourceFilter {
            source: TEST_TEXT,
            filter: TEST_FILTER_2,
            kind: FILTER_COLOR,
            settings: Some(serde_json::Map::new()),
        })
        .await?;
    client.remove(TEST_TEXT, TEST_FILTER_2).await?;

    client
        .set_name(SetSourceFilterName {
            source: TEST_TEXT,
            filter: TEST_FILTER,
            new_name: TEST_FILTER_RENAME,
        })
        .await?;
    client
        .set_name(SetSourceFilterName {
            source: TEST_TEXT,
            filter: TEST_FILTER_RENAME,
            new_name: TEST_FILTER,
        })
        .await?;

    client.get(TEST_TEXT, TEST_FILTER).await?;

    client
        .set_index(SetSourceFilterIndex {
            source: TEST_TEXT,
            filter: TEST_FILTER,
            index: 0,
        })
        .await?;
    client
        .set_settings(SetSourceFilterSettings {
            source: TEST_TEXT,
            filter: TEST_FILTER,
            settings: serde_json::Map::new(),
            overlay: Some(true),
        })
        .await?;
    client
        .set_enabled(SetSourceFilterEnabled {
            source: TEST_TEXT,
            filter: TEST_FILTER,
            enabled: false,
        })
        .await?;
    client
        .set_enabled(SetSourceFilterEnabled {
            source: TEST_TEXT,
            filter: TEST_FILTER,
            enabled: true,
        })
        .await?;

    Ok(())
}
