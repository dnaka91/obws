//! Additional structs for use with
//! [`Sources::set_source_settings`](crate::client::Sources::set_source_settings).

use std::path::Path;

use chrono::Duration;
use rgb::RGBA8;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_repr::Serialize_repr;

use crate::common::FontFlags;
use crate::requests::ser;

/// Identifier for input capture sources.
pub const SOURCE_COREAUDIO_INPUT_CAPTURE: &str = "coreaudio_input_capture";
/// Identifier for output capture sources.
pub const SOURCE_COREAUDIO_OUTPUT_CAPTURE: &str = "coreaudio_output_capture";
/// Identifier for browser sources.
pub const SOURCE_BROWSER_SOURCE: &str = "browser_source";
/// Identifier for color sources.
pub const SOURCE_COLOR_SOURCE_V3: &str = "color_source_v3";
/// Identifier for display capture sources.
pub const SOURCE_DISPLAY_CAPTURE: &str = "display_capture";
/// Identifier for image sources.
pub const SOURCE_IMAGE_SOURCE: &str = "image_source";
/// Identifier for image slideshow sources.
pub const SOURCE_SLIDESHOW: &str = "slideshow";
/// Identifier for FFmpeg video sources.
pub const SOURCE_FFMPEG_SOURCE: &str = "ffmpeg_source";
/// Identifier for FreeType2 text sources.
pub const SOURCE_TEXT_FT2_SOURCE_V2: &str = "text_ft2_source_v2";
/// Identifier for VLC video sources.
pub const SOURCE_VLC_SOURCE: &str = "vlc_source";
/// Identifier for audio/video input capture sources.
pub const SOURCE_AV_CAPTURE_INPUT: &str = "av_capture_input";
/// Identifier for source window capture sources.
pub const SOURCE_WINDOW_CAPTURE: &str = "window_capture";

/// Settings specific to a CoreAudio input capture source.
#[derive(Serialize)]
pub struct CoreaudioInputCapture<'a> {
    /// Input device identifier.
    pub device_id: &'a str,
}

/// Settings specific to a CoreAudio output capture source.
#[derive(Serialize)]
pub struct CoreaudioOutputCapture<'a> {
    /// Output device identifier.
    pub device_id: &'a str,
}

/// Settings specific to a browser source.
#[derive(Serialize)]
pub struct BrowserSource<'a> {
    /// Whether to use a local file instead of a remote location.
    ///
    /// If true, the [`Self::local_file`] setting is used, [`Self::url`] otherwise.
    pub is_local_file: bool,
    /// Local file to open as web page. Only used if [`Self::is_local_file`] is set to `true`.
    pub local_file: &'a Path,
    /// Remote location of a web page. Only used if [`Self::is_local_file`] is set to `false`.
    pub url: &'a str,
    /// Browser window width in pixels.
    pub width: u32,
    /// Browser window height in pixels.
    pub height: u32,
    /// Use custom frame rate.
    pub fps_custom: bool,
    /// Custom FPS, only used if [`Self::fps_custom`] is set to `true`.
    pub fps: u16,
    /// Control audio via OBS.
    pub reroute_audio: bool,
    /// Custom CSS.
    pub css: &'a str,
    /// Shutdown source when not visible.
    pub shutdown: bool,
    /// Refresh browser when scene becomes active.
    pub restart_when_active: bool,
}

impl<'a> Default for BrowserSource<'a> {
    fn default() -> Self {
        Self {
            is_local_file: false,
            local_file: Path::new(""),
            url: "https://obsproject.com/browser-source",
            width: 800,
            height: 600,
            fps_custom: false,
            fps: 30,
            reroute_audio: false,
            css: "body { background-color: rgba(0, 0, 0, 0); margin: 0px auto; overflow: hidden; }",
            shutdown: false,
            restart_when_active: false,
        }
    }
}

/// Settings specific to a color source.
#[derive(Serialize)]
pub struct ColorSourceV3 {
    /// Color to display.
    #[serde(serialize_with = "ser::rgba8_inverse")]
    pub color: RGBA8,
    /// Source width in pixels.
    pub width: u32,
    /// Source height in pixels.
    pub height: u32,
}

impl Default for ColorSourceV3 {
    fn default() -> Self {
        Self {
            color: RGBA8::new(209, 209, 209, 255),
            width: 0,
            height: 0,
        }
    }
}

/// Settings specific to a display capture source.
#[derive(Serialize)]
pub struct DisplayCapture<'a> {
    /// Index of the display to capture. Usually `0` for the main display.
    pub display: u8,
    /// Whether to show the cursor on the captured input.
    pub show_cursor: bool,
    /// Cropping of the window input.
    #[serde(flatten)]
    pub crop_mode: CropMode<'a>,
}

