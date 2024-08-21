use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeartbeatNotification {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    producer_ref: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    heartbeat_interval_in_sec: Option<u32>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    service_started_time: Option<DateTime<Utc>>,
}
