//! Requests related to profiles.

use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Serialize)]
#[serde(tag = "requestType", content = "requestData")]
pub(crate) enum Request<'a> {
    #[serde(rename = "GetProfileList")]
    List,
    #[serde(rename = "SetCurrentProfile")]
    SetCurrent {
        /// Name of the profile to switch to.
        #[serde(rename = "profileName")]
        name: &'a str,
    },
    #[serde(rename = "CreateProfile")]
    Create {
        /// Name for the new profile.
        #[serde(rename = "profileName")]
        name: &'a str,
    },
    #[serde(rename = "RemoveProfile")]
    Remove {
        /// Name of the profile to remove.
        #[serde(rename = "profileName")]
        name: &'a str,
    },
    #[serde(rename = "GetProfileParameter")]
    Parameter {
        /// Category of the parameter to get.
        #[serde(rename = "parameterCategory")]
        category: &'a str,
        /// Name of the parameter to get.
        #[serde(rename = "parameterName")]
        name: &'a str,
    },
    #[serde(rename = "SetProfileParameter")]
    SetParameter(SetParameter<'a>),
}

impl<'a> From<Request<'a>> for super::RequestType<'a> {
    fn from(value: Request<'a>) -> Self {
        super::RequestType::Profiles(value)
    }
}

/// Request information for [`crate::client::Profiles::set_parameter`].
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[cfg_attr(feature = "builder", derive(bon::Builder))]
pub struct SetParameter<'a> {
    /// Category of the parameter to set.
    #[serde(rename = "parameterCategory")]
    pub category: &'a str,
    /// Name of the parameter to set.
    #[serde(rename = "parameterName")]
    pub name: &'a str,
    /// Value of the parameter to set. Use [`None`] to delete.
    #[serde(rename = "parameterValue")]
    pub value: Option<&'a str>,
}
