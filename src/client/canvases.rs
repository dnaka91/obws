use super::Client;
use crate::{error::Result, requests::canvases::Request, responses::canvases as responses};

/// API functions related to canvases.
pub struct Canvases<'a> {
    pub(super) client: &'a Client,
}

impl Canvases<'_> {
    /// Gets an array of all canvases in OBS.
    #[doc(alias = "GetCanvasList")]
    pub async fn list(&self) -> Result<Vec<responses::Canvas>> {
        self.client
            .send_message::<_, responses::Canvases>(Request::List)
            .await
            .map(|c| c.canvases)
    }
}
