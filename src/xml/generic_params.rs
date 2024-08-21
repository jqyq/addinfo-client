use serde::{Deserialize, Serialize};

use super::param_list::ParamList;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct GenericParams {
    param_list: ParamList,
}
