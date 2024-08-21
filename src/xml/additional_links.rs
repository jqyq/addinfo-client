use serde::{Deserialize, Serialize};

use super::additional_link::AdditionalLink;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AdditionalLinks {
    additional_link: Vec<AdditionalLink>,
}
