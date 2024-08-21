use serde::{Deserialize, Serialize};

use super::{
    itd_additional_travel_informations::ItdAdditionalTravelInformations, lines::Lines,
    stops::Stops, timestamp::Timestamp,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdAddInfoRequest {
    #[serde(rename = "@requestID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub request_id: Option<String>,

    #[serde(rename = "timeStamp")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub timestamp: Option<Timestamp>,

    pub itd_additional_travel_informations: Vec<ItdAdditionalTravelInformations>,

    pub itd_unique_line_list: Lines,

    pub itd_unique_stop_list: Stops,
}
