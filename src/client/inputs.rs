use serde::{de::DeserializeOwned, Serialize};
use time::Duration;

use super::Client;
use crate::{
    common::MonitorType,
    requests::inputs::{
        Create, CreateInputInternal, Request, SetSettings, SetSettingsInternal, Volume,
    },
    responses::inputs as responses,
    Error, Result,
};

/// API functions related to inputs.
pub struct Inputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Inputs<'a> {
    /// Gets an array of all inputs in OBS.
    pub async fn list(&self, kind: Option<&str>) -> Result<Vec<responses::Input>> {
        self.client
            .send_message::<_, responses::Inputs>(Request::List { kind })
            .await
            .map(|i| i.inputs)
    }

    /// Gets an array of all available input kinds in OBS.
    pub async fn list_kinds(&self, unversioned: bool) -> Result<Vec<String>> {
        self.client
            .send_message::<_, responses::InputKinds>(Request::ListKinds { unversioned })
            .await
            .map(|ik| ik.input_kinds)
    }

    /// Gets the names of all special inputs.
    pub async fn specials(&self) -> Result<responses::SpecialInputs> {
        self.client.send_message(Request::Specials).await
    }

    /// Gets the default settings for an input kind.
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
    pub async fn settings<T>(&self, name: &str) -> Result<responses::InputSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client.send_message(Request::Settings { name }).await
    }

    /// Sets the settings of an input.
    pub async fn set_settings<T>(&self, settings: SetSettings<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetSettings(SetSettingsInternal {
                input: settings.input,
                settings: serde_json::to_value(settings.settings)
                    .map_err(Error::SerializeCustomData)?,
                overlay: settings.overlay,
            }))
            .await
    }

    /// Gets the audio mute state of an input.
    pub async fn muted(&self, name: &str) -> Result<bool> {
        self.client
            .send_message::<_, responses::InputMuted>(Request::Muted { name })
            .await
            .map(|im| im.muted)
    }

    /// Sets the audio mute state of an input.
    pub async fn set_muted(&self, name: &str, muted: bool) -> Result<()> {
        self.client
            .send_message(Request::SetMuted { name, muted })
            .await
    }

    /// Toggles the audio mute state of an input.
    pub async fn toggle_mute(&self, name: &str) -> Result<bool> {
        self.client
            .send_message::<_, responses::InputMuted>(Request::ToggleMute { name })
            .await
            .map(|im| im.muted)
    }

    /// Gets the current volume setting of an input.
    pub async fn volume(&self, name: &str) -> Result<responses::InputVolume> {
        self.client.send_message(Request::Volume { name }).await
    }

    /// Sets the volume setting of an input.
    pub async fn set_volume(&self, name: &str, volume: Volume) -> Result<()> {
        self.client
            .send_message(Request::SetVolume { name, volume })
            .await
    }

    /// Sets the name of an input (rename).
    pub async fn set_name(&self, name: &str, new: &str) -> Result<()> {
        self.client
            .send_message(Request::SetName { name, new })
            .await
    }

    /// Creates a new input, adding it as a scene item to the specified scene.
    pub async fn create<T>(&self, input: Create<'_, T>) -> Result<i64>
    where
        T: Serialize,
    {
        self.client
            .send_message::<_, responses::SceneItemId>(Request::Create(CreateInputInternal {
                scene: input.scene,
                input: input.input,
                kind: input.kind,
                settings: input
                    .settings
                    .map(|settings| {
                        serde_json::to_value(&settings).map_err(Error::SerializeCustomData)
                    })
                    .transpose()?,
                enabled: input.enabled,
            }))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Removes an existing input.
    ///
    /// **Note:** Will immediately remove all associated scene items.
    pub async fn remove(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Remove { name }).await
    }

    /// Gets the audio balance of an input.
    pub async fn audio_balance(&self, name: &str) -> Result<f32> {
        self.client
            .send_message::<_, responses::AudioBalance>(Request::AudioBalance { name })
            .await
            .map(|ab| ab.audio_balance)
    }

    /// Sets the audio balance of an input.
    pub async fn set_audio_balance(&self, name: &str, balance: f32) -> Result<()> {
        self.client
            .send_message(Request::SetAudioBalance { name, balance })
            .await
    }

    /// Gets the audio sync offset of an input.
    ///
    /// **Note:** The audio sync offset can be negative too!
    pub async fn audio_sync_offset(&self, name: &str) -> Result<Duration> {
        self.client
            .send_message::<_, responses::AudioSyncOffset>(Request::AudioSyncOffset { name })
            .await
            .map(|aso| aso.input_audio_sync_offset)
    }

    /// Sets the audio sync offset of an input.
    pub async fn set_audio_sync_offset(&self, name: &str, offset: Duration) -> Result<()> {
        self.client
            .send_message(Request::SetAudioSyncOffset { name, offset })
            .await
    }

    /// Gets the audio monitor type of input.
    pub async fn audio_monitor_type(&self, name: &str) -> Result<MonitorType> {
        self.client
            .send_message::<_, responses::AudioMonitorType>(Request::AudioMonitorType { name })
            .await
            .map(|amt| amt.monitor_type)
    }

    /// Sets the audio monitor type of input.
    pub async fn set_audio_monitor_type(
        &self,
        name: &str,
        monitor_type: MonitorType,
    ) -> Result<()> {
        self.client
            .send_message(Request::SetAudioMonitorType { name, monitor_type })
            .await
    }

    /// Gets the enable state of all audio tracks of an input.
    pub async fn audio_tracks(&self, name: &str) -> Result<[bool; 6]> {
        self.client
            .send_message::<_, responses::AudioTracks>(Request::AudioTracks { name })
            .await
            .map(|at| at.audio_tracks)
    }

    /// Sets the enable state of audio tracks of an input.
    pub async fn set_audio_tracks(&self, name: &str, tracks: [Option<bool>; 6]) -> Result<()> {
        self.client
            .send_message(Request::SetAudioTracks { name, tracks })
            .await
    }

    /// Gets the items of a list property from an input's properties.
    ///
    /// **Note:** Use this in cases where an input provides a dynamic, selectable list of items. For
    /// example, display capture, where it provides a list of available displays.
    pub async fn properties_list_property_items(
        &self,
        input: &str,
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
    pub async fn press_properties_button(&self, input: &str, property: &str) -> Result<()> {
        self.client
            .send_message(Request::PressPropertiesButton { input, property })
            .await
    }
}
