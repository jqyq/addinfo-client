use serde::{Deserialize, Serialize};

use crate::xml::param::Param;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ParamList {
    param: Vec<Param>,
}
