//! All responses that can be received from the API.

use std::collections::HashSet;
use std::path::PathBuf;

use chrono::Duration;
use serde::Deserialize;

pub use rgb::RGBA8;
pub use semver::Version as SemVerVersion;

use crate::common::{
    Align, Bounds, Crop, FontFlags, MonitorType, Position, Scale, SceneItem, SceneItemTransform,
    StreamType, Valign,
};

mod de;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Response<T> {
    pub message_id: String,
    pub status: Status,
    pub error: Option<String>,
    #[serde(flatten)]
    pub details: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Status {
    Ok,
    Error,
}

/// Response value for [`get_version`](crate::client::General::get_version).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Version {
    /// OBSRemote compatible API version. Fixed to 1.1 for retrocompatibility.
    pub version: f64,
    /// obs-websocket plugin version.
    pub obs_websocket_version: SemVerVersion,
    /// OBS Studio program version.
    pub obs_studio_version: SemVerVersion,
    /// List of available request types, formatted as a comma-separated list string (e.g. :
    /// "Method1,Method2,Method3").
    #[serde(deserialize_with = "de::string_comma_list")]
    pub available_requests: HashSet<String>,
    /// List of supported formats for features that use image export (like the
    /// [`take_source_screenshot`](crate::client::Sources::take_source_screenshot) request)
    /// formatted as a comma-separated list string.
    #[serde(deserialize_with = "de::string_comma_list")]
    pub supported_image_export_formats: HashSet<String>,
}

/// Response value for [`get_auth_required`](crate::client::General::get_auth_required).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRequired {
    /// Indicates whether authentication is required.
    pub auth_required: bool,
    /// A random string that will be used to generate the auth response.
    pub challenge: Option<String>,
    /// Applied to the password when generating the auth response.
    pub salt: Option<String>,
}

/// Response value for [`get_filename_formatting`](crate::client::General::get_filename_formatting).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct FilenameFormatting {
    /// Current filename formatting string.
    pub filename_formatting: String,
}

/// Response value for [`get_stats`](crate::client::General::get_stats).
#[derive(Debug, Deserialize)]
pub(crate) struct Stats {
    /// See [`ObsStats`].
    pub stats: ObsStats,
}

/// Response value for [`get_video_info`](crate::client::General::get_video_info).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    /// Base (canvas) width.
    pub base_width: u64,
    /// Base (canvas) height.
    pub base_height: u64,
    /// Output width.
    pub output_width: u64,
    /// Output height.
    pub output_height: u64,
    /// Scaling method used if output size differs from base size.
    pub scale_type: ScaleType,
    /// Frames rendered per second.
    pub fps: f64,
    /// Video color format.
    pub video_format: VideoFormat,
    /// Color space for YUV.
    pub color_space: ColorSpace,
    /// Color range (full or partial).
    pub color_range: ColorRange,
}

/// Possible scaling types for the output.
///
/// Response value for [`get_video_info`](crate::client::General::get_video_info) as part of
/// [`VideoInfo`].
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ScaleType {
    /// Fastest, but blurry scaling.
    #[serde(rename = "VIDEO_SCALE_BILINEAR")]
    Bilinear,
    /// Weighted sum, 4/6/9 samples.
    #[serde(rename = "VIDEO_SCALE_DEFAULT")]
    Area,
    /// Sharpened scaling, 16 samples.
    #[serde(rename = "VIDEO_SCALE_FAST_BILINEAR")]
    Bicubic,
    /// Sharpened scaling, 36 samples.
    #[serde(rename = "VIDEO_SCALE_BICUBIC")]
    Lanczos,
}

/// Supported formats for video output.
///
/// Response value for [`get_video_info`](crate::client::General::get_video_info) as part of
/// [`VideoInfo`].
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum VideoFormat {
    /// NV12 format.
    #[serde(rename = "VIDEO_FORMAT_NV12")]
    Nv12,
    /// I420 format.
    #[serde(rename = "VIDEO_FORMAT_I420")]
    I420,
    /// I444 format.
    #[serde(rename = "VIDEO_FORMAT_I444")]
    I444,
    /// RGB format.
    #[serde(rename = "VIDEO_FORMAT_RGBA")]
    RGB,
}

