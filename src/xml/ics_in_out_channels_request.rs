use serde::{Deserialize, Serialize};

use super::{ics_task::IcsTask, source_system::SourceSystem};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IcsInOutChannelsRequest {
    pub source_system: SourceSystem,
    pub ics_task: IcsTask,
}
