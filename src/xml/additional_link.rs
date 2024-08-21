use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AdditionalLink {
    #[serde(rename = "@ID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    id: Option<String>,

    #[serde(rename = "linkURL")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    link_url: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    link_text: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    link_text_short: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    link_target: Option<String>,
}
