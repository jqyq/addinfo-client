use serde::{Deserialize, Serialize};

use super::coord::Coord;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Coords {
    coord: Vec<Coord>,
}
