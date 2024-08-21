use serde::{Deserialize, Serialize};

use super::info_link::InfoLink;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdBannerInfoList {
    pub info_link: Vec<InfoLink>,
}
