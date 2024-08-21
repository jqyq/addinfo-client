use anyhow::{Context, Error};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use super::{itd_date::ItdDate, itd_time::ItdTime};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]

pub struct ItdDateTime {
    #[serde(rename = "@utc")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub utc: Option<DateTime<Utc>>,

    #[serde(rename = "@absolute")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_silent")]
    pub absolute: Option<f32>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub itd_date: Option<ItdDate>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub itd_time: Option<ItdTime>,
}

impl TryInto<NaiveDateTime> for ItdDateTime {
    type Error = Error;

    fn try_into(self) -> Result<NaiveDateTime, Self::Error> {
        let date: NaiveDate = self.itd_date.context("no date")?.try_into()?;
        let time: NaiveTime = self.itd_time.context("no time")?.try_into()?;
        Ok(date.and_time(time))
    }
}