/// The capture cropping for a [`DisplayCapture`] source.
pub enum CropMode<'a> {
    /// Disable any cropping.
    None,
    /// Manual cropping by pixel dimensions.
    Manual {
        /// Left side cropping.
        left: f64,
        /// Top side cropping.
        top: f64,
        /// Right side cropping.
        right: f64,
        /// Bottom side cropping.
        bottom: f64,
    },
    /// Crop the capture to a specific window on the screen.
    ToWindow {
        /// Owner of the window. Usually the program name.
        owner_name: &'a str,
        /// Title of the window. Depending on the OS usually found at the top window corner.
        window_name: &'a str,
        /// ID of the window.
        window: u32,
        /// List up windows with empty names in the UI dropdown selection.
        show_empty_names: bool,
    },
    /// A combination of [`Self::ToWindow`] and [`Self::Manual`], cropping to the window first, then
    /// applying manual cropping.
    ToWindowAndManual {
        /// Owner of the window. Usually the program name.
        owner_name: &'a str,
        /// Title of the window. Depending on the OS usually found at the top window corner.
        window_name: &'a str,
        /// ID of the window.
        window: u32,
        /// List up windows with empty names in the UI dropdown selection.
        show_empty_names: bool,
        /// Left side cropping.
        left: f64,
        /// Top side cropping.
        top: f64,
        /// Right side cropping.
        right: f64,
        /// Bottom side cropping.
        bottom: f64,
    },
}

impl<'a> Serialize for CropMode<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => {
                let mut s = serializer.serialize_struct("CropMode", 1)?;
                s.serialize_field("crop_mode", &0u8)?;
                s.end()
            }
            Self::Manual {
                left,
                top,
                right,
                bottom,
            } => {
                let mut s = serializer.serialize_struct("CropMode", 5)?;
                s.serialize_field("crop_mode", &1u8)?;
                s.serialize_field("manual.origin.x", left)?;
                s.serialize_field("manual.origin.y", top)?;
                s.serialize_field("manual.size.width", right)?;
                s.serialize_field("manual.size.height", bottom)?;
                s.end()
            }
            Self::ToWindow {
                owner_name,
                window_name,
                window,
                show_empty_names,
            } => {
                let mut s = serializer.serialize_struct("CropMode", 5)?;
                s.serialize_field("crop_mode", &2u8)?;
                s.serialize_field("owner_name", owner_name)?;
                s.serialize_field("window_name", window_name)?;
                s.serialize_field("window", window)?;
                s.serialize_field("show_empty_names", show_empty_names)?;
                s.end()
            }
            CropMode::ToWindowAndManual {
                owner_name,
                window_name,
                window,
                show_empty_names,
                left,
                top,
                right,
                bottom,
            } => {
                let mut s = serializer.serialize_struct("CropMode", 9)?;
                s.serialize_field("crop_mode", &3u8)?;
                s.serialize_field("owner_name", owner_name)?;
                s.serialize_field("window_name", window_name)?;
                s.serialize_field("window", window)?;
                s.serialize_field("show_empty_names", show_empty_names)?;
                s.serialize_field("window.origin.x", left)?;
                s.serialize_field("window.origin.y", top)?;
                s.serialize_field("window.size.width", right)?;
                s.serialize_field("window.size.height", bottom)?;
                s.end()
            }
        }
    }
}

/// Settings specific to an image source.
#[derive(Serialize)]
pub struct ImageSource<'a> {
    /// Location of the file to display.
    pub file: &'a Path,
    /// Unload the image file when the source isn't visible.
    pub unload: bool,
}

impl<'a> Default for ImageSource<'a> {
    fn default() -> Self {
        Self {
            file: Path::new(""),
            unload: false,
        }
    }
}

/// Settings specific to an image slideshow source.
#[derive(Serialize)]
pub struct Slideshow<'a> {
    /// Behavior of playback in relation to visibility.
    pub playback_behavior: PlaybackBehavior,
    /// Control mode to change between pictures.
    pub slide_mode: SlideMode,
    /// Swapping animation between current and next picture.
    pub transition: Transition,
    /// Time between Slides. Minimum value is `50ms`.
    #[serde(serialize_with = "ser::duration_millis")]
    pub slide_time: Duration,
    /// Minimum value is `0ms`.
    #[serde(serialize_with = "ser::duration_millis")]
    pub transition_speed: Duration,
    /// Wether to endlessly loop the slideshow images.
    #[serde(rename = "loop")]
    pub loop_: bool,
    /// Hide when slideshow is done.
    pub hide: bool,
    /// Randomize playback.
    pub randomize: bool,
    /// Bounding Size / Aspect Ratio.
    pub use_custom_size: CustomSize,
    /// Image files.
    pub files: &'a [SlideshowFile<'a>],
}

