use super::Client;
use crate::requests::RequestType;
use crate::responses;
use crate::Result;

/// API functions related to profiles.
pub struct Profiles<'a> {
    pub(super) client: &'a Client,
}

impl<'a> Profiles<'a> {
    /// Set the currently active profile.
    ///
    /// - `profile_name`: Name of the desired profile.
    pub async fn set_current_profile(&self, profile_name: String) -> Result<()> {
        self.client
            .send_message(RequestType::SetCurrentProfile { profile_name })
            .await
    }

    /// Get the name of the current profile.
    pub async fn get_current_profile(&self) -> Result<String> {
        self.client
            .send_message::<responses::CurrentProfile>(RequestType::GetCurrentProfile)
            .await
            .map(|cp| cp.profile_name)
    }

    /// Get a list of available profiles.
    pub async fn list_profiles(&self) -> Result<Vec<responses::Profile>> {
        self.client
            .send_message::<responses::Profiles>(RequestType::ListProfiles)
            .await
            .map(|cp| cp.profiles)
    }
}
