use serde::{Deserialize, Serialize};

use super::stop::Stop;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Stops {
    pub stop: Vec<Stop>,
}
