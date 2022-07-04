//! Requests related to outputs.

use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetOutputList")]
    List,
    #[serde(rename = "GetOutputStatus")]
    Status {
        /// Output name.
        #[serde(rename = "outputName")]
        name: &'a str,
    },
    #[serde(rename = "ToggleOutput")]
    Toggle {
        /// Output name.
        #[serde(rename = "outputName")]
        name: &'a str,
    },
    #[serde(rename = "StartOutput")]
    Start {
        /// Output name.
        #[serde(rename = "outputName")]
        name: &'a str,
    },
    #[serde(rename = "StopOutput")]
    Stop {
        /// Output name.
        #[serde(rename = "outputName")]
        name: &'a str,
    },
    #[serde(rename = "GetOutputSettings")]
    Settings {
        /// Output name.
        #[serde(rename = "outputName")]
        name: &'a str,
    },
    #[serde(rename = "SetOutputSettings")]
    SetSettings {
        /// Output name.
        #[serde(rename = "outputName")]
        name: &'a str,
        /// Output settings.
        #[serde(rename = "outputSettings")]
        settings: serde_json::Value,
    },
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Outputs(value)
    }
}
