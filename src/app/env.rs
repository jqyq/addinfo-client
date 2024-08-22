use dotenv::var;

// Contains all optional environment variables.
// Sensible defaults are used where no value is provided.
pub struct Env {}

impl Env {
    // The client should be reachable via this protocol.
    // There are no plausability checks that identify issues
    // such as using HTTP on port 443 or HTTPS on port 80.
    pub fn get_client_prot() -> String {
        var("CLIENT_PROT").unwrap_or(String::from("http"))
    }

    // The client should be rechable externally via this FQDN or IP address.
    // Your client will likely sit behind a reverse proxy and in that case,
    // this should be the address the reverse proxy will forward traffic to.
    pub fn get_client_host() -> String {
        var("CLIENT_HOST").unwrap_or(String::from("localhost"))
    }

    // Same as above but for the port. No errors are thrown for illegal values.
    // Port 80 will be used instead. This also applies if no port was provided.
    pub fn get_client_port() -> u16 {
        var("CLIENT_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(80)
    }

    // EFA and ICS usually refer to this as the virtual directory.
    // You can choose an arbitrary value. Sync requests to ICS will
    // include this value in the request body such that ICS is aware
    // to which virtual directory it should send requests to. Be
    // careful if you have a reverse proxy that changes URL paths.
    pub fn get_client_path() -> String {
        var("CLIENT_PATH").unwrap_or(String::from("api"))
    }

    // By default the client will listen on all network interfaces on port 80.
    pub fn get_client_bind() -> String {
        var("CLIENT_BIND").unwrap_or(format!("0.0.0.0:80"))
    }

    // Our client acts as if it was an EFA instance.
    pub fn get_system_id() -> String {
        var("SOURCE_SYSTEM_ID").unwrap_or(String::from("EFAJP"))
    }

    // Our client acts as if it was an EFA instance.
    pub fn get_system_name() -> String {
        var("SOURCE_SYSTEM_NAME").unwrap_or(String::from("EFAJP"))
    }

    // Our client acts as if it was an EFA instance.
    pub fn get_system_type() -> String {
        var("SOURCE_SYSTEM_TYPE").unwrap_or(String::from("EFAJP"))
    }

    // This needs to be reconfigured in production. Common values are TEST and LIVE.
    pub fn get_cm_server_group() -> String {
        var("CM_SERVER_GROUP").unwrap_or(String::from("TEST"))
    }

    // The heartbeal interval that the client will broker when contacting ICS.
    pub fn get_heartbeat_interval() -> u32 {
        var("HEARTBEAT_INTERVAL")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(60)
    }

    // The client will try to resync after a given amount of consecutively
    // failed heartbeats. Note that resyncing is likely not going to work
    // if the server side isn't available. But the resync process will
    // happen more quickly once the server side becomes available again.
    pub fn get_resync_after_failed_heartbeats() -> u32 {
        var("RESYNC_AFTER_FAILED_HEARTBEATS")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5)
    }

    // Write debug files to this directory.
    pub fn get_debug_dir() -> String {
        var("DEBUG_DIR").unwrap_or("debug".into())
    }

    // Should we pretty print XML exchanges to disk?
    pub fn get_pretty_print_xml_to_disk() -> bool {
        var("PRETTY_PRINT_XML_TO_DISK").is_ok_and(|s| s.parse().unwrap_or_default())
    }
}
