use super::Client;
use crate::{
    error::Result,
    requests::profiles::{Request, SetParameter},
    responses::profiles as responses,
};

/// API functions related to profiles.
pub struct Profiles<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Profiles<'a> {
    /// Gets an array of all profiles.
    #[doc(alias = "GetProfileList")]
    pub async fn list(&self) -> Result<responses::Profiles> {
        self.client.send_message(Request::List).await
    }

    /// Get the currently active profile name.
    #[doc(alias = "GetProfileList")]
    pub async fn current(&self) -> Result<String> {
        self.client
            .send_message::<_, responses::Profiles>(Request::List)
            .await
            .map(|p| p.current)
    }

    /// Switches to a profile.
    #[doc(alias = "SetCurrentProfile")]
    pub async fn set_current(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::SetCurrent { name }).await
    }

    /// Creates a new profile, switching to it in the process.
    #[doc(alias = "CreateProfile")]
    pub async fn create(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Create { name }).await
    }

    /// Removes a profile. If the current profile is chosen, it will change to a different profile
    /// first.
    #[doc(alias = "RemoveProfile")]
    pub async fn remove(&self, name: &str) -> Result<()> {
        self.client.send_message(Request::Remove { name }).await
    }

    /// Gets a parameter from the current profile's configuration.
    #[doc(alias = "GetProfileParameter")]
    pub async fn parameter(
        &self,
        category: &str,
        name: &str,
    ) -> Result<responses::ProfileParameter> {
        self.client
            .send_message(Request::Parameter { category, name })
            .await
    }

    /// Sets the value of a parameter in the current profile's configuration.
    #[doc(alias = "SetProfileParameter")]
    pub async fn set_parameter(&self, parameter: SetParameter<'_>) -> Result<()> {
        self.client
            .send_message(Request::SetParameter(parameter))
            .await
    }
}
