use std::time::Duration;

use tokio::time::sleep;
use tracing::{info, warn};

use super::{client::AddInfoClient, env::Env};

// This is a simple heartbeat thread without any fancy features such as alerting.
pub async fn heartbeat_thread(client: AddInfoClient) {
    let heartbeat_interval = Env::get_heartbeat_interval() as u64;

    loop {
        // A sync with ICS is attempted. This will be retried until successful.
        if client.xml_jpsync_request().await.is_err() {
            sleep(Duration::from_secs(heartbeat_interval)).await;
            continue;
        }

        let mut failed_heartbeats = 0;

        // Once a sync could be established, we won't need to repeat this
        // step until we noticed that the server side isn't available.
        // Hence, we enter another loop that will periodically send out heartbeats.
        loop {
            sleep(Duration::from_secs(heartbeat_interval)).await;

            // We send out a heartbeat to check whether the server side is available.
            failed_heartbeats = match client.xml_heartbeat_request().await {
                Ok(_) => 0,
                Err(_) => failed_heartbeats + 1,
            };

            info!("consecutively failed heartbeats: {failed_heartbeats}");

            // This isn't sophisticated at all. We simply try to resync afer a
            // given amount of consecutively failed heartbeats. You may want to
            // implement some kind of alerting, as this may imply a system outage
            // on the server side. However, it could also be a network issue.
            if failed_heartbeats >= Env::get_resync_after_failed_heartbeats() {
                warn!("resyncing because failed heartbeat limit was reached");
                break;
            }
        }
    }
}
