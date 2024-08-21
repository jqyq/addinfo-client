use serde::{Deserialize, Serialize};

use super::{cm_target_systems::CmTargetSystems, sync_mode::SyncMode};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IcsTask {
    #[serde(
        rename = "@addTS",
        serialize_with = "crate::xml::bool_into_i8_infallible"
    )]
    pub add_ts: bool,

    #[serde(rename = "@syncMode")]
    pub sync_mode: SyncMode,

    pub cm_target_systems: CmTargetSystems,
}
