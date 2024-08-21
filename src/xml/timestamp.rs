use serde::{Deserialize, Serialize};

use super::itd_date_time::ItdDateTime;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timestamp {
    itd_date_time: ItdDateTime,
}
