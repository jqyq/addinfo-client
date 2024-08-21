use serde::{Deserialize, Serialize};

use super::interchange::Interchange;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Interchanges {
    coord: Vec<Interchange>,
}
