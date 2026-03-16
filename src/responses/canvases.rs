//! Responses related to scenes.

use bitflags::bitflags;
use derive_more::Debug;
use serde::{Deserialize, Serialize};

use super::config::VideoSettings;
pub use super::ids::CanvasId;
use crate::common::FlagsDebug;

#[derive(Debug, Deserialize)]
pub(crate) struct Canvases {
    /// Array of canvases.
    #[serde(rename = "canvases")]
    pub canvases: Vec<Canvas>,
}

/// Response value for [`crate::client::Canvases::list`].
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Canvas {
    /// Identifier of the canvas.
    #[serde(flatten)]
    pub id: CanvasId,
    /// Flags of the canvas.
    #[serde(rename = "canvasFlags")]
    pub flags: CanvasFlags,
    /// Video settings of the canvas.
    #[serde(rename = "canvasVideoSettings")]
    pub video_settings: VideoSettings,
}

/// Different flags for canvases defining their properties.
#[derive(
    Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Debug, Deserialize, Serialize,
)]
#[serde(from = "Flags", into = "Flags")]
pub struct CanvasFlags(#[debug("{:?}", FlagsDebug(self))] u8);

bitflags! {
    impl CanvasFlags: u8 {
        /// Main canvas created by libobs, cannot be renamed or reset, cannot be set by user.
        const MAIN = 1 << 0;
        /// Canvas sources will become active when they are visible.
        const ACTIVATE = 1 << 1;
        /// Audio from channels in this canvas will be mixed into the audio output.
        const MIX_AUDIO = 1 << 2;
        /// Canvas will hold references for scene sources.
        const SCENE_REF = 1 << 3;
        /// Indicates this canvas is not supposed to be saved.
        const EPHEMERAL = 1 << 4;

        /// Program preset.
        const PROGRAM = Self::ACTIVATE.bits() | Self::MIX_AUDIO.bits() | Self::SCENE_REF.bits();
        /// Preview preset.
        const PREVIEW = Self::EPHEMERAL.bits();
        /// Device preset.
        const DEVICE = Self::ACTIVATE.bits() | Self::EPHEMERAL.bits();
    }
}

impl From<Flags> for CanvasFlags {
    fn from(value: Flags) -> Self {
        let mut flags = Self::empty();
        flags.set(Self::MAIN, value.main);
        flags.set(Self::ACTIVATE, value.activate);
        flags.set(Self::MIX_AUDIO, value.mix_audio);
        flags.set(Self::SCENE_REF, value.scene_ref);
        flags.set(Self::EPHEMERAL, value.ephemeral);
        flags
    }
}

impl From<CanvasFlags> for Flags {
    fn from(value: CanvasFlags) -> Self {
        Self {
            main: value.contains(CanvasFlags::MAIN),
            activate: value.contains(CanvasFlags::ACTIVATE),
            mix_audio: value.contains(CanvasFlags::MIX_AUDIO),
            scene_ref: value.contains(CanvasFlags::SCENE_REF),
            ephemeral: value.contains(CanvasFlags::EPHEMERAL),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Flags {
    #[serde(default)]
    main: bool,
    #[serde(default)]
    activate: bool,
    #[serde(default)]
    mix_audio: bool,
    #[serde(default)]
    scene_ref: bool,
    #[serde(default)]
    ephemeral: bool,
}
