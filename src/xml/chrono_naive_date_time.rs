use anyhow::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::xml::itd_date_time::ItdDateTime;

// This is a helper in order to deserialize and itdDateTime into a more
// suitable type. We use the chrono crate to handle dates and times.
// EFA and ICS usually don't offer timezone information and for this
// reason we use chrono's NaiveDateTime to store timestamps. Where
// a timezone is supplied, this will be provided as an ISO 8601
// timestamp anyway and so we don't need this helper type in that case.
// Instead, we directly use chrono's DateTime<Utc> in such cases.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, try_from = "ItdDateTime")]
pub struct ChronoNaiveDateTime {
    // itdDateTime objects may come with some or all values set to -1
    // in order to indicate there is no actual date or time information
    // available. We handle this by deserializing into a `None` value.
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    inner: Option<NaiveDateTime>,
}

// We try to convert an itdDateTime in XML into chrono's NaiveDateTime.
// We simply call the TryInto<ChronoNaiveDateTime> impl of ItdDateTime.
impl TryFrom<ItdDateTime> for ChronoNaiveDateTime {
    type Error = Error;

    fn try_from(itd_date_time: ItdDateTime) -> Result<Self, Self::Error> {
        Ok(ChronoNaiveDateTime {
            inner: itd_date_time.try_into().ok(),
        })
    }
}
