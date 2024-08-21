use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct OutputClientText {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    html_text: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    sms_text: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    speech_text: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    wml_text: Option<String>,
}
