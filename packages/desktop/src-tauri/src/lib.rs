mod cmds;
mod logging;
mod server;
mod state;
mod store;

use state::{AppState, ServerReadyData, SidecarHandle};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::{Listener, Manager};

fn server_dist_dir(app: &tauri::AppHandle) -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.parent().and_then(|p| p.parent()).and_then(|p| p.parent()).unwrap_or(Path::new("."));
    let dev = root.join("packages").join("opencode").join("dist").join("node");
    if dev.join("node.js").exists() && dev.join("sidecar-entry.mjs").exists() {
        return dev;
    }
    if let Ok(dir) = app.path().resource_dir() {
        let bundled = dir.join("node-dist");
        if bundled.join("node.js").exists() {
            return bundled;
        }
    }
    dev
}

fn node_modules_path() -> PathBuf {
    // ponytail: dev-only path — in production, bundle deps as resources
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.parent().and_then(|p| p.parent()).and_then(|p| p.parent()).unwrap_or(Path::new("."));
    root.join("packages").join("desktop").join("node_modules")
}

fn generate_password() -> String {
    let mut buf = [0u8; 16];
    getrandom::getrandom(&mut buf).expect("rng should never fail");
    format!("opencode-{}", hex_encode(&buf))
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| PathBuf::from("."));
            std::fs::create_dir_all(&data_dir).ok();

            let app_state = AppState::new(data_dir);
            app.manage(app_state);

            let window = app.get_webview_window("main").ok_or("no main window")?;

            let shim = include_str!("../api-shim.js");
            window.eval(shim).ok();

            // Register deep link handler
            let handle = app.handle().clone();
            app.listen("deep-link://new-url", move |event| {
                let state = handle.state::<AppState>();
                let mut links = store::lock(&state.pending_deep_links);
                if let Ok(urls) =
                    serde_json::from_str::<Vec<String>>(event.payload())
                {
                    for url in urls {
                        links.push(url);
                    }
                }
            });

            // Start sidecar
            let stderr_log: Arc<Mutex<Vec<String>>> =
                Arc::new(Mutex::new(Vec::new()));
            {
                let state = app.state::<AppState>();
                if let Ok(port) = server::find_free_port() {
                    let password = generate_password();
                    let server_dir = server_dist_dir(app.handle());
                    let server_dir_str = server_dir.to_string_lossy().to_string();
                    let node_mods_str = node_modules_path().to_string_lossy().to_string();
                    let data_dir_str = state.data_dir.to_string_lossy().to_string();

                    match server::spawn_sidecar(
                        port,
                        &password,
                        &server_dir_str,
                        &node_mods_str,
                        &data_dir_str,
                        &stderr_log,
                    ) {
                        Ok(child) => {
                            *store::lock(&state.server_port) = Some(port);
                            *store::lock(&state.sidecar) = Some(SidecarHandle { child });

                            let host = "127.0.0.1";
                            let ready = (0..200).any(|_| {
                                std::thread::sleep(
                                    std::time::Duration::from_millis(100),
                                );
                                server::port_ready(host, port)
                            });

                            if ready {
                                let url = format!("http://{}:{}", host, port);
                                *store::lock(&state.init_data) = Some(
                                    ServerReadyData {
                                        url,
                                        username: Some("opencode".into()),
                                        password: Some(password),
                                    },
                                );
                            } else {
                                eprintln!("sidecar failed to become ready");
                                let log = store::lock(&stderr_log);
                                if !log.is_empty() {
                                    eprintln!("sidecar stderr:\n{}", log.join("\n"));
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("failed to spawn sidecar: {}", e);
                        }
                    }
                }
            }

            window.show().ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmds::kill_sidecar,
            cmds::await_initialization,
            cmds::consume_initial_deep_links,
            cmds::get_default_server_url,
            cmds::set_default_server_url,
            cmds::is_first_launch_onboarding_pending,
            cmds::finish_first_launch_onboarding,
            cmds::get_display_backend,
            cmds::set_display_backend,
            cmds::parse_markdown,
            cmds::check_app_exists,
            cmds::resolve_app_path,
            cmds::set_background_color,
            cmds::export_debug_logs,
            cmds::record_fatal_renderer_error,
            cmds::relaunch,
            cmds::install_cli,
            cmds::store_get,
            cmds::store_set,
            cmds::store_delete,
            cmds::store_clear,
            cmds::store_keys,
            cmds::store_length,
            cmds::get_window_count,
            cmds::get_window_id,
            cmds::get_window_focused,
            cmds::set_window_focus,
            cmds::show_window,
            cmds::get_zoom_factor,
            cmds::set_zoom_factor,
            cmds::get_pinch_zoom_enabled,
            cmds::set_pinch_zoom_enabled,
            cmds::set_titlebar,
            cmds::run_desktop_menu_action,
            cmds::open_link,
            cmds::open_path,
            cmds::read_clipboard_image,
            cmds::show_notification,
            cmds::read_picked_file,
            cmds::release_picked_files,
            cmds::open_directory_picker,
            cmds::open_file_picker,
            cmds::save_file_picker,
            cmds::wsl_get_state,
            cmds::wsl_probe_runtime,
            cmds::wsl_refresh_distros,
            cmds::wsl_install_wsl,
            cmds::wsl_install_distro,
            cmds::wsl_probe_addable,
            cmds::wsl_install_opencode,
            cmds::wsl_open_terminal,
            cmds::wsl_add_server,
            cmds::wsl_remove_server,
            cmds::wsl_start_server,
            cmds::updater_check,
            cmds::updater_install,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
