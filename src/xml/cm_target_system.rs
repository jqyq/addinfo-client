use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{app::env::Env, HEARTBEAT};

use super::{
    cm_server_type::CmServerType, heartbeat::Heartbeat, message_transfer_mode::MessageTransferMode,
    sending_format::SendingFormat, sending_mode::SendingMode, sync_mode::SyncMode,
    time_mode::TimeMode,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CmTargetSystem {
    #[serde(rename = "@cmServerName")]
    cm_server_name: String,

    #[serde(rename = "@messageTransferMode")]
    message_transfer_mode: MessageTransferMode,

    #[serde(rename = "@cmServerGroup")]
    cm_server_group: String,

    #[serde(rename = "@sendingFormat")]
    sending_format: SendingFormat,

    #[serde(rename = "@timeMode")]
    time_mode: TimeMode,

    #[serde(rename = "@cmServerVirtDir")]
    cm_server_virt_dir: String,

    #[serde(rename = "@cmServerPort")]
    cm_server_port: u16,

    #[serde(rename = "@cmServerType")]
    cm_server_type: CmServerType,

    #[serde(rename = "@sendingMode")]
    sending_mode: SendingMode,

    #[serde(rename = "@syncMode")]
    sync_mode: SyncMode,

    heartbeat: Heartbeat,
}

// By default we extract the relevant information from the environment variables.
impl Default for CmTargetSystem {
    #[instrument(level = "debug")]
    fn default() -> Self {
        Self {
            cm_server_name: Env::get_client_host(),
            message_transfer_mode: MessageTransferMode::Single,
            cm_server_group: Env::get_cm_server_group(),
            sending_format: SendingFormat::Addinfoxml,
            time_mode: TimeMode::Direct,
            cm_server_virt_dir: Env::get_client_path(),
            cm_server_port: Env::get_client_port(),
            cm_server_type: CmServerType::Efa,
            sending_mode: SendingMode::Http,
            sync_mode: SyncMode::DiffByDirAsync,
            heartbeat: Heartbeat {
                die_when_failed: true,
                interval: Env::get_heartbeat_interval(),
                address: format!(
                    "{}://{}:{}/{}/{HEARTBEAT}",
                    Env::get_client_prot(),
                    Env::get_client_host(),
                    Env::get_client_port(),
                    Env::get_client_path(),
                ),
            },
        }
    }
}
