use serde::{Deserialize, Serialize};

use super::interchange_type::InterchangeType;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Interchange {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    area_from: Option<u32>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    area_to: Option<u32>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name_from: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name_to: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    stop_from: Option<u32>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    stop_to: Option<u32>,

    #[serde(rename = "type")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    ty: Option<InterchangeType>,

    #[serde(deserialize_with = "crate::xml::string_into_bool_infallible")]
    blocked: bool,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    duration: Option<u32>,

    #[serde(deserialize_with = "crate::xml::direction_into_bool_infallible")]
    direction: bool,
}
