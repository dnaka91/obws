use serde::{de::DeserializeOwned, Serialize};
use time::Duration;

use super::Client;
use crate::{
    requests::{
        CreateInput, CreateInputInternal, RequestType, SetInputSettings, SetInputSettingsInternal,
        Volume,
    },
    responses, Error, MonitorType, Result,
};

/// API functions related to inputs.
pub struct Inputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Inputs<'a> {
    /// Gets an array of all inputs in OBS.
    ///
    /// - `input_kind`: Restrict the array to only inputs of the specified kind.
    pub async fn get_input_list(&self, input_kind: Option<&str>) -> Result<Vec<responses::Input>> {
        self.client
            .send_message::<responses::Inputs>(RequestType::GetInputList { input_kind })
            .await
            .map(|i| i.inputs)
    }

    /// Gets an array of all available input kinds in OBS.
    ///
    /// - `unversioned`: Return all kinds as unversioned or with version suffixes (if available).
    pub async fn get_input_kind_list(&self, unversioned: bool) -> Result<Vec<String>> {
        self.client
            .send_message::<responses::InputKinds>(RequestType::GetInputKindList { unversioned })
            .await
            .map(|ik| ik.input_kinds)
    }

    /// Gets the default settings for an input kind.
    ///
    /// - `input_kind`: Input kind to get the default settings for.
    pub async fn get_input_default_settings<'de, T>(&self, input_kind: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message::<responses::DefaultInputSettings<T>>(
                RequestType::GetInputDefaultSettings { input_kind },
            )
            .await
            .map(|dis| dis.default_input_settings)
    }

    /// Gets the settings of an input.
    ///
    /// **Note:** Does not include defaults. To create the entire settings object, overlay input
    /// settings over the default input settings provided by [`get_input_default_settings`].
    ///
    /// - `input_name`: Name of the input to get the settings of.
    ///
    /// [`get_input_default_settings`]: Inputs::get_input_default_settings
    pub async fn get_input_settings<T>(
        &self,
        input_name: &str,
    ) -> Result<responses::InputSettings<T>>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message(RequestType::GetInputSettings { input_name })
            .await
    }

    /// Sets the settings of an input.
    pub async fn set_input_settings<T>(&self, settings: SetInputSettings<'_, T>) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(RequestType::SetInputSettings(SetInputSettingsInternal {
                input_name: settings.input_name,
                input_settings: serde_json::to_value(&settings.input_settings)
                    .map_err(Error::SerializeCustomData)?,
                overlay: settings.overlay,
            }))
            .await
    }

    /// Gets the audio mute state of an input.
    ///
    /// - `input_name`: Name of input to get the mute state of.
    pub async fn get_input_mute(&self, input_name: &str) -> Result<bool> {
        self.client
            .send_message::<responses::InputMuted>(RequestType::GetInputMute { input_name })
            .await
            .map(|im| im.input_muted)
    }

    /// Sets the audio mute state of an input..
    ///
    /// - `input_name`: Name of the input to set the mute state of.
    /// - `input_muted`: Whether to mute the input.
    pub async fn set_input_mute(&self, input_name: &str, input_muted: bool) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputMute {
                input_name,
                input_muted,
            })
            .await
    }

    /// Toggles the audio mute state of an input.
    ///
    /// - `input_name`: Name of the input to toggle the mute state of.
    pub async fn toggle_input_mute(&self, input_name: &str) -> Result<bool> {
        self.client
            .send_message::<responses::InputMuted>(RequestType::ToggleInputMute { input_name })
            .await
            .map(|im| im.input_muted)
    }

    /// Gets the current volume setting of an input.
    ///
    /// - `input_name`: Name of the input to get the volume of.
    pub async fn get_input_volume(&self, input_name: &str) -> Result<responses::InputVolume> {
        self.client
            .send_message(RequestType::GetInputVolume { input_name })
            .await
    }

    /// Sets the volume setting of an input.
    ///
    /// - `input_name`: Name of the input to set the volume of.
    /// - `input_volume`: Volume settings in either mul or dB.
    pub async fn set_input_volume(&self, input_name: &str, input_volume: Volume) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputVolume {
                input_name,
                input_volume,
            })
            .await
    }

    /// Sets the name of an input (rename).
    ///
    /// - `input_name`: Current input name.
    /// - `new_input_name`: New name for the input.
    pub async fn set_input_name(&self, input_name: &str, new_input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputName {
                input_name,
                new_input_name,
            })
            .await
    }

    /// Creates a new input, adding it as a scene item to the specified scene.
    pub async fn create_input<T>(&self, input: CreateInput<'_, T>) -> Result<i64>
    where
        T: Serialize,
    {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::CreateInput(CreateInputInternal {
                scene_name: input.scene_name,
                input_name: input.input_name,
                input_kind: input.input_kind,
                input_settings: input
                    .input_settings
                    .map(|settings| {
                        serde_json::to_value(&settings).map_err(Error::SerializeCustomData)
                    })
                    .transpose()?,
                scene_item_enabled: input.scene_item_enabled,
            }))
            .await
            .map(|sii| sii.scene_item_id)
    }

    /// Removes an existing input.
    ///
    /// **Note:** Will immediately remove all associated scene items.
    ///
    /// - `input_name`: Name of the input to remove.
    pub async fn remove_input(&self, input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveInput { input_name })
            .await
    }

    /// Gets the audio sync offset of an input.
    ///
    /// **Note:** The audio sync offset can be negative too!
    ///
    /// - `input_name`: Name of the input to get the audio sync offset of.
    pub async fn get_input_audio_sync_offset(&self, input_name: &str) -> Result<Duration> {
        self.client
            .send_message::<responses::AudioSyncOffset>(RequestType::GetInputAudioSyncOffset {
                input_name,
            })
            .await
            .map(|aso| aso.input_audio_sync_offset)
    }

    /// Sets the audio sync offset of an input.
    ///
    /// - `input_name`: Name of the input to set the audio sync offset of.
    /// - `input_audio_sync_offset`: New audio sync offset in milliseconds.
    pub async fn set_input_audio_sync_offset(
        &self,
        input_name: &str,
        input_audio_sync_offset: Duration,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputAudioSyncOffset {
                input_name,
                input_audio_sync_offset,
            })
            .await
    }

    /// Gets the audio monitor type of input.
    ///
    /// - `input_name`: Name of the input to get the audio monitor type of.
    pub async fn get_input_audio_monitor_type(&self, input_name: &str) -> Result<MonitorType> {
        self.client
            .send_message::<responses::AudioMonitorType>(RequestType::GetInputAudioMonitorType {
                input_name,
            })
            .await
            .map(|amt| amt.monitor_type)
    }

    /// Sets the audio monitor type of input.
    ///
    /// - `input_name`: Name of the input to set the audio monitor type of.
    /// - `monitor_type`: Audio monitor type.
    pub async fn set_input_audio_monitor_type(
        &self,
        input_name: &str,
        monitor_type: MonitorType,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputAudioMonitorType {
                input_name,
                monitor_type,
            })
            .await
    }

    /// Gets the items of a list property from an input's properties.
    ///
    /// **Note:** Use this in cases where an input provides a dynamic, selectable list of items. For
    /// example, display capture, where it provides a list of available displays.
    ///
    /// - `input_name`: Name of the input.
    /// - `property_name`: Name of the list property to get the items of.
    pub async fn get_input_properties_list_property_items(
        &self,
        input_name: &str,
        property_name: &str,
    ) -> Result<Vec<responses::ListPropertyItem>> {
        self.client
            .send_message::<responses::ListPropertyItems>(
                RequestType::GetInputPropertiesListPropertyItems {
                    input_name,
                    property_name,
                },
            )
            .await
            .map(|lpi| lpi.property_items)
    }

    /// Presses a button in the properties of an input.
    ///
    /// **Note:** Use this in cases where there is a button in the properties of an input that
    /// cannot be accessed in any other way. For example, browser sources, where there is a refresh
    /// button.
    ///
    /// - `input_name`: Name of the input.
    /// - `property_name`: Name of the button property to press.
    pub async fn press_input_properties_button(
        &self,
        input_name: &str,
        property_name: &str,
    ) -> Result<()> {
        self.client
            .send_message(RequestType::PressInputPropertiesButton {
                input_name,
                property_name,
            })
            .await
    }
}
