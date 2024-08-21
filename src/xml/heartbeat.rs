use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    #[serde(
        rename = "@dieWhenFailed",
        serialize_with = "crate::xml::bool_into_i8_infallible"
    )]
    pub die_when_failed: bool,

    pub interval: u32,

    pub address: String,
}