/// Supported color spaces for video output.
///
/// Response value for [`get_video_info`](crate::client::General::get_video_info) as part of
/// [`VideoInfo`].
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ColorSpace {
    /// 709 color space.
    #[serde(rename = "VIDEO_CS_709")]
    Cs709,
    /// 601 color space.
    #[serde(rename = "VIDEO_CS_601")]
    Cs601,
    /// sRGB color space.
    #[serde(rename = "VIDEO_CS_DEFAULT")]
    CsSRgb,
}

/// Supported color ranges for video output.
///
/// Response value for [`get_video_info`](crate::client::General::get_video_info) as part of
/// [`VideoInfo`].
#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ColorRange {
    /// Partial color range.
    #[serde(rename = "VIDEO_RANGE_PARTIAL")]
    Partial,
    /// Full range.
    #[serde(rename = "VIDEO_RANGE_FULL")]
    Full,
}

/// Response value for [`get_media_duration`](crate::client::MediaControl::get_media_duration).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaDuration {
    /// The total length of media in milliseconds.
    #[serde(deserialize_with = "crate::de::duration_millis")]
    pub media_duration: Duration,
}

/// Response value for [`get_media_time`](crate::client::MediaControl::get_media_time).
#[derive(Debug, Deserialize)]
pub(crate) struct MediaTime {
    /// The time in milliseconds since the start of the media.
    #[serde(deserialize_with = "crate::de::duration_millis")]
    pub timestamp: Duration,
}

/// Response value for [`get_media_state`](crate::client::MediaControl::get_media_state).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetMediaState {
    /// The media state of the provided source.
    pub media_state: MediaState,
}

/// Response value for [`get_media_state`](crate::client::MediaControl::get_media_state).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaState {
    /// No state.
    None,
    /// Media is playing.
    Playing,
    /// Opening file for replay.
    Opening,
    /// Buffering data for replay.
    Buffering,
    /// Media is paused.
    Paused,
    /// Media stopped.
    Stopped,
    /// All media in the playlist played.
    Ended,
    /// Error occured while trying to play the media.
    Error,
    /// Unknown state.
    #[serde(other)]
    Unknown,
}

/// Response value for [`get_media_sources_list`](crate::client::Sources::get_media_sources_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MediaSourcesList {
    /// Array of sources.
    pub media_sources: Vec<MediaSource>,
}

/// Response value for [`get_media_sources_list`](crate::client::Sources::get_media_sources_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaSource {
    /// Unique source name.
    pub source_name: String,
    /// Unique source internal type (a.k.a `ffmpeg_source` or `vlc_source`).
    pub source_kind: String,
    /// The current state of media for that source.
    pub media_state: MediaState,
}

/// Response value for [`create_source`](crate::client::Sources::create_source).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SourceItemId {
    /// ID of the SceneItem in the scene.
    pub item_id: i64,
}

/// Response value for [`get_sources_list`](crate::client::Sources::get_sources_list).
#[derive(Debug, Deserialize)]
pub(crate) struct SourcesList {
    /// Array of sources.
    pub sources: Vec<SourceListItem>,
}

/// Response value for [`get_sources_types_list`](crate::client::Sources::get_sources_types_list).
#[derive(Debug, Deserialize)]
pub(crate) struct SourceTypesList {
    /// Array of source types.
    pub types: Vec<SourceTypeItem>,
}

/// Response value for [`get_volume`](crate::client::Sources::get_volume).
#[derive(Debug, Deserialize)]
pub struct Volume {
    /// Source name.
    pub name: String,
    /// Volume of the source. Between `0.0` and `20.0` if using mul, under `26.0` if using dB.
    pub volume: f64,
    /// Indicates whether the source is muted.
    pub muted: bool,
}

/// Response value for [`get_mute`](crate::client::Sources::get_mute).
#[derive(Debug, Deserialize)]
pub struct Mute {
    /// Source name.
    pub name: String,
    /// Mute status of the source.
    pub muted: bool,
}

/// Response value for [`get_audio_active`](crate::client::Sources::get_audio_active).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioActive {
    /// Audio active status of the source.
    pub audio_active: bool,
}

/// Response value for [`get_sync_offset`](crate::client::Sources::get_sync_offset).
#[derive(Debug, Deserialize)]
pub struct SyncOffset {
    /// Source name.
    pub name: String,
    /// The audio sync offset (in nanoseconds).
    #[serde(deserialize_with = "crate::de::duration_nanos")]
    pub offset: Duration,
}

