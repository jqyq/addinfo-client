use serde::{Deserialize, Serialize};

use super::gid::Gid;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Gids {
    gid: Vec<Gid>,
}
