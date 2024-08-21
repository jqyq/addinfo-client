use anyhow::Result;
use tracing::{info, instrument};

use super::client::AddInfoClient;
use crate::xml::itd_request::ItdRequest;

pub const XML_ADDINFO_REQUEST: &str = "XML_ADDINFO_REQUEST";

impl AddInfoClient {
    // When the client starts for the first time or after a long downtime, it will
    // have to retrieve all available messages from and EFA server tied to the ICS
    // instance that's configured. ICS will pass its messages on to a local EFA
    // instance. This may be the same machine or a separate one with SMB access.
    // We deserialize the XML response into the ItdRequest type, from which we can
    // extract the relevant information we need. This minimal implementation only
    // extracts what's absolutely necessary, namely the seqID and infoID of all the
    // messages. One more required field is the type of message in order to conclude
    // whether a message is a so called "clearance", i.e. the removal of a message.
    #[instrument(level = "debug", skip_all)]
    pub async fn xml_addinfo_request(&self) -> Result<ItdRequest> {
        let url = &format!("{}/{}", self.efa_base_url, XML_ADDINFO_REQUEST);
        info!("{}", &url);

        let mut response = self.client.get(url).send().await?;
        let bytes = Self::as_bytes(&mut response).await?;
        let deserialized = Self::deserialize(&bytes).await?;

        Ok(deserialized)
    }
}