/// Response value for [`get_source_settings`](crate::client::Sources::get_source_settings) and
/// [`set_source_settings`](crate::client::Sources::set_source_settings).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSettings<T> {
    /// Source name.
    pub source_name: String,
    /// Type of the specified source.
    pub source_type: String,
    /// Source settings (varies between source types, may require some probing around).
    pub source_settings: T,
}

/// Response value for
/// [`get_text_gdi_plus_properties`](crate::client::Sources::get_text_gdi_plus_properties).
#[derive(Debug, Deserialize)]
pub struct TextGdiPlusProperties {
    /// Source name.
    pub source: String,
    /// Text Alignment ("left", "center", "right").
    pub align: Align,
    /// Background color.
    pub bk_color: u32,
    /// Background opacity (0-100).
    pub bk_opacity: u8,
    /// Chat log.
    pub chatlog: bool,
    /// Chat log lines.
    pub chatlog_lines: u64,
    /// Text color.
    pub color: u32,
    /// Extents wrap.
    pub extents: bool,
    /// Extents cx.
    pub extents_cx: i64,
    /// Extents cy.
    pub extents_cy: i64,
    /// File path name.
    pub file: PathBuf,
    /// Read text from the specified file.
    pub read_from_file: bool,
    /// Holds data for the font. Ex:
    /// `"font": { "face": "Arial", "flags": 0, "size": 150, "style": "" }`.
    pub font: Font,
    /// Gradient enabled.
    pub gradient: bool,
    /// Gradient color.
    pub gradient_color: u32,
    /// Gradient direction.
    pub gradient_dir: f32,
    /// Gradient opacity (0-100).
    pub gradient_opacity: u8,
    /// Outline.
    pub outline: bool,
    /// Outline color.
    pub outline_color: u32,
    /// Outline size.
    pub outline_size: u64,
    /// Outline opacity (0-100).
    pub outline_opacity: u8,
    /// Text content to be displayed.
    pub text: String,
    /// Text vertical alignment ("top", "center", "bottom").
    pub valign: Valign,
    /// Vertical text enabled.
    pub vertical: bool,
}

/// Response value for
/// [`get_text_freetype2_properties`](crate::client::Sources::get_text_freetype2_properties).
#[derive(Debug, Deserialize)]
pub struct TextFreetype2Properties {
    /// Source name.
    pub source: String,
    /// Gradient top color.
    #[serde(default, deserialize_with = "de::rgba8_inverse_opt")]
    pub color1: Option<RGBA8>,
    /// Gradient bottom color.
    #[serde(default, deserialize_with = "de::rgba8_inverse_opt")]
    pub color2: Option<RGBA8>,
    /// Custom width (0 to disable).
    pub custom_width: Option<u32>,
    /// Drop shadow.
    #[serde(default)]
    pub drop_shadow: bool,
    /// Holds data for the font. Ex:
    /// `"font": { "face": "Arial", "flags": 0, "size": 150, "style": "" }`.
    pub font: Option<Font>,
    /// Read text from the specified file.
    #[serde(default)]
    pub from_file: bool,
    /// Chat log.
    #[serde(default)]
    pub log_mode: bool,
    /// Outline.
    #[serde(default)]
    pub outline: bool,
    /// Text content to be displayed.
    pub text: String,
    /// File path.
    pub text_file: Option<PathBuf>,
    /// Word wrap.
    #[serde(default)]
    pub word_wrap: bool,
}

/// Response value for [`get_special_sources`](crate::client::Sources::get_special_sources).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpecialSources {
    /// Name of the first Desktop Audio capture source.
    pub desktop_1: Option<String>,
    /// Name of the second Desktop Audio capture source.
    pub desktop_2: Option<String>,
    /// Name of the first Mic/Aux input source.
    pub mic_1: Option<String>,
    /// Name of the second Mic/Aux input source.
    pub mic_2: Option<String>,
    /// Name of the third Mic/Aux input source.
    pub mic_3: Option<String>,
}

/// Response value for [`get_source_filters`](crate::client::Sources::get_source_filters).
#[derive(Debug, Deserialize)]
pub(crate) struct SourceFilters {
    /// List of filters for the specified source.
    pub filters: Vec<SourceFilter>,
}

