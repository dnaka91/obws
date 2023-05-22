use serde::{de::DeserializeOwned, Serialize};

use super::Client;
use crate::{requests::outputs::Request, responses::outputs as responses, Error, Result};

/// API functions related to outputs.
pub struct Outputs<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Outputs<'a> {
    /// Gets the list of available outputs.
    #[doc(alias = "GetOutputList")]
    pub async fn list(&self) -> Result<Vec<responses::Output>> {
        self.client
            .send_message::<_, responses::OutputList>(Request::List)
            .await
            .map(|ol| ol.outputs)
    }

    /// Gets the status of an output.
    #[doc(alias = "GetOutputStatus")]
    pub async fn status(&self, name: &str) -> Result<responses::OutputStatus> {
        self.client.send_message(Request::Status { name }).await
    }

    /// Toggles the status of an output.
    #[doc(alias = "ToggleOutput")]
    pub async fn toggle(&self, name: &str) -> Result<bool> {
        self.client
            .send_message::<_, responses::OutputActive>(Request::Toggle { name })
            .await
            .map(|oa| oa.active)
    }

    /// Starts an output.
    #[doc(alias = "StartOutput")]
    pub async fn start(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Start { name }).await
    }

    /// Stops an output.
    #[doc(alias = "StopOutput")]
    pub async fn stop(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Stop { name }).await
    }

    /// Gets the settings of an output.
    #[doc(alias = "GetOutputSettings")]
    pub async fn settings<T>(&self, name: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .send_message::<_, responses::OutputSettings<T>>(Request::Settings { name })
            .await
            .map(|os| os.settings)
    }

    /// Sets the settings of an output.
    #[doc(alias = "SetOutputSettings")]
    pub async fn set_settings<T>(&self, name: &str, settings: T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .send_message(Request::SetSettings {
                name,
                settings: serde_json::to_value(&settings).map_err(Error::SerializeCustomData)?,
            })
            .await
    }
}
