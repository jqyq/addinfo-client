use serde::{Deserialize, Serialize};

use super::{itd_additional_travel_information::ItdAdditionalTravelInformation, status::Status};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdAdditionalTravelInformations {
    pub status: Option<Status>,
    pub itd_additional_travel_information: Vec<ItdAdditionalTravelInformation>,
}
