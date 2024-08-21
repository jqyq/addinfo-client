use serde::{Deserialize, Serialize};

use crate::app::env::Env;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SourceSystem {
    #[serde(rename = "systemID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub system_id: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub system_name: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub system_type: Option<String>,
}

impl Default for SourceSystem {
    fn default() -> Self {
        Self {
            system_id: Some(Env::get_system_id()),
            system_name: Some(Env::get_system_name()),
            system_type: Some(Env::get_system_type()),
        }
    }
}
