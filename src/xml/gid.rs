use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Gid {
    #[serde(
        rename = "@selected",
        default,
        deserialize_with = "crate::xml::string_into_bool_infallible"
    )]
    selected: bool,

    #[serde(rename = "@name")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name: Option<String>,

    #[serde(rename = "$value")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    value: Option<String>,
}
