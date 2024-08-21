use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::stops::Stops;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Trip {
    #[serde(rename = "@tripKey")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    line: Option<String>,

    #[serde(rename = "@trainNumber")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    train_number: Option<String>,

    #[serde(rename = "@realtimeTripId")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    realtime_trip_id: Option<String>,

    #[serde(rename = "@tripTime")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    trip_time: Option<String>,

    #[serde(rename = "@depTime")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    dep_time: Option<DateTime<Utc>>,

    #[serde(rename = "@info")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    info: Option<String>,

    stop_list: Stops,
}