/// Response value for [`get_source_filter_info`](crate::client::Sources::get_source_filter_info).
#[derive(Debug, Deserialize)]
pub struct SourceFilterInfo<T> {
    /// Filter status (enabled or not).
    pub enabled: bool,
    /// Filter type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Filter name.
    pub name: String,
    /// Filter settings.
    pub settings: T,
}

/// Response value for [`get_audio_monitor_type`](crate::client::Sources::get_audio_monitor_type).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioMonitorType {
    /// The monitor type in use. Options: `none`, `monitorOnly`, `monitorAndOutput`.
    pub monitor_type: MonitorType,
}

/// Response value for
/// [`get_source_default_settings`](crate::client::Sources::get_source_default_settings).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceDefaultSettings {
    /// Source kind. Same value as the `source_kind` parameter.
    pub source_kind: String,
    /// Settings object for source.
    pub default_settings: serde_json::Value,
}

/// Response value for [`take_source_screenshot`](crate::client::Sources::take_source_screenshot).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceScreenshot {
    /// Source name.
    pub source_name: String,
    /// Image Data URI (if
    /// [`embed_picture_format`](crate::requests::SourceScreenshot::embed_picture_format) was
    /// specified in the request).
    pub img: Option<String>,
    /// Absolute path to the saved image file (if
    /// [`save_to_file_path`](crate::requests::SourceScreenshot::save_to_file_path) was specified in
    /// the request).
    pub image_file: Option<PathBuf>,
}

/// Response value for [`list_outputs`](crate::client::Outputs::list_outputs).
#[derive(Debug, Deserialize)]
pub(crate) struct Outputs {
    /// Outputs list.
    pub outputs: Vec<Output>,
}

/// Response value for [`get_output_info`](crate::client::Outputs::get_output_info).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputInfo {
    /// Output info.
    pub output_info: Output,
}

/// Response value for [`get_current_profile`](crate::client::Profiles::get_current_profile).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CurrentProfile {
    /// Name of the currently active profile.
    pub profile_name: String,
}

/// Response value for [`list_profiles`](crate::client::Profiles::list_profiles).
#[derive(Debug, Deserialize)]
pub(crate) struct Profiles {
    /// List of available profiles.
    pub profiles: Vec<Profile>,
}

/// Response value for [`get_recording_status`](crate::client::Recording::get_recording_status).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingStatus {
    /// Current recording status.
    pub is_recording: bool,
    /// Whether the recording is paused or not.
    pub is_recording_paused: bool,
    /// Time elapsed since recording started (only present if currently recording).
    #[serde(default, deserialize_with = "crate::de::duration_opt")]
    pub record_timecode: Option<Duration>,
    /// Absolute path to the recording file (only present if currently recording).
    pub recording_filename: Option<PathBuf>,
}

/// Response value for [`get_recording_folder`](crate::client::Recording::get_recording_folder).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct RecordingFolder {
    /// Path of the recording folder.
    pub rec_folder: PathBuf,
}

/// Response value for
/// [`get_replay_buffer_status`](crate::client::ReplayBuffer::get_replay_buffer_status).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReplayBufferStatus {
    /// Current recording status.
    pub is_replay_buffer_active: bool,
}

/// Response value for
/// [`get_current_scene_collection`](crate::client::SceneCollections::get_current_scene_collection).
#[serde(rename_all = "kebab-case")]
#[derive(Debug, Deserialize)]
pub(crate) struct CurrentSceneCollection {
    /// Name of the currently active scene collection.
    pub sc_name: String,
}

/// Response value for
/// [`list_scene_collections`](crate::client::SceneCollections::list_scene_collections).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SceneCollections {
    /// Scene collections list.
    pub scene_collections: Vec<SceneCollection>,
}

/// Response value for
/// [`get_scene_item_list`](crate::client::SceneItems::get_scene_item_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemList {
    /// Name of the requested (or current) scene.
    pub scene_name: String,
    /// Array of scene items.
    pub scene_items: Vec<SceneItemListItem>,
}

/// Response value for
/// [`get_scene_item_list`](crate::client::SceneItems::get_scene_item_list) as part of
/// [`SceneItemList`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemListItem {
    /// Unique item id of the source item
    pub item_id: i64,
    /// ID if the scene item's source. For example `vlc_source` or `image_source`.
    pub source_kind: String,
    /// Name of the scene item's source.
    pub source_name: String,
    /// Type of the scene item's source. Either `input`, `group`, or `scene`.
    pub source_type: String,
}

