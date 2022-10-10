//! Responses related to profiles.

use serde::{Deserialize, Serialize};

/// Response value for [`crate::client::Profiles::list`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Profiles {
    /// The name of the current profile.
    #[serde(rename = "currentProfileName")]
    pub current: String,
    /// Array of all available profiles.
    pub profiles: Vec<String>,
}

/// Response value for [`crate::client::Profiles::parameter`].
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ProfileParameter {
    /// Value associated with the parameter.
    #[serde(rename = "parameterValue")]
    pub value: Option<String>,
    /// Default value associated with the parameter.
    #[serde(rename = "defaultParameterValue")]
    pub default_value: Option<String>,
}
