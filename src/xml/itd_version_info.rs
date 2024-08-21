use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdVersionInfo {
    pub pt_kernel: Option<PtKernel>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PtKernel {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub app_version: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub data_format: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub data_build: Option<NaiveDateTime>,
}
