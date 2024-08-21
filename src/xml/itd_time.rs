use anyhow::{anyhow, Context, Error};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

// The exact integer types don't really matter so long as they
// cover the range of valid hours, minutes, and seconds. We use
// the same types as the chrono crate to avoid a conversion step.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdTime {
    #[serde(rename = "@hour")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    hour: Option<u32>,

    #[serde(rename = "@minute")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    minute: Option<u32>,

    #[serde(rename = "@second")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    second: Option<u32>,
}

// If the hour and/or minute is missing, we treat this as an erroneous time.
// If the second is missing, we take the default value of 0. Note the importance
// of this as ICS will not always supply seconds despite the time being valid.
impl TryInto<NaiveTime> for ItdTime {
    type Error = Error;

    fn try_into(self) -> Result<NaiveTime, Self::Error> {
        NaiveTime::from_hms_opt(
            self.hour.context("invalid hour")?,
            self.minute.context("invalid minute")?,
            self.second.unwrap_or_default(),
        )
        .ok_or(anyhow!("invalid time"))
    }
}
