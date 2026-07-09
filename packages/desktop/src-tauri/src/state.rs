use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerReadyData {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct SidecarHandle {
    pub child: std::process::Child,
}

pub struct AppState {
    pub sidecar: Mutex<Option<SidecarHandle>>,
    pub data_dir: PathBuf,
    pub init_data: Mutex<Option<ServerReadyData>>,
    pub pending_deep_links: Mutex<Vec<String>>,
    pub server_port: Mutex<Option<u16>>,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            sidecar: Mutex::new(None),
            data_dir,
            init_data: Mutex::new(None),
            pending_deep_links: Mutex::new(Vec::new()),
            server_port: Mutex::new(None),
        }
    }
}
