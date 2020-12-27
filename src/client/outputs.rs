use anyhow::Result;

use super::Client;
use crate::requests::RequestType;
use crate::responses;

/// API functions related to outputs.
pub struct Outputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Outputs<'a> {
    /// List existing outputs.
    pub async fn list_outputs(&self) -> Result<Vec<responses::Output>> {
        self.client
            .send_message::<responses::Outputs>(RequestType::ListOutputs)
            .await
            .map(|o| o.outputs)
    }

    /// Get information about a single output.
    ///
    /// - `output_name`: Output name.
    pub async fn get_output_info(&self, output_name: String) -> Result<responses::Output> {
        self.client
            .send_message::<responses::OutputInfo>(RequestType::GetOutputInfo { output_name })
            .await
            .map(|o| o.output_info)
    }

    /// Note: Controlling outputs is an experimental feature of obs-websocket. Some plugins which
    /// add outputs to OBS may not function properly when they are controlled in this way.
    ///
    /// - `output_name`: Output name.
    pub async fn start_output(&self, output_name: String) -> Result<()> {
        self.client
            .send_message(RequestType::StartOutput { output_name })
            .await
    }

    /// Note: Controlling outputs is an experimental feature of obs-websocket. Some plugins which
    /// add outputs to OBS may not function properly when they are controlled in this way.
    ///
    /// - `output_name`: Output name.
    /// - `force`: Force stop (default: false).
    pub async fn stop_output(&self, output_name: String, force: Option<bool>) -> Result<()> {
        self.client
            .send_message(RequestType::StopOutput { output_name, force })
            .await
    }
}
