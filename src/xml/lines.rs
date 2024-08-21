use serde::{Deserialize, Serialize};

use super::line::Line;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Lines {
    pub line: Vec<Line>,
}