impl<'a> Default for Slideshow<'a> {
    fn default() -> Self {
        Self {
            playback_behavior: PlaybackBehavior::AlwaysPlay,
            slide_mode: SlideMode::default(),
            transition: Transition::default(),
            slide_time: Duration::seconds(8),
            transition_speed: Duration::milliseconds(700),
            loop_: true,
            hide: false,
            randomize: false,
            use_custom_size: CustomSize::default(),
            files: &[],
        }
    }
}

#[derive(Serialize)]
pub struct SlideshowFile<'a> {
    /// Location of the file to display.
    pub value: &'a Path,
    /// Whether the file is currently visible in the source.
    pub hidden: bool,
    /// Whether the file is currently selected.
    pub selected: bool,
}

impl<'a> Default for SlideshowFile<'a> {
    fn default() -> Self {
        Self {
            value: Path::new(""),
            hidden: false,
            selected: false,
        }
    }
}

/// Playback behavior setting for use in [`Slideshow`].
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackBehavior {
    /// Always play even when not visible.
    AlwaysPlay,
    /// Stop when not visible, restart when visible.
    StopRestart,
    /// Pause when not visible, unpause when visible
    PauseUnpause,
}

/// Playback control mode for use in [`Slideshow`].
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SlideMode {
    /// Automatic.
    ModeAuto,
    /// Manual (Use hotkeys to control slideshow).
    ModeManual,
}

impl Default for SlideMode {
    fn default() -> Self {
        Self::ModeAuto
    }
}

/// Transition animation between images in a [`Slideshow`].
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Transition {
    /// Immediately replace without animation.
    Cut,
    /// Gradualy fade between the two images until the new one is fully visible.
    Fade,
    /// Swipe the new image over the old one.
    Swipe,
    /// Slide out the old and slide in the new image.
    Slide,
}

impl Default for Transition {
    fn default() -> Self {
        Self::Fade
    }
}

/// Aspect ratios and bounding sizes for use in [`Slideshow`].
#[derive(Clone, Copy, Serialize)]
#[serde(into = "String")]
pub enum CustomSize {
    /// Automatically detect a ratio based on the input.
    Automatic,
    /// 16:9 aspect ratio.
    SixteenToNine,
    /// 16:10 aspect ratio.
    SixteenToTen,
    /// 4:3 aspect ratio.
    FourToThree,
    /// 1:1 aspect ratio.
    OneToOne,
    /// Custom width:height ratio.
    CustomRatio(u32, u32),
    /// Custom width x height size.
    CustomSize(u32, u32),
}

impl Default for CustomSize {
    fn default() -> Self {
        Self::Automatic
    }
}

impl From<CustomSize> for String {
    fn from(s: CustomSize) -> Self {
        match s {
            CustomSize::Automatic => "Automatic".to_owned(),
            CustomSize::SixteenToNine => "16:9".to_owned(),
            CustomSize::SixteenToTen => "16:10".to_owned(),
            CustomSize::FourToThree => "4:3".to_owned(),
            CustomSize::OneToOne => "1:1".to_owned(),
            CustomSize::CustomRatio(w, h) => format!("{}:{}", w, h),
            CustomSize::CustomSize(w, h) => format!("{}x{}", w, h),
        }
    }
}

/// Settings specific to a FFmpeg video source.
#[derive(Serialize)]
pub struct FfmpegSource<'a> {
    pub is_local_file: bool,
    pub local_file: &'a Path,
    pub looping: bool,
    /// Network buffering in MegaBytes.
    pub buffering_mb: u8,
    pub input: &'a str,
    pub input_format: &'a str,
    /// Reconnect delay in seconds.
    pub reconnect_delay_sec: u8,
    /// Restart playback when source becomes active.
    pub restart_on_activate: bool,
    /// Show nothing when playback ends.
    pub clear_on_media_end: bool,
    /// Close file when inactive.
    pub close_when_inactive: bool,
    pub speed_percent: u8,
    /// YUV color range.
    pub color_range: ColorRange,
    pub seekable: bool,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ColorRange {
    Auto = 0,
    Partial = 1,
    Full = 2,
}

