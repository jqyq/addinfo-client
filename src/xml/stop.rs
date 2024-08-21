use serde::{Deserialize, Serialize};

use super::{coords::Coords, gids::Gids, interchanges::Interchanges, place::Place};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Stop {
    #[serde(
        rename = "@selected",
        deserialize_with = "crate::xml::string_into_bool_infallible"
    )]
    selected: bool,

    #[serde(rename = "@stopID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    stop_id: Option<u32>,

    #[serde(rename = "@name")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    name: Option<String>,

    #[serde(rename = "@globalID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    global_id: Option<String>,

    #[serde(rename = "@locality")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    locality: Option<String>,

    #[serde(rename = "@naptanID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    naptan_id: Option<String>,

    #[serde(rename = "@depTime")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    dep_time: Option<String>,

    #[serde(rename = "@arrTime")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    arr_time: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    place: Option<Place>,

    coords: Coords,

    interchanges: Interchanges,

    gids: Gids,
}
