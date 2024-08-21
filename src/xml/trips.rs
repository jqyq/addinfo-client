use serde::{Deserialize, Serialize};

use super::trip::Trip;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Trips {
    trip: Vec<Trip>,
}
