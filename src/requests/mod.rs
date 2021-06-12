//! All requests that can be send to the API.

use std::path::Path;

use bitflags::bitflags;
use chrono::Duration;
use either::Either;
use serde::Serialize;
use serde_with::skip_serializing_none;

pub use rgb::RGBA8;

use crate::common::{Align, Alignment, BoundsType, FontFlags, MonitorType, StreamType, Valign};

mod ser;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Request<'a> {
    pub message_id: &'a str,
    #[serde(flatten)]
    pub ty: RequestType<'a>,
}

#[derive(Serialize)]
#[serde(tag = "request-type")]
pub(crate) enum RequestType<'a> {
    // --------------------------------
    // General
    // --------------------------------
    GetVersion,
    GetAuthRequired,
    Authenticate {
        /// Response to the auth challenge.
        auth: &'a str,
    },
    #[serde(rename_all = "kebab-case")]
    SetFilenameFormatting {
        /// Filename formatting string to set.
        filename_formatting: &'a str,
    },
    GetFilenameFormatting,
    GetStats,
    BroadcastCustomMessage {
        /// Identifier to be choosen by the client.
        realm: &'a str,
        /// User-defined data.
        data: &'a serde_json::Value,
    },
    GetVideoInfo,
    OpenProjector(ProjectorInternal<'a>),
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyByName {
        /// Unique name of the hotkey, as defined when registering the hotkey (e.g.
        /// "ReplayBuffer.Save").
        hotkey_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    TriggerHotkeyBySequence {
        /// Main key identifier (e.g. `OBS_KEY_A` for key "A"). Available identifiers
        /// [here](https://github.com/obsproject/obs-studio/blob/master/libobs/obs-hotkeys.h).
        key_id: &'a str,
        /// Optional key modifiers object. False entries can be omitted.
        key_modifiers: &'a [KeyModifier],
    },
    // --------------------------------
    // Media Control
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    PlayPauseMedia {
        /// Source name.
        source_name: &'a str,
        /// Whether to pause or play the source. `false` for play, `true` for pause.
        play_pause: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    RestartMedia {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    StopMedia {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    NextMedia {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    PreviousMedia {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetMediaDuration {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetMediaTime {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetMediaTime {
        /// Source name.
        source_name: &'a str,
        /// Milliseconds to set the timestamp to.
        #[serde(serialize_with = "ser::duration_millis")]
        timestamp: Duration,
    },
    #[serde(rename_all = "camelCase")]
    ScrubMedia {
        /// Source name.
        source_name: &'a str,
        /// Millisecond offset (positive or negative) to offset the current media position.
        #[serde(serialize_with = "ser::duration_millis")]
        time_offset: Duration,
    },
    #[serde(rename_all = "camelCase")]
    GetMediaState {
        /// Source name.
        source_name: &'a str,
    },
    // --------------------------------
    // Sources
    // --------------------------------
    GetMediaSourcesList,
    CreateSource(CreateSource<'a>),
    GetSourcesList,
    GetSourceTypesList,
    #[serde(rename_all = "camelCase")]
    GetVolume {
        /// Source name.
        source: &'a str,
        /// Output volume in decibels of attenuation instead of amplitude/mul.
        use_decibel: Option<bool>,
    },
    SetVolume(Volume<'a>),
    GetMute {
        /// Source name.
        source: &'a str,
    },
    SetMute {
        /// Source name.
        source: &'a str,
        /// Desired mute status.
        mute: bool,
    },
    ToggleMute {
        /// Source name.
        source: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceActive {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetAudioActive {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetSourceName {
        /// Source name.
        source_name: &'a str,
        /// New source name.
        new_name: &'a str,
    },
    SetSyncOffset {
        /// Source name.
        source: &'a str,
        /// The desired audio sync offset (in nanoseconds).
        #[serde(serialize_with = "ser::duration_nanos")]
        offset: Duration,
    },
    GetSyncOffset {
        /// Source name.
        source: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceSettings {
        /// Source name.
        source_name: &'a str,
        /// Type of the specified source. Useful for type-checking if you expect a specific settings
        /// schema.
        source_type: Option<&'a str>,
    },
    SetSourceSettings(SourceSettings<'a>),
    #[serde(rename = "GetTextGDIPlusProperties")]
    GetTextGdiPlusProperties {
        /// Source name.
        source: &'a str,
    },
    #[serde(rename = "SetTextGDIPlusProperties")]
    SetTextGdiPlusProperties(Box<TextGdiPlusProperties<'a>>),
    GetTextFreetype2Properties {
        /// Source name.
        source: &'a str,
    },
    SetTextFreetype2Properties(TextFreetype2Properties<'a>),
    GetSpecialSources,
    #[serde(rename_all = "camelCase")]
    GetSourceFilters {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceFilterInfo {
        /// Source name.
        source_name: &'a str,
        /// Source filter name.
        filter_name: &'a str,
    },
    AddFilterToSource(AddFilter<'a>),
    #[serde(rename_all = "camelCase")]
    RemoveFilterFromSource {
        /// Name of the source from which the specified filter is removed.
        source_name: &'a str,
        /// Name of the filter to remove.
        filter_name: &'a str,
    },
    ReorderSourceFilter(ReorderFilter<'a>),
    MoveSourceFilter(MoveFilter<'a>),
    SetSourceFilterSettings(SourceFilterSettings<'a>),
    SetSourceFilterVisibility(SourceFilterVisibility<'a>),
    #[serde(rename_all = "camelCase")]
    GetAudioMonitorType {
        /// Source name.
        source_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetAudioMonitorType {
        /// Source name.
        source_name: &'a str,
        /// The monitor type to use. Options: `none`, `monitorOnly`, `monitorAndOutput`.
        monitor_type: MonitorType,
    },
    #[serde(rename_all = "camelCase")]
    GetSourceDefaultSettings {
        /// Source kind. Also called "source id" in libobs terminology.
        source_kind: &'a str,
    },
    TakeSourceScreenshot(SourceScreenshot<'a>),
    #[serde(rename_all = "camelCase")]
    RefreshBrowserSource {
        /// Source name.
        source_name: &'a str,
    },
    // --------------------------------
    // Outputs
    // --------------------------------
    ListOutputs,
    #[serde(rename_all = "camelCase")]
    GetOutputInfo {
        /// Output name.
        output_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    StartOutput {
        /// Output name.
        output_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    StopOutput {
        /// Output name.
        output_name: &'a str,
        /// Force stop (default: false).
        force: Option<bool>,
    },
    // --------------------------------
    // Profiles
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    SetCurrentProfile {
        /// Name of the desired profile.
        profile_name: &'a str,
    },
    GetCurrentProfile,
    ListProfiles,
    // --------------------------------
    // Recording
    // --------------------------------
    GetRecordingStatus,
    StartStopRecording,
    StartRecording,
    StopRecording,
    PauseRecording,
    ResumeRecording,
    #[serde(rename_all = "kebab-case")]
    SetRecordingFolder {
        /// Path of the recording folder.
        rec_folder: &'a Path,
    },
    GetRecordingFolder,
    // --------------------------------
    // Replay Buffer
    // --------------------------------
    GetReplayBufferStatus,
    StartStopReplayBuffer,
    StartReplayBuffer,
    StopReplayBuffer,
    SaveReplayBuffer,
    // --------------------------------
    // Scene Collections
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    SetCurrentSceneCollection {
        /// Name of the desired scene collection.
        sc_name: &'a str,
    },
    GetCurrentSceneCollection,
    ListSceneCollections,
    // --------------------------------
    // Scene Items
    // --------------------------------
    #[serde(rename_all = "camelCase")]
    GetSceneItemList {
        /// Name of the scene to get the list of scene items from. Defaults to the current scene if
        /// not specified.
        scene_name: Option<&'a str>,
    },
    #[serde(rename_all = "kebab-case")]
    GetSceneItemProperties {
        /// Name of the scene the scene item belongs to. Defaults to the current scene.
        scene_name: Option<&'a str>,
        /// Scene Item name (if this field is a string) or specification (if it is an object).
        #[serde(with = "either::serde_untagged")]
        item: Either<&'a str, SceneItemSpecification<'a>>,
    },
    SetSceneItemProperties(SceneItemProperties<'a>),
    #[serde(rename_all = "kebab-case")]
    ResetSceneItem {
        /// Name of the scene the scene item belongs to. Defaults to the current scene.
        scene_name: Option<&'a str>,
        /// Scene Item name (if this field is a string) or specification (if it is an object).
        #[serde(with = "either::serde_untagged")]
        item: Either<&'a str, SceneItemSpecification<'a>>,
    },
    SetSceneItemRender(SceneItemRender<'a>),
    DeleteSceneItem {
        /// Name of the scene the scene item belongs to. Defaults to the current scene.
        scene: Option<&'a str>,
        /// Scene item to delete.
        item: SceneItemSpecification<'a>, // TODO: fields are actually not optional
    },
    AddSceneItem(AddSceneItem<'a>),
    DuplicateSceneItem(DuplicateSceneItem<'a>),
    // --------------------------------
    // Scenes
    // --------------------------------
    #[serde(rename_all = "kebab-case")]
    SetCurrentScene {
        /// Name of the scene to switch to.
        scene_name: &'a str,
    },
    GetCurrentScene,
    GetSceneList,
    #[serde(rename_all = "camelCase")]
    CreateScene {
        /// Name of the scene to create.
        scene_name: &'a str,
    },
    ReorderSceneItems {
        /// Name of the scene to reorder (defaults to current).
        scene: Option<&'a str>,
        /// Ordered list of objects with name and/or id specified. Id preferred due to uniqueness
        /// per scene.
        items: &'a [SceneItem<'a>],
    },
    SetSceneTransitionOverride(SceneTransitionOverride<'a>),
    #[serde(rename_all = "camelCase")]
    RemoveSceneTransitionOverride {
        /// Name of the scene to remove the override from.
        scene_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    GetSceneTransitionOverride {
        /// Name of the scene to get the override for.
        scene_name: &'a str,
    },
    // --------------------------------
    // Streaming
    // --------------------------------
    GetStreamingStatus,
    StartStopStreaming,
    StartStreaming {
        /// Special stream configuration. Please note: these won't be saved to OBS' configuration.
        stream: Option<Stream<'a>>,
    },
    StopStreaming,
    SetStreamSettings(SetStreamSettings<'a>),
    GetStreamSettings,
    SaveStreamSettings,
    SendCaptions {
        /// Captions text.
        text: &'a str,
    },
    // --------------------------------
    // Studio Mode
    // --------------------------------
    GetStudioModeStatus,
    GetPreviewScene,
    #[serde(rename_all = "kebab-case")]
    SetPreviewScene {
        /// The name of the scene to preview.
        scene_name: &'a str,
    },
    TransitionToProgram {
        /// Change the active transition before switching scenes. Defaults to the active transition.
        with_transition: Option<Transition<'a>>,
    },
    EnableStudioMode,
    DisableStudioMode,
    ToggleStudioMode,
    // --------------------------------
    // Transitions
    // --------------------------------
    GetTransitionList,
    GetCurrentTransition,
    #[serde(rename_all = "kebab-case")]
    SetCurrentTransition {
        /// The name of the transition.
        transition_name: &'a str,
    },
    SetTransitionDuration {
        /// Desired duration of the transition (in milliseconds).
        #[serde(serialize_with = "ser::duration_millis")]
        duration: Duration,
    },
    GetTransitionDuration,
    GetTransitionPosition,
    #[serde(rename_all = "camelCase")]
    GetTransitionSettings {
        /// Transition name.
        transition_name: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    SetTransitionSettings {
        /// Transition name.
        transition_name: &'a str,
        /// Transition settings (they can be partial).
        transition_settings: &'a serde_json::Value,
    },
    ReleaseTBar,
    #[serde(rename_all = "camelCase")]
    SetTBarPosition {
        /// T-Bar position. This value must be between 0.0 and 1.0.
        position: f64,
        /// Whether or not the T-Bar gets released automatically after setting its new position
        /// (like a user releasing their mouse button after moving the T-Bar). Call `ReleaseTBar`
        /// manually if you set `release` to false. Defaults to true.
        release: Option<bool>,
    },
    // --------------------------------
    // Virtual Cam
    // --------------------------------
    GetVirtualCamStatus,
    StartStopVirtualCam,
    StartVirtualCam,
    StopVirtualCam,
}

/// Request information for [`open_projector`](crate::client::General::open_projector).
#[derive(Debug, Default)]
pub struct Projector<'a> {
    /// Type of projector: `Preview` (default), `Source`, `Scene`, `StudioProgram`, or `Multiview`
    /// (case insensitive).
    pub ty: Option<ProjectorType>,
    /// Monitor to open the projector on. If -1 or omitted, opens a window.
    pub monitor: Option<i64>,
    /// Size and position of the projector window (only if monitor is -1). Encoded in Base64 using
    /// [Qt's geometry encoding](https://doc.qt.io/qt-5/qwidget.html#saveGeometry). Corresponds to
    /// OBS's saved projectors.
    pub geometry: Option<&'a QtGeometry>,
    /// Name of the source or scene to be displayed (ignored for other projector types).
    pub name: Option<&'a str>,
}

/// Request information for [`open_projector`](crate::client::General::open_projector) as part of
/// [`Projector`].
#[derive(Clone, Copy, Debug, Serialize)]
pub enum ProjectorType {
    /// Open a projector of the preview area.
    Preview,
    /// Open a projector for a source.
    Source,
    /// Open a projector for a scene.
    Scene,
    /// Open a projector of the program pane in studio mode.
    StudioProgram,
    /// Open a projector in multiview.
    Multiview,
}

/// Internal version of [`Projector`] which is the one that's actually send to the API. This allows
/// to let the user define a typed version of the geometry while sending the encoded version through
/// this struct.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub(crate) struct ProjectorInternal<'a> {
    /// Type of projector: `Preview` (default), `Source`, `Scene`, `StudioProgram`, or `Multiview`
    /// (case insensitive).
    #[serde(rename = "type")]
    pub ty: Option<ProjectorType>,
    /// Monitor to open the projector on. If -1 or omitted, opens a window.
    pub monitor: Option<i64>,
    /// Size and position of the projector window (only if monitor is -1). Encoded in Base64 using
    /// [Qt's geometry encoding](https://doc.qt.io/qt-5/qwidget.html#saveGeometry). Corresponds to
    /// OBS's saved projectors.
    pub geometry: Option<&'a str>,
    /// Name of the source or scene to be displayed (ignored for other projector types).
    pub name: Option<&'a str>,
}

/// Request information for
/// [`trigger_hotkey_by_sequence`](crate::client::General::trigger_hotkey_by_sequence).
#[derive(Debug, Default, Serialize)]
pub struct KeyModifier {
    /// Trigger Shift key.
    pub shift: bool,
    /// Trigger Alt key.
    pub alt: bool,
    /// Trigger Control (Ctrl) key.
    pub control: bool,
    /// Trigger Command key (Mac).
    pub command: bool,
}

/// Request information for [`create_source`](crate::client::Sources::create_source).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSource<'a> {
    /// Source name.
    pub source_name: &'a str,
    /// Source kind, Eg. `vlc_source`.
    pub source_kind: &'a str,
    /// Scene to add the new source to.
    pub scene_name: &'a str,
    /// Source settings data.
    pub source_settings: Option<&'a serde_json::Value>,
    /// Set the created SceneItem as visible or not. Defaults to true.
    pub set_visible: Option<bool>,
}

/// Request information for [`set_volume`](crate::client::Sources::set_volume).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume<'a> {
    /// Source name.
    pub source: &'a str,
    /// Desired volume. Must be between `0.0` and `20.0` for mul, and under 26.0 for dB. OBS will
    /// interpret dB values under -100.0 as Inf. Note: The OBS volume sliders only reach a maximum
    /// of 1.0mul/0.0dB, however OBS actually supports larger values.
    pub volume: f64,
    /// Interpret `volume` data as decibels instead of amplitude/mul.
    pub use_decibel: Option<bool>,
}

/// Request information for [`set_source_settings`](crate::client::Sources::set_source_settings).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceSettings<'a> {
    /// Source name.
    pub source_name: &'a str,
    /// Type of the specified source. Useful for type-checking to avoid settings a set of settings
    /// incompatible with the actual source's type.
    pub source_type: Option<&'a str>,
    /// Source settings (varies between source types, may require some probing around).
    pub source_settings: &'a serde_json::Value,
}

/// Request information for
/// [`set_text_gdi_plus_properties`](crate::client::Sources::set_text_gdi_plus_properties).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct TextGdiPlusProperties<'a> {
    /// Name of the source.
    pub source: &'a str,
    /// Text Alignment ("left", "center", "right").
    pub align: Option<Align>,
    /// Background color.
    pub bk_color: Option<u32>,
    /// Background opacity (0-100).
    pub bk_opacity: Option<u8>,
    /// Chat log.
    pub chatlog: Option<bool>,
    /// Chat log lines.
    pub chatlog_lines: Option<u64>,
    /// Text color.
    pub color: Option<u32>,
    /// Extents wrap.
    pub extents: Option<bool>,
    /// Extents cx.
    pub extents_cx: Option<i64>,
    /// Extents cy.
    pub extents_cy: Option<i64>,
    /// File path name.
    pub file: Option<&'a Path>,
    /// Read text from the specified file.
    pub read_from_file: Option<bool>,
    /// Holds data for the font. Ex:
    /// `"font": { "face": "Arial", "flags": 0, "size": 150, "style": "" }`.
    pub font: Option<Font<'a>>,
    /// Gradient enabled.
    pub gradient: Option<bool>,
    /// Gradient color.
    pub gradient_color: Option<u32>,
    /// Gradient direction.
    pub gradient_dir: Option<f32>,
    /// Gradient opacity (0-100).
    pub gradient_opacity: Option<u8>,
    /// Outline.
    pub outline: Option<bool>,
    /// Outline color.
    pub outline_color: Option<u32>,
    /// Outline size.
    pub outline_size: Option<u64>,
    /// Outline opacity (0-100).
    pub outline_opacity: Option<u8>,
    /// Text content to be displayed.
    pub text: Option<&'a str>,
    /// Text vertical alignment ("top", "center", "bottom").
    pub valign: Option<Valign>,
    /// Vertical text enabled.
    pub vertical: Option<bool>,
    /// Visibility of the scene item.
    pub render: Option<bool>,
}

/// Request information for
/// [`set_text_freetype2_properties`](crate::client::Sources::set_text_freetype2_properties).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct TextFreetype2Properties<'a> {
    /// Source name.
    pub source: &'a str,
    /// Gradient top color.
    #[serde(serialize_with = "ser::rgba8_inverse_opt")]
    pub color1: Option<RGBA8>,
    /// Gradient bottom color.
    #[serde(serialize_with = "ser::rgba8_inverse_opt")]
    pub color2: Option<RGBA8>,
    /// Custom width (0 to disable).
    pub custom_width: Option<u32>,
    /// Drop shadow.
    pub drop_shadow: Option<bool>,
    /// Holds data for the font. Ex:
    /// `"font": { "face": "Arial", "flags": 0, "size": 150, "style": "" }`.
    pub font: Option<Font<'a>>,
    /// Read text from the specified file.
    pub from_file: Option<bool>,
    /// Chat log.
    pub log_mode: Option<bool>,
    /// Outline.
    pub outline: Option<bool>,
    /// Text content to be displayed.
    pub text: Option<&'a str>,
    /// File path.
    pub text_file: Option<&'a Path>,
    /// Word wrap.
    pub word_wrap: Option<bool>,
}

impl<'a> From<&'a crate::responses::TextFreetype2Properties> for TextFreetype2Properties<'a> {
    fn from(p: &'a crate::responses::TextFreetype2Properties) -> Self {
        Self {
            source: &p.source,
            color1: p.color1,
            color2: p.color2,
            custom_width: p.custom_width,
            drop_shadow: Some(p.drop_shadow),
            font: p.font.as_ref().map(Into::into),
            from_file: Some(p.from_file),
            log_mode: Some(p.log_mode),
            outline: Some(p.outline),
            text: Some(&p.text),
            text_file: p.text_file.as_deref(),
            word_wrap: Some(p.word_wrap),
        }
    }
}

/// Request information for [`add_filter_to_source`](crate::client::Sources::add_filter_to_source).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddFilter<'a> {
    /// Name of the source on which the filter is added.
    pub source_name: &'a str,
    /// Name of the new filter.
    pub filter_name: &'a str,
    /// Filter type.
    pub filter_type: &'a str,
    /// Filter settings.
    pub filter_settings: &'a serde_json::Value,
}

/// Request information for
/// [`reorder_source_filter`](crate::client::Sources::reorder_source_filter).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderFilter<'a> {
    /// Name of the source to which the filter belongs.
    pub source_name: &'a str,
    /// Name of the filter to reorder.
    pub filter_name: &'a str,
    /// Desired position of the filter in the chain.
    pub new_index: u32,
}

/// Request information for [`move_source_filter`](crate::client::Sources::move_source_filter).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveFilter<'a> {
    /// Name of the source to which the filter belongs.
    pub source_name: &'a str,
    /// Name of the filter to reorder.
    pub filter_name: &'a str,
    /// How to move the filter around in the source's filter chain. Either "up", "down", "top" or
    /// "bottom".
    pub movement_type: MovementType,
}

/// Request information for [`move_source_filter`](crate::client::Sources::move_source_filter) as
/// part of [`MoveFilter`].
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MovementType {
    /// Move up by one position.
    Up,
    /// Move down by one position.
    Down,
    /// Move to the very top.
    Top,
    /// Move to the very bottom.
    Bottom,
}

/// Request information for
/// [`set_source_filter_settings`](crate::client::Sources::set_source_filter_settings).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFilterSettings<'a> {
    /// Name of the source to which the filter belongs.
    pub source_name: &'a str,
    /// Name of the filter to reconfigure.
    pub filter_name: &'a str,
    /// New settings. These will be merged to the current filter settings.
    pub filter_settings: &'a serde_json::Value,
}

/// Request information for
/// [`set_source_filter_visibility`](crate::client::Sources::set_source_filter_visibility).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFilterVisibility<'a> {
    /// Source name.
    pub source_name: &'a str,
    /// Source filter name.
    pub filter_name: &'a str,
    /// New filter state.
    pub filter_enabled: bool,
}

/// Request information for
/// [`take_source_screenshot`](crate::client::Sources::take_source_screenshot).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceScreenshot<'a> {
    /// Source name.
    ///
    /// Note: Since scenes are also sources, you can also provide a scene name. If not provided, the
    /// currently active scene is used.
    pub source_name: Option<&'a str>,
    /// Format of the Data URI encoded picture. Can be "png", "jpg", "jpeg" or "bmp" (or any other
    /// value supported by Qt's Image module).
    pub embed_picture_format: Option<&'a str>,
    /// Full file path (file extension included) where the captured image is to be saved. Can be in
    /// a format different from [`embed_picture_format`](SourceScreenshot::embed_picture_format).
    /// Can be a relative path.
    pub save_to_file_path: Option<&'a Path>,
    /// Format to save the image file as (one of the values provided in the
    /// [`supported_image_export_formats`](crate::responses::Version::supported_image_export_formats)
    /// response field of [`get_version`](crate::client::General::get_version)). If not specified,
    /// tries to guess based on file extension.
    pub file_format: Option<&'a str>,
    /// Compression ratio between -1 and 100 to write the image with. -1 is automatic, 1 is smallest
    /// file/most compression, 100 is largest file/least compression. Varies with image type.
    pub compress_quality: Option<i8>,
    /// Screenshot width. Defaults to the source's base width.
    pub width: Option<u32>,
    /// Screenshot height. Defaults to the source's base height.
    pub height: Option<u32>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SceneItemProperties<'a> {
    /// Name of the scene the source item belongs to. Defaults to the current scene.
    pub scene_name: Option<&'a str>,
    /// Scene Item name (if this field is a string) or specification (if it is an object).
    #[serde(with = "either::serde_untagged")]
    pub item: Either<&'a str, SceneItemSpecification<'a>>,
    /// Position of the scene item.
    pub position: Option<Position>,
    /// The new clockwise rotation of the item in degrees.
    pub rotation: Option<f64>,
    /// Scaling factor of the scene item.
    pub scale: Option<Scale>,
    /// Pixel cropping of the scene item before scaling.
    pub crop: Option<Crop>,
    /// The new visibility of the source. 'true' shows source, 'false' hides source.
    pub visible: Option<bool>,
    /// The new locked status of the source. 'true' keeps it in its current position, 'false' allows
    /// movement.
    pub locked: Option<bool>,
    /// Bounding box of the scene item.
    pub bounds: Option<Bounds>,
}

impl<'a> Default for SceneItemProperties<'a> {
    fn default() -> Self {
        Self {
            scene_name: None,
            item: Either::Left(""),
            position: None,
            rotation: None,
            scale: None,
            crop: None,
            visible: None,
            locked: None,
            bounds: None,
        }
    }
}

/// Request information for
/// [`set_scene_item_render`](crate::client::SceneItems::set_scene_item_render).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SceneItemRender<'a> {
    /// Name of the scene the scene item belongs to. Defaults to the currently active scene.
    pub scene_name: Option<&'a str>,
    /// Scene Item name.
    pub source: &'a str,
    /// Scene Item id.
    pub item: Option<i64>,
    /// true = shown ; false = hidden.
    pub render: bool,
}

/// Request information for [`add_scene_item`](crate::client::SceneItems::add_scene_item).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSceneItem<'a> {
    /// Name of the scene to create the scene item in.
    pub scene_name: &'a str,
    /// Name of the source to be added.
    pub source_name: &'a str,
    /// Whether to make the sceneitem visible on creation or not. Default `true`.
    pub set_visible: bool,
}

/// Request information for
/// [`duplicate_scene_item`](crate::client::SceneItems::duplicate_scene_item).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateSceneItem<'a> {
    /// Name of the scene to copy the item from. Defaults to the current scene.
    pub from_scene: Option<&'a str>,
    /// Name of the scene to create the item in. Defaults to the current scene.
    pub to_scene: Option<&'a str>,
    /// Scene Item to duplicate from the source scene.
    pub item: SceneItemSpecification<'a>, // TODO: fields are actually not optional
}

/// Request information for
/// [`set_scene_transition_override`](crate::client::Scenes::set_scene_transition_override).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SceneTransitionOverride<'a> {
    /// Name of the scene to switch to.
    pub scene_name: &'a str,
    /// Name of the transition to use.
    pub transition_name: &'a str,
    /// Duration in milliseconds of the transition if transition is not fixed. Defaults to the
    /// current duration specified in the UI if there is no current override and this value is not
    /// given.
    #[serde(serialize_with = "ser::duration_millis_opt")]
    pub transition_duration: Option<Duration>,
}

/// Request information for [`set_stream_settings`](crate::client::Streaming::set_stream_settings).
#[derive(Debug, Serialize)]
pub struct SetStreamSettings<'a> {
    /// The type of streaming service configuration, usually `rtmp_custom` or `rtmp_common`.
    #[serde(rename = "type")]
    pub ty: StreamType,
    /// The actual settings of the stream.
    pub settings: StreamSettings<'a>,
    /// Persist the settings to disk.
    pub save: bool,
}

/// Request information for
/// [`set_text_gdi_plus_properties`](crate::client::Sources::set_text_gdi_plus_properties) as part
/// of [`TextGdiPlusProperties`] and
/// [`set_text_freetype2_properties`](crate::client::Sources::set_text_freetype2_properties) as part
/// of [`TextFreetype2Properties`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Font<'a> {
    /// Font face.
    pub face: Option<&'a str>,
    /// Font text styling flag. `Bold=1, Italic=2, Bold Italic=3, Underline=5, Strikeout=8`.
    #[serde(serialize_with = "ser::bitflags_u8_opt")]
    pub flags: Option<FontFlags>,
    /// Font text size.
    pub size: Option<u32>,
    /// Font Style (unknown function).
    pub style: Option<&'a str>,
}

impl<'a> From<&'a crate::responses::Font> for Font<'a> {
    fn from(f: &'a crate::responses::Font) -> Self {
        Self {
            face: Some(&f.face),
            flags: Some(f.flags),
            size: Some(f.size),
            style: Some(&f.style),
        }
    }
}

/// Request information for
/// [`get_scene_item_properties`](crate::client::SceneItems::get_scene_item_properties),
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`], [`reset_scene_item`](crate::client::SceneItems::reset_scene_item),
/// [`delete_scene_item`](crate::client::SceneItems::delete_scene_item) and
/// [`duplicate_scene_item`](crate::client::SceneItems::duplicate_scene_item) as part of
/// [`DuplicateSceneItem`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct SceneItemSpecification<'a> {
    /// Scene Item name.
    pub name: Option<&'a str>,
    /// Scene Item ID.
    pub id: Option<i64>,
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Position {
    /// The new x position of the source.
    pub x: Option<f64>,
    /// The new y position of the source.
    pub y: Option<f64>,
    /// The new alignment of the source.
    #[serde(serialize_with = "ser::bitflags_u8_opt")]
    pub alignment: Option<Alignment>,
}

impl From<&crate::common::Position> for Position {
    fn from(p: &crate::common::Position) -> Self {
        Self {
            x: Some(p.x),
            y: Some(p.y),
            alignment: Some(p.alignment),
        }
    }
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Scale {
    /// The new x scale of the item.
    pub x: Option<f64>,
    /// The new y scale of the item.
    pub y: Option<f64>,
}

impl From<&crate::common::Scale> for Scale {
    fn from(s: &crate::common::Scale) -> Self {
        Self {
            x: Some(s.x),
            y: Some(s.y),
        }
    }
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Crop {
    /// The new amount of pixels cropped off the top of the source before scaling.
    pub top: Option<u32>,
    /// The new amount of pixels cropped off the bottom of the source before scaling.
    pub bottom: Option<u32>,
    /// The new amount of pixels cropped off the left of the source before scaling.
    pub left: Option<u32>,
    /// The new amount of pixels cropped off the right of the source before scaling.
    pub right: Option<u32>,
}

impl From<&crate::common::Crop> for Crop {
    fn from(c: &crate::common::Crop) -> Self {
        Self {
            top: Some(c.top),
            bottom: Some(c.bottom),
            left: Some(c.left),
            right: Some(c.right),
        }
    }
}

/// Request information for
/// [`set_scene_item_properties`](crate::client::SceneItems::set_scene_item_properties) as part of
/// [`SceneItemProperties`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Bounds {
    /// The new bounds type of the source. Can be "OBS_BOUNDS_STRETCH", "OBS_BOUNDS_SCALE_INNER",
    /// "OBS_BOUNDS_SCALE_OUTER", "OBS_BOUNDS_SCALE_TO_WIDTH", "OBS_BOUNDS_SCALE_TO_HEIGHT",
    /// "OBS_BOUNDS_MAX_ONLY" or "OBS_BOUNDS_NONE".
    #[serde(rename = "type")]
    pub ty: Option<BoundsType>,
    /// The new alignment of the bounding box. (0-2, 4-6, 8-10).
    #[serde(serialize_with = "ser::bitflags_u8_opt")]
    pub alignment: Option<Alignment>,
    /// The new width of the bounding box.
    pub x: Option<f64>,
    /// The new height of the bounding box.
    pub y: Option<f64>,
}

impl From<&crate::common::Bounds> for Bounds {
    fn from(b: &crate::common::Bounds) -> Self {
        Self {
            ty: Some(b.ty),
            alignment: Some(b.alignment),
            x: Some(b.x),
            y: Some(b.y),
        }
    }
}

/// Request information for
/// [`reorder_scene_items`](crate::client::Scenes::reorder_scene_items).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct SceneItem<'a> {
    /// Id of a specific scene item. Unique on a scene by scene basis.
    pub id: Option<i64>,
    /// Name of a scene item. Sufficiently unique if no scene items share sources within the scene.
    pub name: Option<&'a str>,
}

/// Request information for [`start_streaming`](crate::client::Streaming::start_streaming).
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Stream<'a> {
    /// If specified ensures the type of stream matches the given type (usually 'rtmp_custom' or
    /// 'rtmp_common'). If the currently configured stream type does not match the given stream
    /// type, all settings must be specified in the `settings` object or an error will occur when
    /// starting the stream.
    #[serde(rename = "type")]
    pub ty: Option<StreamType>,
    /// Adds the given object parameters as encoded query string parameters to the 'key' of the RTMP
    /// stream. Used to pass data to the RTMP service about the streaming. May be any String,
    /// Numeric, or Boolean field.
    pub metadata: Option<&'a serde_json::Value>,
    /// Settings for the stream.
    pub settings: Option<StreamSettings<'a>>,
}

/// Request information for [`start_streaming`](crate::client::Streaming::start_streaming) as part
/// of [`Stream`] and [`set_stream_settings`](crate::client::Streaming::set_stream_settings) as part
/// of [`SetStreamSettings`].
#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct StreamSettings<'a> {
    /// The publish URL.
    pub server: Option<&'a str>,
    /// The publish key of the stream.
    pub key: Option<&'a str>,
    /// Indicates whether authentication should be used when connecting to the streaming server.
    pub use_auth: Option<bool>,
    /// If authentication is enabled, the username for the streaming server. Ignored if
    /// [`use_auth`](Self::use_auth) is not set to `true`.
    pub username: Option<&'a str>,
    /// If authentication is enabled, the password for the streaming server. Ignored if
    /// [`use_auth`](Self::use_auth) is not set to `true`.
    pub password: Option<&'a str>,
}

/// Request information for
/// [`transition_to_program`](crate::client::StudioMode::transition_to_program).
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Transition<'a> {
    /// Name of the transition.
    pub name: &'a str,
    /// Transition duration (in milliseconds).
    #[serde(serialize_with = "ser::duration_millis_opt")]
    pub duration: Option<Duration>,
}

/// Request information for [`open_projector`](crate::client::General::open_projector) as part of
/// [`Projector`].
///
/// This describes a position on the screen starting from the top left corner with 0.
///
/// ```txt
/// Screen
/// ┌────────────────────── X
/// │
/// │          top
/// │       ┌────────┐
/// │  left │  Rect  │ right
/// │       └────────┘
/// │         bottom
/// │
/// Y
///
#[derive(Clone, Copy, Debug, Default)]
pub struct QtRect {
    /// Left or X/horizontal position of the rectangle.
    pub left: i32,
    /// Top or Y/vertical position of the rectangle.
    pub top: i32,
    /// The right side of a rectangle counted from the left. For example with left = 100 and right =
    /// 300 the width would be 200.
    pub right: i32,
    /// Bottom side of a rectangle counted from the top. For example with top = 100 and bottom = 300
    /// the height would be 200.
    pub bottom: i32,
}

bitflags! {
    /// Request information for [`open_projector`](crate::client::General::open_projector) as part of
    /// [`Projector`].
    #[derive(Default)]
    pub struct QtWindowState: u32 {
        /// Window with maximum size, taking up as much space as possible but still showing
        /// the window frame.
        const MAXIMIZED = 2;
        /// Show the window in fullscreen mode, taking up the whole display.
        const FULLSCREEN = 4;
    }
}

impl QtWindowState {
    /// Convert the state into a byte array for usage in [`QtGeometry::serialize`] .
    fn to_be_bytes(&self) -> [u8; 2] {
        [
            if self.contains(Self::MAXIMIZED) { 1 } else { 0 },
            if self.contains(Self::FULLSCREEN) {
                1
            } else {
                0
            },
        ]
    }
}

/// Request information for [`open_projector`](crate::client::General::open_projector) as part of
/// [`Projector`].
#[derive(Debug)]
pub struct QtGeometry {
    /// The screen number to display a widget or [`Self::DEFAULT_SCREEN`] to let OBS pick the
    /// default.
    pub screen_number: i32,
    /// Additional window state like maximized or fullscreen.
    pub window_state: QtWindowState,
    /// The width of the screen. Seems to have no specific effect but is used for some internal
    /// calculations in Qt.
    pub screen_width: i32,
    /// The target position and size for a widget to display at.
    pub rect: QtRect,
}

impl QtGeometry {
    /// Value indicating to use the default screen.
    pub const DEFAULT_SCREEN: i32 = -1;

    /// Create a new geometry instance without only size information set.
    pub fn new(rect: QtRect) -> Self {
        Self {
            rect,
            ..Self::default()
        }
    }

    /// Serialize this instance into a base64 encoded byte array.
    ///
    /// The exact format can be found in the
    /// [Qt source code](https://code.woboq.org/qt5/qtbase/src/widgets/kernel/qwidget.cpp.html#_ZNK7QWidget12saveGeometryEv).
    ///
    /// | Length | Content                                            |
    /// |--------|----------------------------------------------------|
    /// | 4      | Magic number                                       |
    /// | 2      | Major format version                               |
    /// | 2      | Minor format version                               |
    /// | 16     | Frame rect (lef, top, right, bottom) 4 bytes each  |
    /// | 16     | Normal rect (lef, top, right, bottom) 4 bytes each |
    /// | 4      | Screen number                                      |
    /// | 1      | Window maximized (1 or 0)                          |
    /// | 1      | Window fullscreen (1 or 0)                         |
    /// | 4      | Screen width                                       |
    /// | 16     | Main rect (lef, top, right, bottom) 4 bytes each   |
    pub(crate) fn serialize(&self) -> String {
        /// Indicator for serialized Qt geometry data.
        const MAGIC_NUMBER: u32 = 0x1D9D0CB;
        /// Major version of this format.
        const MAJOR_VERSION: u16 = 3;
        /// Minor version of this format.
        const MINOR_VERSION: u16 = 0;
        /// Output data length BEFORE base64 encoding. This allows to reduce allocations in the
        /// byte buffer and must be updated whenever the format changes.
        const DATA_LENGTH: usize = 66;

        fn serialize_rect(data: &mut Vec<u8>, rect: &QtRect) {
            data.extend(&rect.left.to_be_bytes());
            data.extend(&rect.top.to_be_bytes());
            data.extend(&rect.right.to_be_bytes());
            data.extend(&rect.bottom.to_be_bytes());
        }

        let mut data = Vec::<u8>::with_capacity(DATA_LENGTH);

        data.extend(&MAGIC_NUMBER.to_be_bytes());
        data.extend(&MAJOR_VERSION.to_be_bytes());
        data.extend(&MINOR_VERSION.to_be_bytes());

        serialize_rect(&mut data, &self.rect); // frame geometry
        serialize_rect(&mut data, &self.rect); // normal geometry

        data.extend(&self.screen_number.to_be_bytes());
        data.extend(&self.window_state.to_be_bytes());
        data.extend(&self.screen_width.to_be_bytes());

        serialize_rect(&mut data, &self.rect);

        base64::encode(data)
    }
}

impl Default for QtGeometry {
    fn default() -> Self {
        Self {
            screen_number: Self::DEFAULT_SCREEN,
            window_state: QtWindowState::default(),
            screen_width: 0,
            rect: QtRect::default(),
        }
    }
}
