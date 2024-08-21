use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Param {
    #[serde(
        rename = "@edit",
        deserialize_with = "crate::xml::string_into_bool_infallible"
    )]
    edit: bool,

    #[serde(rename = "type")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    ty: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    value: Option<String>,
}
