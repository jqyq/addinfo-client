use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Place {
    #[serde(
        rename = "@selected",
        deserialize_with = "crate::xml::string_into_bool_infallible"
    )]
    selected: bool,

    #[serde(rename = "placeID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    place_id: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    place_name: Option<String>,

    #[serde(rename = "OMC")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    omc: Option<String>,
}
