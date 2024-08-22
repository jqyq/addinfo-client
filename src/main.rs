use anyhow::Result;
use app::client::AddInfoClient;
use app::env::Env;
use app::heartbeat_thread;
use app::state::AppState;
use axum::routing::{get, post};
use axum::{serve, Router};

use dotenv::{dotenv, var};
use heartbeat_thread::heartbeat_thread;

use tokio::{fs, main};
use tokio::net::TcpListener;
use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::app::server::{addinfo, heartbeat, soapaction};

mod app;
mod xml;

pub const HEARTBEAT: &str = "HEARTBEAT";
pub const SOAPACTION: &str = "SOAPACTION";
pub const XML_ADDINFO_REQUEST: &str = "XML_ADDINFO_REQUEST";

#[main]
async fn main() -> Result<()> {
    // Ensure we have a .env file.
    dotenv()?;

    // We log very verbosely and may want to make this configurable.
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    // No call via Env::get_* as no default values make sense.
    // We immediately quit if these two environment variable aren't available.
    let ics_base_url = &var("ICS_BASE_URL")?;
    let efa_base_url = &var("EFA_BASE_URL")?;

    // In AddInfo, this often is referred to as the virtual directory.
    // It's the first segment of the URL path, e.g. api.
    let path = Env::get_client_path();

    // If debugging is enabled we need to ensure to first create the debug directory.
    if Env::get_pretty_print_xml_to_disk() {
        fs::create_dir_all(Env::get_debug_dir()).await?;
    }

    // We always start with an empty state as we
    // don't intend to store any messages to disk.
    let state = AppState::default();

    // The client will handle any outgoing API calls, but not incoming ones.
    let client = AddInfoClient::new(ics_base_url, efa_base_url);

    // Given we start with an empty state, we first need to query all the messgaes.
    client.xml_addinfo_request().await?.persist(&state).await;

    // A separate thread will subscribe to ICS and regulary send out heartbeats.
    tokio::spawn(async move {
        heartbeat_thread(client).await;
    });

    // By default we listen on all network interfaces on port 80 for incoming traffic.
    let listener = TcpListener::bind(Env::get_client_bind()).await?;

    // To some extent, the client acts a server in that
    // it offers the following endpoints for ICS to call.
    let app = Router::new()
        .route(&format!("/{path}/{HEARTBEAT}"), post(heartbeat))
        .route(&format!("/{path}/{SOAPACTION}"), post(soapaction))
        .route(&format!("/{path}/{XML_ADDINFO_REQUEST}"), get(addinfo))
        .with_state(state);

    serve(listener, app).await?;

    Ok(())
}
