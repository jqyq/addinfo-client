use serde::{Deserialize, Serialize};

use super::{info_text::InfoText, param_list::ParamList};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct InfoLink {
    #[serde(rename = "@language")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    language: Option<String>,

    #[serde(rename = "@rtStatus")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    rt_status: Option<String>,

    param_list: ParamList,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    info_link_text: Option<String>,

    #[serde(rename = "infoLinkURL")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    info_link_url: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    info_link_image: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    info_text: Option<InfoText>,
}
