use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    AreaBlocking,
    AreaInfo,
    BannerInfo,
    Clearance,
    GeneralInfo,
    IndexInfo,
    InterchangeInfo,
    InterchangeDisruption,
    InterchangeMessage,
    LineBlocking,
    LineInfo,
    RouteBlocking,
    RouteDisruption,
    RouteInfo,
    StopBlocking,
    StopDisplacement,
    StopInfo,
    TrafficSituation,
    TripMessage,
}
