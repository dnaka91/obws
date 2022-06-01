//! Responses related to profiles.

use serde::Deserialize;

/// Response value for [`crate::client::Profiles::list`].
#[derive(Debug, Deserialize)]
pub struct Profiles {
    /// The name of the current profile.
    #[serde(rename = "currentProfileName")]
    pub current: String,
    /// Array of all available profiles.
    pub profiles: Vec<String>,
}

/// Response value for [`crate::client::Profiles::parameter`].
#[derive(Debug, Deserialize)]
pub struct ProfileParameter {
    /// Value associated with the parameter.
    #[serde(rename = "parameterValue")]
    pub value: Option<String>,
    /// Default value associated with the parameter.
    #[serde(rename = "defaultParameterValue")]
    pub default_value: Option<String>,
}
