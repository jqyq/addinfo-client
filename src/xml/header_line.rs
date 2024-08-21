use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct HeaderLine {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    value: Option<String>,
}
