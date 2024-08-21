use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Image {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    path: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    size: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    base_64_data: Option<String>,
}
