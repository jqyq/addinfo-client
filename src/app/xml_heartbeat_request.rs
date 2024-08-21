use anyhow::{bail, Result};
use tracing::{info, instrument};

use super::client::AddInfoClient;

pub const XML_HEARTBEAT_REQUEST: &str = "XML_HEARTBEAT_REQUEST";

impl AddInfoClient {
    // We send a heartbeat request to ICS in order to know whether it's available.
    // We conclude ICS is operational if it returns any of the HTTP 2XX status codes.
    #[instrument(level = "debug", skip_all)]
    pub async fn xml_heartbeat_request(&self) -> Result<()> {
        let url = &format!("{}/{}", self.ics_base_url, XML_HEARTBEAT_REQUEST);
        info!("{}", url);

        let response = self.client.get(url).send().await?;
        dbg!(&response);

        if response.status().is_success() {
            Ok(())
        } else {
            bail!("bad response")
        }
    }
}
