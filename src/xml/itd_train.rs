use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdTrain {
    #[serde(rename = "type")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    ty: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name: Option<String>,
}
