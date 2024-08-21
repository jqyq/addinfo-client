use serde::{Deserialize, Serialize};

use super::cm_target_system::CmTargetSystem;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CmTargetSystems {
    cm_target_system: Vec<CmTargetSystem>,
}

// We usually only have a single CmTargetSystem
// so we can make use of this helper impl.
impl From<CmTargetSystem> for CmTargetSystems {
    fn from(cm_target_system: CmTargetSystem) -> Self {
        Self {
            cm_target_system: vec![cm_target_system],
        }
    }
}
