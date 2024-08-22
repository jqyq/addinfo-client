use serde::{Deserialize, Deserializer, Serialize};

use super::{itd_operator::ItdOperator, itd_train::ItdTrain, trips::Trips};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Line {
    #[serde(rename = "@line")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    line: Option<String>,

    #[serde(rename = "@number")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    number: Option<String>,

    #[serde(rename = "@name")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name: Option<String>,

    #[serde(rename = "@supplement")]
    #[serde(deserialize_with = "maybe_deserialize_supplement")]
    supplement: Option<char>,

    #[serde(rename = "@direction")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    direction: Option<char>,

    #[serde(rename = "@directionName")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    direction_name: Option<String>,

    #[serde(rename = "@partialNet")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    partial_net: Option<String>,

    #[serde(rename = "@project")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    project: Option<String>,

    #[serde(rename = "@motType")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    mot_type: Option<u32>,

    #[serde(rename = "@motCode")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    mot_code: Option<u32>,

    #[serde(rename = "@productId")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    product_id: Option<u32>,

    #[serde(rename = "@mtSubcode")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    mt_subcode: Option<u32>,

    #[serde(rename = "@routeDescText")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    route_desc_text: Option<String>,

    trips: Trips,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    itd_operator: Option<ItdOperator>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    itd_train: Option<ItdTrain>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    direction_text: Option<String>,
}

// A space character indicates a line with no supplement.
fn maybe_deserialize_supplement<'de, D>(d: D) -> Result<Option<char>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d).ok().and_then(|s| match s.as_str() {
        " " => None,
        s => Some(s.chars().next()?),
    }))
}
