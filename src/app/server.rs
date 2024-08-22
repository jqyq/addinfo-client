use std::{fs::File, path::Path};

use anyhow::Result;
use axum::{
    body::Bytes,
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::Local;
use itertools::Itertools;
use quick_xml::se::Serializer;
use serde::Serialize;
use tracing::{error, info, instrument};
use xmltree::{Element, EmitterConfig};

use crate::app::error::AppError;
use crate::xml::{
    heartbeat_notification::HeartbeatNotification,
    itd_request::{ItdRequestCurrentMessages, ItdRequestDelivery},
};

use super::{client::AddInfoClient, env::Env, state::AppState};

// Root element name for XML serialization of the request body.
pub const ITD_REQUEST: &str = "itdRequest";

// Successful responses (HTTP 200 OK) by default should send out the following HTML body.
pub const REPLY_OK: Html<&str> = Html("<Envelop><Header></Header><Body>ok</Body></Envelop>");

// We don't really have to add this to our XML bodies but it seems reasonable to do so.
pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

// Handle incoming heartbeats by replying with the default envelope message.
// Deserializing the request body actually isn't necessary given we make no use of it.
// However, you may want to check the information and update the heartbeat interval
// based on what's in the request body. Also, you could ensure that the provided
// timestamp is recent, i.e. no older than the current time minus the heartbeat interval.
#[instrument(level = "debug", skip_all)]
pub async fn heartbeat(
    State(_): State<AppState>,
    bytes: Bytes,
) -> Result<impl IntoResponse, AppError> {
    match AddInfoClient::deserialize::<HeartbeatNotification>(&bytes).await {
        Ok(_) => {
            info!("heartbeat received");
            Ok(REPLY_OK)
        }
        Err(e) => {
            error!("failed to deserialize heartbeat: {e}");
            Err(e.into())
        }
    }
}

// We need to offer an endpoint that accepts new messages or ones we've missed.
// Note, this also includes so called "clearances", i.e. the removal of a message.
// Incoming request bodies may be of different forms. New messages may come in one
// at a time or as a chunk. By default, ICS will send no more than 100 messages in
// one chunk. You're likely to only receive chunks of messages if you've been down
// for a long time. Server-side configuration may influence what you receive.
#[instrument(level = "debug", skip_all)]
pub async fn soapaction(
    State(state): State<AppState>,
    bytes: Bytes,
) -> Result<impl IntoResponse, AppError> {
    if let Err(e) = pretty_print_xml_to_disk(bytes.as_ref()) {
        error!("failed to pretty print xml to disk: {e}");
    }

    match AddInfoClient::deserialize::<ItdRequestDelivery>(&bytes).await {
        Ok(delivery) => {
            delivery.persist(&state).await;
            Ok(REPLY_OK)
        }
        Err(e) => {
            error!("failed to deserialize: {e}");
            Err(e.into())
        }
    }
}

// For debugging purposes we pretty print the incoming messages to disk.
fn pretty_print_xml_to_disk(bytes: &[u8]) -> Result<()> {
    // Make this configurable via an environment variable.
    if Env::get_pretty_print_xml_to_disk() {
        // Use chronological file names. We might want to improve file naming.
        let now = Local::now().format("%y%m%d_%H%M%S");
        let path = Path::new(&Env::get_debug_dir()).join(format!("{}.xml", now));
        let mut file = File::create(path)?;
        let config = EmitterConfig::new().perform_indent(true);

        // Pretty print XML with indentation.
        Element::parse(bytes.as_ref())?.write_with_config(&mut file, config)?;
    }

    Ok(())
}

// We need to implement the XML_ADDINFO_REQUEST of EFA in order to let ICS know
// the messages that we currently hold. This way, ICS can follow up with any
// messages we may have missed due to a downtime or intermittent network issues.
// We need not implement the entire response structure and will only return the
// objects and fields that are absolutely necessary for the communication to work.
// A sophisticated implementation of this endpoint would only return the current
// messages if the GET parameter command=cmdGetCurrentMessageList is passed.
// Given this is the only type of request that ICS will send, we didn't bother
// implementing this specific behavior, but you may want to do so for consistency.
#[instrument(level = "debug", skip_all)]
pub async fn addinfo(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let rlock = state.messages.read().await;

    let current_messages = rlock.iter().map(|(k, v)| format!("{k}${v}")).join(";");
    info!("found {} current messages", current_messages.len());
    let response = ItdRequestCurrentMessages::from(current_messages);

    let mut buffer = String::from(XML_HEADER);
    let serializer = Serializer::with_root(&mut buffer, Some(ITD_REQUEST))?;
    response.serialize(serializer)?;

    info!("ok");
    Ok(buffer)
}
