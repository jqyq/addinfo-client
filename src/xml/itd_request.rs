use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::state::AppState;

use super::{
    client_header_lines::ClientHeaderLines, itd_add_info_request::ItdAddInfoRequest,
    itd_additional_travel_information::ItdAdditionalTravelInformation,
    itd_additional_travel_informations::ItdAdditionalTravelInformations,
    itd_info_link_list::ItdInfoLinkList, itd_version_info::ItdVersionInfo,
    message_type::MessageType, timestamp::Timestamp,
};

// Note that we have three different types of itdRequest in this file.
// Whilst it might be sensible to properly model this as one struct,
// introducing this complexity is likely not worth the effort. We aren't
// an EFA system and only need these three types of itdRequest that are
// far from similar to each other in terms of which fields are present.

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdRequest {
    #[serde(rename = "@version")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub version: Option<String>,

    #[serde(rename = "@language")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub language: Option<String>,

    #[serde(rename = "@lengthUnit")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub length_unit: Option<String>,

    #[serde(rename = "@sessionID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub sesion_id: Option<String>,

    #[serde(rename = "@client")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub client: Option<String>,

    #[serde(rename = "@clientIP")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub client_ip: Option<String>,

    #[serde(rename = "@serverID")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub server_id: Option<String>,

    #[serde(rename = "@virtDir")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub virt_dir: Option<String>,

    #[serde(rename = "@calcTime")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub calc_time: Option<f32>,

    #[serde(rename = "@now")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub now: Option<NaiveDateTime>,

    #[serde(rename = "@nowWD")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub now_wd: Option<u8>,

    #[serde(rename = "@logRequestId")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub log_request_id: Option<String>,

    pub client_header_lines: ClientHeaderLines,

    pub itd_info_link_list: ItdInfoLinkList,

    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub itd_version_info: Option<ItdVersionInfo>,

    pub itd_add_info_request: ItdAddInfoRequest,
}

impl ItdRequest {
    // We store the infoID and seqID of incoming messages in memory but not disk.
    pub async fn persist(self, state: &AppState) {
        // Note the call to flatten, i.e. we process both "current" and "history" messages.
        // Also note that we skip messages, which do not have an infoID or seqID set.
        let messages = self
            .itd_add_info_request
            .itd_additional_travel_informations
            .into_iter()
            .map(|v| v.itd_additional_travel_information)
            .flatten()
            .filter_map(|m| Some((m.info_id?, m.seq_id?)));

        // We need write access to the messages we hold.
        let mut wlock = state.messages.write().await;

        // We simply overwrite the seqID without checking if the new one is actually higher
        // than the current one we have. We rely on a proper server side implementation.
        for (info_id, seq_id) in messages {
            info!("got msg {} seq {}", &info_id, &seq_id);
            wlock.insert(info_id, seq_id);
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ItdRequestDelivery {
    #[serde(rename = "timeStamp")]
    #[serde(deserialize_with = "crate::xml::try_deserialize_verbose")]
    pub timestamp: Option<Timestamp>,

    // For a delivery of a single message.
    pub itd_additional_travel_information: Option<ItdAdditionalTravelInformation>,

    // For a delivery of a chunk of messages.
    pub itd_additional_travel_informations: Option<ItdAdditionalTravelInformations>,
}

impl ItdRequestDelivery {
    // We only store the infoID and seqID of messages in memory but not to disk.
    pub async fn persist(self, state: &AppState) {
        // We usually receive a delivery with a single message.
        let single = self.itd_additional_travel_information.into_iter();

        // We may receive a chunk of messages in one delivery, e.g. after recovering from downtime.
        let chunk = self
            .itd_additional_travel_informations
            .into_iter()
            .flat_map(|v| v.itd_additional_travel_information);

        // We need write access to the messages we hold.
        let mut wlock = state.messages.write().await;

        // We loop through all the messages that have a type, infoID, and seqID.
        // It might be worth to log any messages that don't fulfil these requirements.
        single
            .chain(chunk)
            .filter_map(|m| Some((m.ty?, m.info_id?, m.seq_id?)))
            .for_each(move |(ty, id, seq)| {
                if ty == MessageType::Clearance {
                    // A clearance indicates a message was removed.
                    info!("got clr {} seq {}", &id, &seq);
                    wlock.remove(&id);
                } else {
                    // Any other message we treate as a new one. Please note that
                    // we blindly insert the message without checking the provided
                    // seqID is actually higher than the one we currently have. A
                    // sophisticated implementation should do this check of course.
                    info!("got msg {} seq {}", &id, &seq);
                    wlock.insert(id, seq);
                }
            });
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItdRequestCurrentMessages {
    current_messages: String,
}

impl<S> From<S> for ItdRequestCurrentMessages
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        Self {
            current_messages: s.as_ref().into(),
        }
    }
}
