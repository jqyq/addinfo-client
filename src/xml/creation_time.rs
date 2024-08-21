use serde::{Deserialize, Serialize};

use super::chrono_naive_date_time::ChronoNaiveDateTime;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CreationTime {
    itd_date_time: ChronoNaiveDateTime,
}
