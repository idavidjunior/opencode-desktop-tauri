use std::fs;

use crate::state::{AppState, ServerReadyData};
use crate::store::{lock, Store};

fn get_store(state: &AppState, name: &str) -> Store {
    Store::new(state.data_dir.join(name))
}

#[tauri::command]
pub fn kill_sidecar(state: tauri::State<'_, AppState>) {
    let mut guard = lock(&state.inner().sidecar);
    if let Some(mut h) = guard.take() {
        crate::server::stop_sidecar(&mut h);
    }
}

#[tauri::command]
pub async fn await_initialization(
    state: tauri::State<'_, AppState>,
) -> Result<ServerReadyData, String> {
    for _ in 0..300 {
        {
            let guard = lock(&state.inner().init_data);
            if let Some(ref data) = *guard {
                return Ok(data.clone());
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Err("initialization timed out".into())
}

#[tauri::command]
pub fn consume_initial_deep_links(state: tauri::State<'_, AppState>) -> Vec<String> {
    lock(&state.inner().pending_deep_links).drain(..).collect()
}

#[tauri::command]
pub fn get_default_server_url(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    Ok(get_store(state.inner(), "opencode.settings").get("defaultServerUrl"))
}

#[tauri::command]
pub fn set_default_server_url(
    state: tauri::State<'_, AppState>,
    url: Option<String>,
) -> Result<(), String> {
    let store = get_store(state.inner(), "opencode.settings");
    if let Some(u) = url {
        store.set("defaultServerUrl", &u)?;
    } else {
        store.delete("defaultServerUrl")?;
    }
    Ok(())
}

#[tauri::command]
pub fn is_first_launch_onboarding_pending(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let val = get_store(state.inner(), "opencode.settings").get("firstLaunchOnboardingComplete");
    Ok(val != Some("true".into()))
}

#[tauri::command]
pub fn finish_first_launch_onboarding(
    state: tauri::State<'_, AppState>,
    create_default_project: bool,
) -> Result<Option<String>, String> {
    let store = get_store(state.inner(), "opencode.settings");
    let pending = store.get("firstLaunchOnboardingComplete") != Some("true".into());
    if !pending {
        return Ok(None);
    }
    let dir = if create_default_project {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        let path = format!("{}/New OpenCode Project", home);
        let _ = fs::create_dir_all(&path);
        Some(path)
    } else {
        None
    };
    store.set("firstLaunchOnboardingComplete", "true").ok();
    Ok(dir)
}

// ponytail: display backend is Electron-specific (webview choice); Tauri always uses system webview
#[tauri::command]
pub fn get_display_backend() -> Option<String> {
    None
}

#[tauri::command]
pub fn set_display_backend(_backend: Option<String>) {}

// ponytail: markdown parsed by `marked` in Electron main — Tauri delegates to renderer-side parsing
#[tauri::command]
pub fn parse_markdown(markdown: String) -> String {
    markdown
}

#[tauri::command]
pub fn check_app_exists(_app_name: String) -> bool {
    true
}

#[tauri::command]
pub fn resolve_app_path(app_name: String) -> Option<String> {
    Some(app_name)
}

#[tauri::command]
pub fn set_background_color(_color: String) {}

// ponytail: export ZIP of log files — stub until tauri-plugin-log or custom log rolling is wired
#[tauri::command]
pub fn export_debug_logs() -> String {
    "not implemented".into()
}

#[tauri::command]
pub fn record_fatal_renderer_error(error: serde_json::Value) {
    eprintln!("[renderer error] {}", error);
}

#[tauri::command]
pub fn relaunch(app: tauri::AppHandle) {
    app.request_restart();
}

// ponytail: CLI install copies opencode binary from resources — stub until resource bundling is wired
#[tauri::command]
pub fn install_cli() -> String {
    "not implemented".into()
}
