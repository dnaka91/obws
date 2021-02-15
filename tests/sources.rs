#![cfg(feature = "test-integration")]

use anyhow::{Context, Result};
use chrono::Duration;
use obws::{
    common::MonitorType,
    requests::{
        AddFilter, MoveFilter, ReorderFilter, SourceFilterSettings, SourceFilterVisibility,
        SourceScreenshot, SourceSettings, Volume,
    },
};
use serde_json::json;

use common::{SOURCE_KIND_VLC, TEST_BROWSER, TEST_MEDIA, TEXT_SOURCE};

mod common;

#[tokio::test]
async fn main() -> Result<()> {
    let client = common::new_client().await?;
    let client = client.sources();

    client.get_media_sources_list().await?;
    client.get_sources_list().await?;
    client.get_sources_types_list().await?;

    client.get_audio_active(TEST_MEDIA).await?;
    client.get_source_default_settings(SOURCE_KIND_VLC).await?;

    client.refresh_browser_source(TEST_BROWSER).await?;

    // Volume

    let original = client.get_volume(TEXT_SOURCE, None).await?.volume;
    client.get_volume(TEXT_SOURCE, Some(true)).await?;

    client
        .set_volume(Volume {
            source: TEXT_SOURCE,
            volume: 0.5,
            use_decibel: None,
        })
        .await?;

    client
        .set_volume(Volume {
            source: TEXT_SOURCE,
            volume: original,
            use_decibel: None,
        })
        .await?;

    // Mute

    let original = client.get_mute(TEXT_SOURCE).await?.muted;
    client.toggle_mute(TEXT_SOURCE).await?;
    client.set_mute(TEXT_SOURCE, original).await?;

    // Source name

    let new_name = format!("{}-Test", TEXT_SOURCE);
    client.set_source_name(TEXT_SOURCE, &new_name).await?;
    client.set_source_name(&new_name, TEXT_SOURCE).await?;

    // Sync offset

    let original = client.get_sync_offset(TEXT_SOURCE).await?;
    client
        .set_sync_offset(TEXT_SOURCE, Duration::milliseconds(200))
        .await?;
    client.set_sync_offset(TEXT_SOURCE, original.offset).await?;

    // Source settings

    let settings = client
        .get_source_settings::<serde_json::Value>(TEXT_SOURCE, None)
        .await?;
    client
        .set_source_settings::<serde_json::Value>(SourceSettings {
            source_name: &settings.source_name,
            source_type: Some(&settings.source_type),
            source_settings: &settings.source_settings,
        })
        .await?;

    // TODO: GDI+ only on windows?

    // Freetype2 properties

    let props = client.get_text_freetype2_properties(TEXT_SOURCE).await?;
    client
        .set_text_freetype2_properties((&props).into())
        .await?;

    // Special sources

    client.get_special_sources().await?;

    // Filters
    const FILTER1: &str = "Scroll-Test1";
    const FILTER2: &str = "Scroll-Test2";

    client.get_source_filters(TEXT_SOURCE).await?;
    client
        .add_filter_to_source(AddFilter {
            source_name: TEXT_SOURCE,
            filter_name: FILTER1,
            filter_type: "scroll_filter",
            filter_settings: &json! {{
                "limit_cx": false,
                "limit_cy": false,
                "speed_x": 50.0
            }},
        })
        .await?;
    client
        .get_source_filter_info::<serde_json::Value>(TEXT_SOURCE, FILTER1)
        .await?;
    client
        .add_filter_to_source(AddFilter {
            source_name: TEXT_SOURCE,
            filter_name: FILTER2,
            filter_type: "scroll_filter",
            filter_settings: &json! {{
                "limit_cx": false,
                "limit_cy": false,
                "speed_x": 20.0
            }},
        })
        .await?;

    client
        .reorder_source_filter(ReorderFilter {
            source_name: TEXT_SOURCE,
            filter_name: FILTER1,
            new_index: 1,
        })
        .await?;
    client
        .move_source_filter(MoveFilter {
            source_name: TEXT_SOURCE,
            filter_name: FILTER1,
            movement_type: obws::requests::MovementType::Up,
        })
        .await?;
    client
        .set_source_filter_settings(SourceFilterSettings {
            source_name: TEXT_SOURCE,
            filter_name: FILTER1,
            filter_settings: &json! {{
                "limit_cx": false,
                "limit_cy": false,
                "speed_x": -100.0
            }},
        })
        .await?;
    client
        .set_source_filter_visibility(SourceFilterVisibility {
            source_name: TEXT_SOURCE,
            filter_name: FILTER1,
            filter_enabled: false,
        })
        .await?;

    client
        .remove_filter_from_source(TEXT_SOURCE, FILTER1)
        .await?;
    client
        .remove_filter_from_source(TEXT_SOURCE, FILTER2)
        .await?;

    // Audio monitor type

    let source = client
        .get_special_sources()
        .await?
        .desktop_1
        .context("desktop audio device required for tests")?;

    let original = client.get_audio_monitor_type(&source).await?;
    client
        .set_audio_monitor_type(&source, MonitorType::MonitorAndOutput)
        .await?;
    client.set_audio_monitor_type(&source, original).await?;

    // Take source screenshot

    client
        .take_source_screenshot(SourceScreenshot {
            source_name: Some(TEXT_SOURCE),
            embed_picture_format: Some("png"),
            width: Some(10),
            ..Default::default()
        })
        .await?;

    Ok(())
}
