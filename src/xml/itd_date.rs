use anyhow::{Context, Error};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// The exact integer types don't really matter so long as they
// cover the range of valid years, months, and days. We use the
// same types as the chrono crate to avoid a conversion step.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdDate {
    #[serde(rename = "@year")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    year: Option<i32>,

    #[serde(rename = "@month")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    month: Option<u32>,

    #[serde(rename = "@day")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    day: Option<u32>,

    // We deserialize the weekday but don't really use it as the
    // chrono crate will infer the weekday from the above values.
    #[serde(rename = "@weekday")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    weekday: Option<i8>,
}

// If any of the three values are missing, we treat this an erroneous date.
impl TryInto<NaiveDate> for ItdDate {
    type Error = Error;

    fn try_into(self) -> Result<NaiveDate, Self::Error> {
        NaiveDate::from_ymd_opt(
            self.year.context("invalid year")?,
            self.month.context("invalid month")?,
            self.day.context("invalid day")?,
        )
        .context("invalid date")
    }
}