/// Settings specific to a FreeType2 text source.
#[derive(Serialize)]
pub struct TextFt2SourceV2<'a> {
    /// Draw the text with smoothed corners.
    pub antialiasing: bool,
    /// Top color of the text.
    #[serde(serialize_with = "ser::rgba8_inverse")]
    pub color1: RGBA8,
    /// Bottom color of the text.
    #[serde(serialize_with = "ser::rgba8_inverse")]
    pub color2: RGBA8,
    /// Custom width (seems to have no effect).
    pub custom_width: u32,
    /// Draw a dark blurred shadow effect behind the text.
    pub drop_shadow: bool,
    /// Settings for the font.
    pub font: Font<'a>,
    /// Load the text from a file (must be set in combination with [`text_file`]).
    pub from_file: bool,
    /// Amount of log lines if [`log_mode`] is `true`. Minimum value is `1`.
    pub log_lines: u32,
    /// Log mode (not sure what this does).
    pub log_mode: bool,
    /// Draw a black border around the text corners.
    pub outline: bool,
    /// Text to display (only used if [`from_file`] is `false`).
    pub text: &'a str,
    /// File to load the display text from ([`from_file`] must be `true`). The content must be in
    /// either **UTF-8** or **UTF-16** encoding.
    pub text_file: &'a Path,
    /// Wrap the words within the boundaries of the scene item.
    pub word_wrap: bool,
}

impl<'a> Default for TextFt2SourceV2<'a> {
    fn default() -> Self {
        Self {
            antialiasing: true,
            color1: RGBA8::new(255, 255, 255, 255),
            color2: RGBA8::new(255, 255, 255, 255),
            custom_width: 0,
            drop_shadow: false,
            font: Font::default(),
            from_file: false,
            log_lines: 6,
            log_mode: false,
            outline: false,
            text: "",
            text_file: Path::new(""),
            word_wrap: false,
        }
    }
}

/// Font settings for a [`TextFt2SourceV2`].
#[derive(Serialize)]
pub struct Font<'a> {
    /// Font face.
    pub face: &'a str,
    /// Flags for different display styles.
    #[serde(serialize_with = "ser::bitflags_u8")]
    pub flags: FontFlags,
    /// Display size.
    pub size: u32,
    /// Specific font style. Must eventually be set together with [`flags`].
    ///
    /// For example:
    /// - [`FontFlags::BOLD`] and style `"Bold"`.
    /// - [`FontFlags::ITALIC`] and style `"Italic"`.
    pub style: &'a str,
}

impl<'a> Default for Font<'a> {
    fn default() -> Self {
        Self {
            face: "Helvetica",
            flags: FontFlags::empty(),
            size: 256,
            style: "Regular",
        }
    }
}

/// Settings specific to a VLC video source.
#[derive(Serialize)]
pub struct VlcSource<'a> {
    /// Loop playlist.
    #[serde(rename = "bool")]
    pub loop_: bool,
    /// Shuffle playlist.
    pub shuffle: bool,
    /// Visibility behavior.
    pub playback_behavior: PlaybackBehavior,
    /// List of files to play.
    pub playlist: &'a [SlideshowFile<'a>],
    /// Network caching time. Mimimum value is `100ms`.
    #[serde(serialize_with = "ser::duration_millis")]
    pub network_caching: Duration,
    /// Audio track. Minimum value is `1`.
    pub track: u32,
    /// Subtitles enabled.
    pub subtitle_enable: bool,
    /// Subtitle track. Minimum value is `1`.
    pub subtitle: u32,
}

impl<'a> Default for VlcSource<'a> {
    fn default() -> Self {
        Self {
            loop_: true,
            shuffle: false,
            playback_behavior: PlaybackBehavior::StopRestart,
            playlist: &[],
            network_caching: Duration::milliseconds(400),
            track: 1,
            subtitle_enable: false,
            subtitle: 1,
        }
    }
}

/// Settings specific to an audio/video input capture source.
#[derive(Serialize)]
pub struct AvCaptureInput<'a> {
    pub buffering: bool,
    pub color_space: ColorSpace,
    pub device: &'a str,
    pub device_name: &'a str,
    pub frame_rate: FrameRate,
    pub input_format: u32,
    #[serde(serialize_with = "ser::json_string")]
    pub resolution: Resolution,
    pub use_preset: bool,
}

#[derive(Serialize_repr)]
#[repr(i8)]
pub enum ColorSpace {
    Auto = -1,
    Rec601 = 1,
    Rec709 = 2,
}

#[derive(Serialize_repr)]
#[repr(i8)]
pub enum VideoRange {
    Auto = -1,
    Partial = 1,
    Full = 2,
}

#[derive(Serialize)]
pub struct FrameRate {
    pub numerator: u64,
    pub denominator: u64,
}

#[derive(Serialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

/// Settings specific to a window capture source.
#[derive(Default, Serialize)]
pub struct WindowCapture<'a> {
    pub owner_name: &'a str,
    pub window_name: &'a str,
    pub window: u16,
    /// Show windows with empty names.
    pub show_empty_names: bool,
    /// Show window shadow.
    pub show_shadow: bool,
}
