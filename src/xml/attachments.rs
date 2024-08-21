use serde::{Deserialize, Serialize};

use super::attachment::Attachment;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Attachments {
    attachment: Vec<Attachment>,
}
