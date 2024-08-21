use serde::{Deserialize, Serialize};

use super::{
    additional_links::AdditionalLinks, attachments::Attachments, image::Image,
    output_client_text::OutputClientText,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct InfoText {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    content: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    subtitle: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    subject: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    additional_text: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    image: Option<Image>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    output_client_text: Option<OutputClientText>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    additional_links: Option<AdditionalLinks>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    attachments: Option<Attachments>,
}
