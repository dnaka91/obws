use super::Client;
use crate::{
    requests::{CreateInput, RequestType, SetInputSettings, Volume},
    responses, Result,
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

    pub async fn get_input_default_settings(&self, input_kind: &str) -> Result<serde_json::Value> {
        self.client
            .send_message::<responses::DefaultInputSettings>(RequestType::GetInputDefaultSettings {
                input_kind,
            })
            .await
            .map(|dis| dis.default_input_settings)
    }

    pub async fn get_input_settings(&self, input_name: &str) -> Result<responses::InputSettings> {
        self.client
            .send_message(RequestType::GetInputSettings { input_name })
            .await
    }

    pub async fn set_input_settings(&self, settings: SetInputSettings<'_>) -> Result<()> {
        self.client
            .send_message(RequestType::SetInputSettings(settings))
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

    pub async fn create_input(&self, input: CreateInput<'_>) -> Result<String> {
        self.client
            .send_message::<responses::SceneItemId>(RequestType::CreateInput(input))
            .await
            .map(|sii| sii.scene_item_id)
    }
}