/// Response value for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemProperties {
    /// Scene Item name.
    pub name: String,
    /// Scene Item ID.
    pub item_id: i64,
    /// Position of the source.
    pub position: Position,
    /// The clockwise rotation of the item in degrees around the point of alignment.
    pub rotation: f64,
    /// Scaling factor of the source.
    pub scale: Scale,
    /// Pixel cropping of the source before scaling.
    pub crop: Crop,
    /// If the source is visible.
    pub visible: bool,
    /// If the source is muted.
    #[serde(default)]
    pub muted: bool,
    /// If the source's transform is locked.
    pub locked: bool,
    /// Bounding box of the source.
    pub bounds: Bounds,
    /// Base width (without scaling) of the source.
    pub source_width: u32,
    /// Base source (without scaling) of the source.
    pub source_height: u32,
    /// Scene item width (base source width multiplied by the horizontal scaling factor).
    pub width: f64,
    /// Scene item height (base source height multiplied by the vertical scaling factor).
    pub height: f64,
    // pub alignment: u8,
    /// Name of the item's parent (if this item belongs to a group).
    pub parent_group_name: Option<String>,
    /// List of children (if this item is a group).
    #[serde(default)]
    pub group_children: Vec<SceneItemTransform>,
}

/// Response value for [`add_scene_item`](crate::client::SceneItems::add_scene_item).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SceneItemId {
    /// Numerical ID of the created scene item.
    pub item_id: i64,
}

/// Response value for [`duplicate_scene_item`](crate::client::SceneItems::duplicate_scene_item).
#[derive(Debug, Deserialize)]
pub struct DuplicateSceneItem {
    /// Name of the scene where the new item was created.
    pub scene: String,
    /// New item info.
    pub item: SceneItemSpecification,
}

/// Response value for [`get_current_scene`](crate::client::Scenes::get_current_scene).
#[derive(Debug, Deserialize)]
pub struct CurrentScene {
    /// Name of the currently active scene.
    pub name: String,
    /// Ordered list of the current scene's source items.
    pub sources: Vec<SceneItem>,
}

/// Response value for [`get_scene_list`](crate::client::Scenes::get_scene_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SceneList {
    /// Name of the currently active scene.
    pub current_scene: String,
    /// Ordered list of the current profile's scenes.
    pub scenes: Vec<Scene>,
}

/// Response value for
/// [`get_scene_transition_override`](crate::client::Scenes::get_scene_transition_override).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneTransitionOverride {
    /// Name of the current overriding transition. Empty string if no override is set.
    pub transition_name: String,
    /// Transition duration. `-1` if no override is set.
    pub transition_duration: i64,
}

/// Response value for [`get_streaming_status`](crate::client::Streaming::get_streaming_status).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StreamingStatus {
    /// Current streaming status.
    pub streaming: bool,
    /// Current recording status.
    pub recording: bool,
    /// If recording is paused.
    pub recording_paused: bool,
    /// Always false. Retrocompatibility with OBSRemote.
    #[serde(default)]
    pub preview_only: bool,
    /// Time elapsed since streaming started (only present if currently streaming).
    #[serde(default, deserialize_with = "crate::de::duration_opt")]
    pub stream_timecode: Option<Duration>,
    /// Time elapsed since recording started (only present if currently recording).
    #[serde(default, deserialize_with = "crate::de::duration_opt")]
    pub rec_timecode: Option<Duration>,
}

/// Response value for [`get_stream_settings`](crate::client::Streaming::get_stream_settings).
#[derive(Debug, Deserialize)]
pub struct GetStreamSettings {
    /// The type of streaming service configuration. Possible values: `rtmp_custom` or
    /// `rtmp_common`.
    #[serde(rename = "type")]
    pub ty: StreamType,
    /// Stream settings object.
    pub settings: StreamSettings,
}

/// Response value for
/// [`get_studio_mode_status`](crate::client::StudioMode::get_studio_mode_status).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct StudioModeStatus {
    /// Indicates if Studio Mode is enabled.
    pub studio_mode: bool,
}

/// Response value for [`get_preview_scene`](crate::client::StudioMode::get_preview_scene).
#[derive(Debug, Deserialize)]
pub struct PreviewScene {
    /// The name of the active preview scene.
    pub name: String,
    /// Array of scene items of the active preview scene.
    pub sources: Vec<SceneItem>,
}

