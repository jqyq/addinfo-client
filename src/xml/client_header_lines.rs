use serde::{Deserialize, Serialize};

use super::header_line::HeaderLine;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ClientHeaderLines {
    header_line: Vec<HeaderLine>,
}
