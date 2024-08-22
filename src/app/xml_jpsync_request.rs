use anyhow::{bail, Result};
use quick_xml::se::Serializer;
use serde::Serialize;
use tracing::{info, instrument};

use super::client::AddInfoClient;

use crate::{
    app::server::XML_HEADER,
    xml::{
        cm_target_system::CmTargetSystem, cm_target_systems::CmTargetSystems,
        ics_in_out_channels_request::IcsInOutChannelsRequest, ics_task::IcsTask,
        source_system::SourceSystem, sync_mode::SyncMode,
    },
};

impl AddInfoClient {
    // We start the message sync process by setting up a durable subscription within ICS.
    // We construct the request body the same way an EFA instance would. The default impl of
    // `CmTargetSystem` will prefill the heartbeat callback URL from the given environment
    // variables. Note that this implementation will only allow to set the URL up to the
    // virtual directory, i.e. https://client.example.com/api, but not the specific endpoint
    // name, which will always be HEARTBEAT. With that we obtain the following example URL:
    // https://client.example.com/api/HEARTBEAT
    // Note that customizing the SOAPACTION endpoint name won't be possible, which is why we
    // didn't make the HEARTBEAT endpoint name configurable, but you may choose to do so.
    #[instrument(level = "debug", skip_all)]
    pub async fn xml_jpsync_request(&self) -> Result<()> {
        let url = &format!("{}/{}", self.ics_base_url, "XML_JPSYNC_REQUEST");
        info!("{}", &url);

        // Construct a default request body.
        let body = IcsInOutChannelsRequest {
            source_system: SourceSystem::default(),
            ics_task: IcsTask {
                add_ts: true,
                sync_mode: SyncMode::DiffByDirAsync,
                cm_target_systems: CmTargetSystems::from(CmTargetSystem::default()),
            },
        };
        info!("{:?}", &body);

        let mut buffer = String::from(XML_HEADER);
        let serializer = Serializer::with_root(&mut buffer, Some("icsInOutChannelsRequest"))?;
        body.serialize(serializer)?;

        // Request a durable subscription be set up on the server side as an InOutChannel.
        let response = self
            .client
            .post(url)
            .header("content-type", "text/xml")
            .body(buffer)
            .send()
            .await?;

        // We assume the subscription was set up successfully if a HTTP 2XX status code is returned.
        if response.status().is_success() {
            info!("subscription is now active");
            Ok(())
        } else {
            bail!("bad response")
        }
    }
}