/// Response value for [`get_transition_list`](crate::client::Transitions::get_transition_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TransitionList {
    /// Name of the currently active transition.
    pub current_transition: String,
    /// List of transitions.
    pub transitions: Vec<Transition>,
}

/// Response value for
/// [`get_current_transition`](crate::client::Transitions::get_current_transition).
#[derive(Debug, Deserialize)]
pub struct CurrentTransition {
    /// Name of the selected transition.
    pub name: String,
    /// Transition duration (in milliseconds) if supported by the transition.
    #[serde(default, deserialize_with = "crate::de::duration_millis_opt")]
    pub duration: Option<Duration>,
}

/// Response value for
/// [`get_transition_duration`](crate::client::Transitions::get_transition_duration).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct TransitionDuration {
    /// Duration of the current transition (in milliseconds).
    #[serde(deserialize_with = "crate::de::duration_millis")]
    pub transition_duration: Duration,
}

/// Response value for
/// [`get_transition_position`](crate::client::Transitions::get_transition_position).
#[derive(Debug, Deserialize)]
pub(crate) struct TransitionPosition {
    /// Current transition position. This value will be between 0.0 and 1.0.
    ///
    /// Note: Transition returns 1.0 when not active.
    pub position: f64,
}

/// Response value for
/// [`get_transition_settings`](crate::client::Transitions::get_transition_settings) and
/// [`set_transition_settings`](crate::client::Transitions::set_transition_settings).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TransitionSettings {
    /// Current or updated transition settings.
    pub transition_settings: serde_json::Value,
}

/// Response value for [`get_stats`](crate::client::General::get_stats).
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ObsStats {
    /// Current framerate.
    pub fps: f64,
    /// Number of frames rendered.
    pub render_total_frames: u64,
    /// Number of frames missed due to rendering lag.
    pub render_missed_frames: u64,
    /// Number of frames outputted.
    pub output_total_frames: u64,
    /// Number of frames skipped due to encoding lag.
    pub output_skipped_frames: u64,
    /// Average frame render time (in milliseconds).
    pub average_frame_time: f64,
    /// Current CPU usage (percentage).
    pub cpu_usage: f64,
    /// Current RAM usage (in megabytes).
    pub memory_usage: f64,
    /// Free recording disk space (in megabytes)
    pub free_disk_space: f64,
}

/// Response value for [`get_sources_list`](crate::client::Sources::get_sources_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceListItem {
    /// Unique source name.
    pub name: String,
    /// Non-unique source internal type (a.k.a kind).
    pub type_id: String,
    /// Source type.
    #[serde(rename = "type")]
    pub ty: String,
}

/// Response value for [`get_sources_types_list`](crate::client::Sources::get_sources_types_list).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceTypeItem {
    /// Non-unique internal source type ID.
    pub type_id: String,
    /// Display name of the source type.
    pub display_name: String,
    /// Type. Value is one of the following: "input", "filter", "transition" or "other".
    #[serde(rename = "type")]
    pub ty: SourceType,
    /// Default settings of this source type.
    pub default_settings: serde_json::Value,
    /// Source type capabilities.
    pub caps: Caps,
}

/// Source type as part of [`SourceTypeItem`].
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    /// Input source from outside of OBS.
    Input,
    /// Filter applied to other items.
    Filter,
    /// Transition when switching scenes.
    Transition,
    /// Other kinds of sources.
    Other,
}

/// Response value for [`get_sources_types_list`](crate::client::Sources::get_sources_types_list) as
/// part of [`SourceTypeItem`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Caps {
    /// True if source of this type provide frames asynchronously.
    pub is_async: bool,
    /// True if sources of this type provide video.
    pub has_video: bool,
    /// True if sources of this type provide audio.
    pub has_audio: bool,
    /// True if interaction with this sources of this type is possible.
    pub can_interact: bool,
    /// True if sources of this type composite one or more sub-sources.
    pub is_composite: bool,
    /// True if sources of this type should not be fully duplicated.
    pub do_not_duplicate: bool,
    /// True if sources of this type may cause a feedback loop if it's audio is monitored and
    /// shouldn't be.
    pub do_not_self_monitor: bool,
}

