use serde::{Deserialize, Serialize};

use super::itd_banner_info_list::ItdBannerInfoList;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdInfoLinkList {
    pub itd_banner_info_list: ItdBannerInfoList,
}
