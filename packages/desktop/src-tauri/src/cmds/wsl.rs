// ponytail: stub WSL commands — implement when Windows-native tauri-plugin-wsl exists or direct Windows API calls
use serde::Serialize;

#[derive(Serialize)]
pub struct WslState {
    pub servers: Vec<String>,
    pub runtimes: Vec<String>,
}

#[tauri::command]
pub fn wsl_get_state() -> WslState {
    WslState { servers: vec![], runtimes: vec![] }
}

#[tauri::command]
pub fn wsl_probe_runtime() -> Option<String> { None }

#[tauri::command]
pub fn wsl_refresh_distros() -> Vec<String> { vec![] }

#[tauri::command]
pub async fn wsl_install_wsl() -> Result<(), String> {
    Err("WSL not supported on this platform".into())
}

#[tauri::command]
pub async fn wsl_install_distro(_name: String) -> Result<(), String> {
    Err("WSL not supported on this platform".into())
}

#[tauri::command]
pub fn wsl_probe_addable(_distros: Vec<String>) -> Vec<String> { vec![] }

#[tauri::command]
pub async fn wsl_install_opencode(_name: String) -> Result<(), String> {
    Err("WSL not supported on this platform".into())
}

#[tauri::command]
pub fn wsl_open_terminal(_name: String) {}

#[tauri::command]
pub async fn wsl_add_server(_distro: String) -> Result<(), String> {
    Err("WSL not supported on this platform".into())
}

#[tauri::command]
pub async fn wsl_remove_server(_id: String) -> Result<(), String> {
    Err("WSL not supported on this platform".into())
}

#[tauri::command]
pub async fn wsl_start_server(_id: String) -> Result<(), String> {
    Err("WSL not supported on this platform".into())
}
