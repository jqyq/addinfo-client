use serde::{Deserialize, Serialize};

// You may want to use f64 for millimeter precision when using WGS84
// as the coord_system above. However, there will probably be no
// benefit in that as the source data is unlikely to have that much
// accuracy. After all, it makes no sense to even have that precision
// for locations of stops, as these are more of an area than a point.
// If you don't intend to use WGS84, you may want to change the types.
// Technically, this struct should be able to handle any coordinate
// system whatsoever, but reastically you're only going to use one.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Coord {
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    coord_system: Option<String>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    longitude: Option<f32>,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    latitude: Option<f32>,
}
