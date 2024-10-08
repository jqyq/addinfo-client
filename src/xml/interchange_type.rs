use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterchangeType {
    Elevator,
    Escalators,
    Illuminated,
    Level,
    Ramp,
    Stairs,
}
