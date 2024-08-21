use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "@ID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    id: Option<String>,

    #[serde(rename = "@type")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    ty: Option<String>,

    #[serde(rename = "@path")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    path: Option<String>,

    #[serde(rename = "@fileName")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    file_name: Option<String>,

    #[serde(rename = "@virtPath")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    virt_path: Option<String>,

    #[serde(rename = "@linkText")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    link_text: Option<String>,

    #[serde(rename = "@linkTarget")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    link_target: Option<String>,

    #[serde(rename = "@size")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    size: Option<String>,
}
