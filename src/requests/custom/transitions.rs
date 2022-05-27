//! Additional structs for use with [`crate::client::Inputs::set_settings`].

use std::path::Path;

use rgb::RGBA8;
use serde::Serialize;
use serde_repr::Serialize_repr;

use crate::requests::ser;

/// Identifier for swipe transitions.
pub const TYPE_SWIPE: &str = "swipe_transition";
/// Identifier for slide transitions.
pub const TYPE_SLIDE: &str = "slide_transition";
/// Identifier for stinger transitions.
pub const TYPE_STINGER: &str = "obs_stinger_transition";
/// Identifier for fade to color transitions.
pub const TYPE_FADE_TO_COLOR: &str = "fade_to_color_transition";
/// Identifier for luma wipe transitions.
pub const TYPE_WIPE: &str = "wipe_transition";

/// Options for a swipe transition. A swipe describes one scene hovering over another and making
/// the other scene visible by moving in/out of the scene.
#[derive(Debug, Default, Serialize)]
pub struct Swipe {
    /// Direction of the swipe.
    pub direction: Direction,
    /// Let the new scene swipe into the screen over the current scene. Otherwise the current scene
    /// swipes out with the new scene behind it becoming visible.
    pub swipe_in: bool,
}

/// Options for a slide transition. A slide describes two scene directly next to each other making
/// one visible by "pushing" the other one away.
#[derive(Debug, Default, Serialize)]
pub struct Slide {
    /// Direction of the slide.
    pub direction: Direction,
}

/// The direction for a [`Swipe`] or [`Slide].
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// From/to the left.
    Left,
    /// From/to the right.
    Right,
    /// From/to the top.
    Up,
    /// From/to the bottom.
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Left
    }
}

/// Options for a stinger transition. A stinger describes a video being used to hide the old scene
/// completely, then switch the scene while only the video is visible. Afterwards the video moves
/// out of the view again to make the new scene visible.
#[derive(Serialize)]
pub struct Stinger<'a> {
    /// Location of the video file.
    pub path: &'a Path,
    /// The type of value that [`Self::transition_point`] stands for.
    pub tp_type: TransitionPointType,
    /// Point at which the scene transition triggers. What unit of this value depends on the set
    /// [`Self::tp_type`].
    pub transition_point: u32,
    /// The kind of audio monitoring to apply. This means whether to send the audio to the output
    /// stream, only play it locally or do it both.
    pub audio_monitoring: AudioMonitoring,
    /// The way audio is gradually swapped between two scenes.
    pub audio_fade_style: AudioFadeStyle,
}

/// Different units that are used together with a value to define scene switching point of a video
/// transition.
#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum TransitionPointType {
    /// Time in milliseconds.
    Time = 0,
    /// Frames (single images) of the video.
    Frame = 1,
}

impl Default for TransitionPointType {
    fn default() -> Self {
        Self::Time
    }
}

/// Setting for the audio monitoring which defines whether audio is send to the stream, played
/// locally or both at the same time.
#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum AudioMonitoring {
    /// No monitoring, means to insert the audio into the output stream but not playing it on the
    /// local machine.
    MonitorOff = 0,
    /// The opposite of [`Self::MonitorOff`], playing the audio locally but not sending it to
    /// the stream.
    MonitorOnly = 1,
    /// A combination of the other options where audio is send to the stream as well as played
    /// locally.
    MonitorAndOutput = 2,
}

impl Default for AudioMonitoring {
    fn default() -> Self {
        Self::MonitorOff
    }
}

/// Describes the way in which the audio is faded between two scenes with a [`Stinger`] transition.
#[derive(Clone, Copy, Debug, Serialize_repr)]
#[repr(u8)]
pub enum AudioFadeStyle {
    /// Fade out to transition point then fade in.
    FadeOutFadeIn = 0,
    /// Fade out the audio from the old scene and fade in the new scene's audio at the same time,
    /// creating a slight overlap.
    Crossfade = 1,
}

impl Default for AudioFadeStyle {
    fn default() -> Self {
        Self::FadeOutFadeIn
    }
}

/// Options for a fade to color transition. A color fading describes one scene being blended with
/// a given color until only the color is visible and then blend from the color to the new scene
/// until the color is fully gone.
#[derive(Serialize)]
pub struct FadeToColor {
    /// Color to blend in/out.
    #[serde(serialize_with = "ser::rgba8_inverse")]
    pub color: RGBA8,
    /// The point at which the scenes are swapped. Maximum value is `100`.
    pub switch_point: u8,
}

