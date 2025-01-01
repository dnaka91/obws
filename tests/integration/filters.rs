use anyhow::Result;
use obws::requests::filters::{Create, SetEnabled, SetIndex, SetName, SetSettings};
use serde_json::json;
use test_log::test;

use crate::common::{
    self, FILTER_COLOR, TEST_FILTER, TEST_FILTER_2, TEST_FILTER_RENAME, TEST_TEXT,
};

#[test(tokio::test)]
async fn filters() -> Result<()> {
    let (client, server) = common::new_client().await?;
    let client = client.filters();

    server.expect(
        "GetSourceFilterKindList",
        json!(null),
        json!({"sourceFilterKinds": []}),
    );

    client.list_kinds().await?;

    server.expect(
        "GetSourceFilterList",
        json!({"sourceName": "OBWS-TEST-Text"}),
        json!({"filters": []}),
    );

    client.list(TEST_TEXT.as_source()).await?;

    server.expect(
        "GetSourceFilterDefaultSettings",
        json!({"filterKind": "color_filter"}),
        json!({"defaultFilterSettings": {}}),
    );

    client
        .default_settings::<serde_json::Value>(FILTER_COLOR)
        .await?;

    server.expect(
        "CreateSourceFilter",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter2",
            "filterKind": "color_filter",
            "filterSettings": {},
        }),
        json!(null),
    );

    client
        .create(Create {
            source: TEST_TEXT.as_source(),
            filter: TEST_FILTER_2,
            kind: FILTER_COLOR,
            settings: Some(serde_json::Map::new()),
        })
        .await?;

    server.expect(
        "RemoveSourceFilter",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter2",
        }),
        json!(null),
    );

    client.remove(TEST_TEXT.as_source(), TEST_FILTER_2).await?;

    server.expect(
        "SetSourceFilterName",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter",
            "newFilterName": "OBWS-TEST-Filter-Renamed",
        }),
        json!(null),
    );

    client
        .set_name(SetName {
            source: TEST_TEXT.as_source(),
            filter: TEST_FILTER,
            new_name: TEST_FILTER_RENAME,
        })
        .await?;

    server.expect(
        "GetSourceFilter",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter",
        }),
        json!({
            "filterEnabled": true,
            "filterIndex": 1,
            "filterKind": "color_filter",
            "filterSettings": {},
        }),
    );

    client.get(TEST_TEXT.as_source(), TEST_FILTER).await?;

    server.expect(
        "SetSourceFilterIndex",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter",
            "filterIndex": 0,
        }),
        json!(null),
    );

    client
        .set_index(SetIndex {
            source: TEST_TEXT.as_source(),
            filter: TEST_FILTER,
            index: 0,
        })
        .await?;

    server.expect(
        "SetSourceFilterSettings",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter",
            "filterSettings": {},
            "overlay": true,
        }),
        json!(null),
    );

    client
        .set_settings(SetSettings {
            source: TEST_TEXT.as_source(),
            filter: TEST_FILTER,
            settings: serde_json::Map::new(),
            overlay: Some(true),
        })
        .await?;

    server.expect(
        "SetSourceFilterEnabled",
        json!({
            "sourceName": "OBWS-TEST-Text",
            "filterName": "OBWS-TEST-Filter",
            "filterEnabled": false,
        }),
        json!(null),
    );

    client
        .set_enabled(SetEnabled {
            source: TEST_TEXT.as_source(),
            filter: TEST_FILTER,
            enabled: false,
        })
        .await?;

    server.stop().await
}
