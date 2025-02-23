use serde::{Serialize, de::DeserializeOwned};
use time::Duration;

use super::Client;
use crate::{
    common::MonitorType,
    error::Result,
    requests::inputs::{
        Create, CreateInputInternal, InputId, Request, SetSettings, SetSettingsInternal, Volume,
    },
    responses::inputs as responses,
};

/// API functions related to inputs.
pub struct Inputs<'a> {
    pub(super) client: &'a Client,
}

impl Inputs<'_> {
    /// Gets an array of all inputs in OBS.
    #[doc(alias = "GetInputList")]
    pub async fn list(&self, kind: Option<&str>) -> Result<Vec<responses::Input>> {
        self.client
            .send_message::<_, responses::Inputs>(Request::List { kind })
            .await
            .map(|i| i.inputs)
    }

    /// Gets an array of all available input kinds in OBS.
    #[doc(alias = "GetInputKindList")]
    pub async fn list_kinds(&self, unversioned: bool) -> Result<Vec<String>> {
        self.client
            .send_message::<_, responses::InputKinds>(Request::ListKinds { unversioned })
            .await
            .map(|ik| ik.input_kinds)
    }

    /// Gets the names of all special inputs.
    #[doc(alias = "GetSpecialInputs")]
    pub async fn specials(&self) -> Result<responses::SpecialInputs> {
        self.client.send_message(Request::Specials).await
    }

    /// Gets the default settings for an input kind.
    #[doc(alias = "GetInputDefaultSettings")]
    pub async fn default_settings<T>(&self, kind: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message::<_, responses::DefaultInputSettings<T>>(Request::DefaultSettings {
                kind,
            })
            .await
            .map(|dis| dis.default_input_settings)
    }

    /// Gets the settings of an input.
    ///
    /// **Note:** Does not include defaults. To create the entire settings object, overlay input
    /// settings over the default input settings provided by [`Inputs::default_settings`].
    #[doc(alias = "GetInputSettings")]
    pub async fn settings<T>(&self, input: InputId<'_>) -> Result<responses::InputSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client.send_message(Request::Settings { input }).await
    }

    /// Sets the settings of an input.
    #[doc(alias = "SetInputSettings")]
    pub async fn set_settings<T>(&self, settings: SetSettings<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetSettings(SetSettingsInternal {
                input: settings.input,
                settings: serde_json::to_value(settings.settings)
                    .map_err(crate::error::SerializeCustomDataError)?,
                overlay: settings.overlay,
            }))
            .await
    }

    /// Gets the audio mute state of an input.
    #[doc(alias = "GetInputMute")]
    pub async fn muted(&self, input: InputId<'_>) -> Result<bool> {
        self.client
            .send_message::<_, responses::InputMuted>(Request::Muted { input })
            .await
            .map(|im| im.muted)
    }

    /// Sets the audio mute state of an input.
    #[doc(alias = "SetInputMute")]
    pub async fn set_muted(&self, input: InputId<'_>, muted: bool) -> Result<()> {
        self.client
            .send_message(Request::SetMuted { input, muted })
            .await
    }

    /// Toggles the audio mute state of an input.
    #[doc(alias = "ToggleInputMute")]
    pub async fn toggle_mute(&self, input: InputId<'_>) -> Result<bool> {
        self.client
            .send_message::<_, responses::InputMuted>(Request::ToggleMute { input })
            .await
            .map(|im| im.muted)
    }

    /// Gets the current volume setting of an input.
    #[doc(alias = "GetInputVolume")]
    pub async fn volume(&self, input: InputId<'_>) -> Result<responses::InputVolume> {
        self.client.send_message(Request::Volume { input }).await
    }

    /// Sets the volume setting of an input.
    #[doc(alias = "SetInputVolume")]
    pub async fn set_volume(&self, input: InputId<'_>, volume: Volume) -> Result<()> {
        self.client
            .send_message(Request::SetVolume { input, volume })
            .await
    }

    /// Sets the name of an input (rename).
    #[doc(alias = "SetInputName")]
    pub async fn set_name(&self, input: InputId<'_>, new: &str) -> Result<()> {
        self.client
            .send_message(Request::SetName { input, new })
            .await
    }

    /// Creates a new input, adding it as a scene item to the specified scene.
    #[doc(alias = "CreateInput")]
    pub async fn create<T>(&self, input: Create<'_, T>) -> Result<responses::SceneItemId>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::Create(CreateInputInternal {
                scene: input.scene,
                input: input.input,
                kind: input.kind,
                settings: input
                    .settings
                    .map(|settings| {
                        serde_json::to_value(&settings)
                            .map_err(crate::error::SerializeCustomDataError)
                    })
                    .transpose()?,
                enabled: input.enabled,
            }))
            .await
    }

    /// Removes an existing input.
    ///
    /// **Note:** Will immediately remove all associated scene items.
    #[doc(alias = "RemoveInput")]
    pub async fn remove(&self, input: InputId<'_>) -> Result<()> {
        self.client.send_message(Request::Remove { input }).await
    }

    /// Gets the audio balance of an input.
    #[doc(alias = "GetInputAudioBalance")]
    pub async fn audio_balance(&self, input: InputId<'_>) -> Result<f32> {
        self.client
            .send_message::<_, responses::AudioBalance>(Request::AudioBalance { input })
            .await
            .map(|ab| ab.audio_balance)
    }

    /// Sets the audio balance of an input.
    #[doc(alias = "SetInputAudioBalance")]
    pub async fn set_audio_balance(&self, input: InputId<'_>, balance: f32) -> Result<()> {
        self.client
            .send_message(Request::SetAudioBalance { input, balance })
            .await
    }

    /// Gets the audio sync offset of an input.
    ///
    /// **Note:** The audio sync offset can be negative too!
    #[doc(alias = "GetInputAudioSyncOffset")]
    pub async fn audio_sync_offset(&self, input: InputId<'_>) -> Result<Duration> {
        self.client
            .send_message::<_, responses::AudioSyncOffset>(Request::AudioSyncOffset { input })
            .await
            .map(|aso| aso.input_audio_sync_offset)
    }

    /// Sets the audio sync offset of an input.
    #[doc(alias = "SetInputAudioSyncOffset")]
    pub async fn set_audio_sync_offset(&self, input: InputId<'_>, offset: Duration) -> Result<()> {
        self.client
            .send_message(Request::SetAudioSyncOffset { input, offset })
            .await
    }

    /// Gets the audio monitor type of input.
    #[doc(alias = "GetInputAudioMonitorType")]
    pub async fn audio_monitor_type(&self, input: InputId<'_>) -> Result<MonitorType> {
        self.client
            .send_message::<_, responses::AudioMonitorType>(Request::AudioMonitorType { input })
            .await
            .map(|amt| amt.monitor_type)
    }

    /// Sets the audio monitor type of input.
    #[doc(alias = "SetInputAudioMonitorType")]
    pub async fn set_audio_monitor_type(
        &self,
        input: InputId<'_>,
        monitor_type: MonitorType,
    ) -> Result<()> {
        self.client
            .send_message(Request::SetAudioMonitorType {
                input,
                monitor_type,
            })
            .await
    }

    /// Gets the enable state of all audio tracks of an input.
    #[doc(alias = "GetInputAudioTracks")]
    pub async fn audio_tracks(&self, input: InputId<'_>) -> Result<[bool; 6]> {
        self.client
            .send_message::<_, responses::AudioTracks>(Request::AudioTracks { input })
            .await
            .map(|at| at.audio_tracks)
    }

    /// Sets the enable state of audio tracks of an input.
    #[doc(alias = "SetInputAudioTracks")]
    pub async fn set_audio_tracks(
        &self,
        input: InputId<'_>,
        tracks: [Option<bool>; 6],
    ) -> Result<()> {
        self.client
            .send_message(Request::SetAudioTracks { input, tracks })
            .await
    }

    /// Gets the items of a list property from an input's properties.
    ///
    /// **Note:** Use this in cases where an input provides a dynamic, selectable list of items. For
    /// example, display capture, where it provides a list of available displays.
    #[doc(alias = "GetInputPropertiesListPropertyItems")]
    pub async fn properties_list_property_items(
        &self,
        input: InputId<'_>,
        property: &str,
    ) -> Result<Vec<responses::ListPropertyItem>> {
        self.client
            .send_message::<_, responses::ListPropertyItems>(Request::PropertiesListPropertyItems {
                input,
                property,
            })
            .await
            .map(|lpi| lpi.property_items)
    }

    /// Presses a button in the properties of an input.
    ///
    /// **Note:** Use this in cases where there is a button in the properties of an input that
    /// cannot be accessed in any other way. For example, browser sources, where there is a refresh
    /// button.
    #[doc(alias = "PressInputPropertiesButton")]
    pub async fn press_properties_button(&self, input: InputId<'_>, property: &str) -> Result<()> {
        self.client
            .send_message(Request::PressPropertiesButton { input, property })
            .await
    }
}