/// Options for a luma wipe transition. A luma wipe describes one scene being gradually displayed
/// over the other, where the luma image defines a certain animation to do so.
#[derive(Serialize)]
pub struct Wipe {
    /// The image to use. This describes the animation that is used.
    pub luma_image: LumaImage,
    /// Invert the animation.
    pub luma_invert: bool,
    /// Softness of the edges inside the animation where old and new scene "touch".
    pub luma_softness: f64,
}

/// A luma image that defines the animation of a [`Wipe`].
#[derive(Serialize)]
pub enum LumaImage {
    /// Barn door animation diagonal from the bottom left.
    #[serde(rename = "barndoor-botleft.png")]
    BarndoorBottomLeft,
    /// Horizontal barn door animation.
    #[serde(rename = "barndoor-h.png")]
    BarndoorHorizontal,
    /// Barn door animation diagonal from the top left.
    #[serde(rename = "barndoor-topleft.png")]
    BarndoorTopLeft,
    /// Vertical barn door animation.
    #[serde(rename = "barndoor-v.png")]
    BarndoorVertical,
    #[serde(rename = "blinds-h.png")]
    /// Horizontal blinds animation.
    BlindsHorizontal,
    /// Box animation from the bottom left.
    #[serde(rename = "box-botleft.png")]
    BoxBottomLeft,
    /// Box animation from the bottom right.
    #[serde(rename = "box-botright.png")]
    BoxBottomRight,
    /// Box animation from the top left.
    #[serde(rename = "box-topleft.png")]
    BoxTopLeft,
    /// Box animation from the top right.
    #[serde(rename = "box-topright.png")]
    BoxTopRight,
    /// Burst animation.
    #[serde(rename = "burst.png")]
    Burst,
    /// Small checkerboard animation.
    #[serde(rename = "checkerboard-small.png")]
    CheckerboardSmall,
    /// Circles animation.
    #[serde(rename = "circles.png")]
    Circles,
    /// Clock sweep animation.
    #[serde(rename = "clock.png")]
    Clock,
    /// Cloud animation.
    #[serde(rename = "cloud.png")]
    Cloud,
    /// Curtain animation.
    #[serde(rename = "curtain.png")]
    Curtain,
    /// Fan animation.
    #[serde(rename = "fan.png")]
    Fan,
    /// Fractal animation.
    #[serde(rename = "fractal.png")]
    Fractal,
    /// Iris animation.
    #[serde(rename = "iris.png")]
    Iris,
    /// Horizontal linear animation.
    #[serde(rename = "linear-h.png")]
    LinearHorizontal,
    /// Linear animation from the top left.
    #[serde(rename = "linear-topleft.png")]
    LinearTopLeft,
    /// Linear animation from the top right.
    #[serde(rename = "linear-topright.png")]
    LinearTopRight,
    /// Vertical liner animation.
    #[serde(rename = "linear-v.png")]
    LinearVertical,
    /// Horizontal parallel zig-zag animation.
    #[serde(rename = "parallel-zigzag-h.png")]
    ParallelZigzagHorizontal,
    /// Vertical parallel zig-zag animation.
    #[serde(rename = "parallel-zigzag-v.png")]
    ParallelZigzagVertical,
    /// Sinus9 animation.
    #[serde(rename = "sinus9.png")]
    Sinus9,
    /// Spiral animation.
    #[serde(rename = "spiral.png")]
    Spiral,
    /// Square animation.
    #[serde(rename = "square.png")]
    Square,
    /// Squares animation.
    #[serde(rename = "squares.png")]
    Squares,
    /// Stripes animation.
    #[serde(rename = "stripes.png")]
    Stripes,
    /// Horizontal strips animation.
    #[serde(rename = "strips-h.png")]
    StripsHorizontal,
    /// Vertical strips animation.
    #[serde(rename = "strips-v.png")]
    StripsVertical,
    /// Watercolor animation.
    #[serde(rename = "watercolor.png")]
    Watercolor,
    /// Horizontal zig-zag animation.
    #[serde(rename = "zigzag-h.png")]
    ZigzagHorizontal,
    /// Vertical zig-zag animation.
    #[serde(rename = "zigzag-v.png")]
    ZigzagVertical,
}

impl Default for LumaImage {
    fn default() -> Self {
        Self::LinearHorizontal
    }
}
