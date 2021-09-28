use chrono::Duration;
use serde::{de::DeserializeOwned, Serialize};

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
    pub async fn get_input_list(&self, input_kind: Option<&str>) -> Result<Vec<responses::Input>> {
        self.client
            .send_message::<responses::Inputs>(RequestType::GetInputList { input_kind })
            .await
            .map(|i| i.inputs)
    }

    pub async fn get_input_kind_list(&self, unversioned: bool) -> Result<Vec<String>> {
        self.client
            .send_message::<responses::InputKinds>(RequestType::GetInputKindList { unversioned })
            .await
            .map(|ik| ik.input_kinds)
    }

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

    pub async fn get_input_mute(&self, input_name: &str) -> Result<bool> {
        self.client
            .send_message::<responses::InputMuted>(RequestType::GetInputMute { input_name })
            .await
            .map(|im| im.input_muted)
    }

    pub async fn set_input_mute(&self, input_name: &str, input_muted: bool) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputMute {
                input_name,
                input_muted,
            })
            .await
    }

    pub async fn toggle_input_mute(&self, input_name: &str) -> Result<bool> {
        self.client
            .send_message::<responses::InputMuted>(RequestType::ToggleInputMute { input_name })
            .await
            .map(|im| im.input_muted)
    }

    pub async fn get_input_volume(&self, input_name: &str) -> Result<responses::InputVolume> {
        self.client
            .send_message(RequestType::GetInputVolume { input_name })
            .await
    }

    pub async fn set_input_volume(&self, input_name: &str, input_volume: Volume) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputVolume {
                input_name,
                input_volume,
            })
            .await
    }

    pub async fn set_input_name(&self, input_name: &str, new_input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputName {
                input_name,
                new_input_name,
            })
            .await
    }

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

    // Currently disabled in obs-websocket and will always fail.
    #[doc(hidden)]
    pub async fn remove_input(&self, input_name: &str) -> Result<()> {
        self.client
            .send_message(RequestType::RemoveInput { input_name })
            .await
    }

    pub async fn get_input_audio_sync_offset(&self, input_name: &str) -> Result<Duration> {
        self.client
            .send_message::<responses::AudioSyncOffset>(RequestType::GetInputAudioSyncOffset {
                input_name,
            })
            .await
            .map(|aso| aso.input_audio_sync_offset)
    }

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

    pub async fn get_input_audio_monitor_type(&self, input_name: &str) -> Result<MonitorType> {
        self.client
            .send_message::<responses::AudioMonitorType>(RequestType::GetInputAudioMonitorType {
                input_name,
            })
            .await
            .map(|amt| amt.monitor_type)
    }

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
