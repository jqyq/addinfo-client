use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    High,
    Low,
    Normal,
    VeryHigh,
    VeryLow,
}
