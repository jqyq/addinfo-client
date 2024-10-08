use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum SyncMode {
    #[serde(rename = "diffByDir_asynchron")]
    DiffByDirAsync,
}