/// Response value for
/// [`get_text_gdi_plus_properties`](crate::client::Sources::get_text_gdi_plus_properties) as part
/// of [`TextGdiPlusProperties`] and
/// [`get_text_freetype2_properties`](crate::client::Sources::get_text_freetype2_properties) as part
/// of [`TextFreetype2Properties`].
#[derive(Debug, Deserialize)]
pub struct Font {
    /// Font face.
    pub face: String,
    /// Font text styling flag. `Bold=1, Italic=2, Bold Italic=3, Underline=5, Strikeout=8`.
    #[serde(deserialize_with = "crate::de::bitflags_u8")]
    pub flags: FontFlags,
    /// Font text size.
    pub size: u32,
    /// Font Style (unknown function).
    pub style: String,
}

/// Response value for [`get_source_filters`](crate::client::Sources::get_source_filters).
#[derive(Debug, Deserialize)]
pub struct SourceFilter {
    /// Filter status (enabled or not).
    pub enabled: bool,
    /// Filter type.
    #[serde(rename = "type")]
    pub ty: String,
    /// Filter name.
    pub name: String,
    /// Filter settings.
    pub settings: serde_json::Value,
}

/// Response value for [`list_outputs`](crate::client::Outputs::list_outputs) and
/// [`get_output_info`](crate::client::Outputs::get_output_info).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    /// Output name.
    pub name: String,
    /// Output type/kind.
    #[serde(rename = "type")]
    pub ty: String,
    /// Video output width.
    pub width: u32,
    /// Video output height.
    pub height: u32,
    /// Output flags.
    pub flags: OutputFlags,
    /// Output settings.
    pub settings: serde_json::Value,
    /// Output status (active or not).
    pub active: bool,
    /// Output reconnection status (reconnecting or not).
    pub reconnecting: bool,
    /// Output congestion.
    pub congestion: f64,
    /// Number of frames sent.
    pub total_frames: u64,
    /// Number of frames dropped.
    pub dropped_frames: u64,
    /// Total bytes sent.
    pub total_bytes: u64,
}

/// Response value for [`list_outputs`](crate::client::Outputs::list_outputs) and
/// [`get_output_info`](crate::client::Outputs::get_output_info) as part of [`Output`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputFlags {
    /// Raw flags value.
    pub raw_value: u64,
    /// Output uses audio.
    pub audio: bool,
    /// Output uses video.
    pub video: bool,
    /// Output is encoded.
    pub encoded: bool,
    /// Output uses several audio tracks.
    pub multi_track: bool,
    /// Output uses a service.
    pub service: bool,
}

/// Response value for [`list_profiles`](crate::client::Profiles::list_profiles).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    /// Profile name.
    pub profile_name: String,
}

/// Response value for
/// [`list_scene_collections`](crate::client::SceneCollections::list_scene_collections).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SceneCollection {
    /// Name of the scene collection.
    pub sc_name: String,
}

/// Response value for [`duplicate_scene_item`](crate::client::SceneItems::duplicate_scene_item) as
/// part of [`DuplicateSceneItem`].
#[derive(Debug, Deserialize)]
pub struct SceneItemSpecification {
    /// New item ID.
    pub id: i64,
    /// New item name.
    pub name: String,
}

/// Response value for [`get_scene_list`](crate::client::Scenes::get_scene_list) as part of
/// [`SceneList`].
// TODO: actually the same as `CurrentScene`.
#[derive(Clone, Debug, Deserialize)]
pub struct Scene {
    /// Name of the scene.
    pub name: String,
    /// Ordered list of the scene's source items.
    #[serde(default)]
    pub sources: Vec<SceneItem>,
}

/// Response value for [`get_stream_settings`](crate::client::Streaming::get_stream_settings) as
/// part of [`GetStreamSettings`].
#[derive(Debug, Deserialize)]
pub struct StreamSettings {
    /// The publish URL.
    pub server: String,
    /// The publish key of the stream.
    pub key: String,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    pub use_auth: bool,
    /// The username to use when accessing the streaming server. Only present if
    /// [`use_auth`](Self::use_auth) is `true`.
    pub username: Option<String>,
    /// The password to use when accessing the streaming server. Only present if
    /// [`use_auth`](Self::use_auth) is `true`.
    pub password: Option<String>,
}

/// Response value for [`get_transition_list`](crate::client::Transitions::get_transition_list) as
/// part of [`TransitionList`].
#[derive(Debug, Deserialize)]
pub struct Transition {
    /// Name of the transition.
    pub name: String,
}
