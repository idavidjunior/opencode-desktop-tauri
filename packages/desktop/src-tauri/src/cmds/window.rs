use tauri::Manager;

use crate::state::AppState;
use crate::store::Store;

#[tauri::command]
pub fn get_window_count(app: tauri::AppHandle) -> Result<usize, String> {
    Ok(app.webview_windows().len())
}

// ponytail: always returns "main" — Tauri single-window for now, multi-window when Electron parity needed
#[tauri::command]
pub fn get_window_id() -> Result<String, String> {
    Ok("main".into())
}

#[tauri::command]
pub fn get_window_focused(app: tauri::AppHandle) -> Result<bool, String> {
    let win = app.get_webview_window("main").ok_or("no main window")?;
    Ok(win.is_focused().unwrap_or(false))
}

#[tauri::command]
pub fn set_window_focus(app: tauri::AppHandle) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("no main window")?;
    win.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_window(app: tauri::AppHandle) -> Result<(), String> {
    let win = app.get_webview_window("main").ok_or("no main window")?;
    win.show().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_zoom_factor() -> Result<f64, String> {
    Ok(1.0)
}

#[tauri::command]
pub fn set_zoom_factor(_factor: f64) {}

#[tauri::command]
pub fn get_pinch_zoom_enabled(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let store = Store::new(state.inner().data_dir.join("opencode.settings"));
    Ok(store.get("pinchZoomEnabled") == Some("true".into()))
}

#[tauri::command]
pub fn set_pinch_zoom_enabled(
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    let store = Store::new(state.inner().data_dir.join("opencode.settings"));
    store.set("pinchZoomEnabled", if enabled { "true" } else { "false" }).ok();
    Ok(())
}

// ponytail: titlebar theme is Electron-specific (BrowserWindow.setTitleBarOverlay); Tauri uses system titlebar
#[tauri::command]
pub fn set_titlebar(_theme: serde_json::Value) {}

// ponytail: Electron handles 19 menu actions — Tauri handles browser-level ones natively, rest are frontend-managed
#[tauri::command]
pub fn run_desktop_menu_action(
    action: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let win = app.get_webview_window("main");
    match action.as_str() {
        "window.close" => {
            if let Some(w) = win {
                w.close().map_err(|e| e.to_string())?;
            }
        }
        "window.minimize" => {
            if let Some(w) = win {
                w.minimize().map_err(|e| e.to_string())?;
            }
        }
        "window.toggleMaximize" => {
            if let Some(w) = win {
                if w.is_maximized().unwrap_or(false) {
                    w.unmaximize().map_err(|e| e.to_string())?;
                } else {
                    w.maximize().map_err(|e| e.to_string())?;
                }
            }
        }
        "window.new" => {
            // ponytail: no multi-window in Tauri — emit event if renderer handles it
        }
        "view.toggleFullscreen" => {
            if let Some(w) = win {
                let full = w.is_fullscreen().unwrap_or(false);
                w.set_fullscreen(!full).map_err(|e| e.to_string())?;
            }
        }
        "view.reload" => {
            if let Some(w) = win {
                w.eval("location.reload()").ok();
            }
        }
        "view.toggleDevTools" => {
            // ponytail: devtools controllable via Tauri window menu or keyboard
        }
        "view.resetZoom" | "view.zoomIn" | "view.zoomOut" => {
            // ponytail: zoom API not exposed by Tauri webview — frontend can scale via CSS
        }
        "app.checkForUpdates" => {
            // ponytail: updater is stubbed — would emit event to trigger frontend update UI
        }
        "app.relaunch" => {
            app.request_restart();
        }
        // edit.* actions are handled by the OS native menu on all platforms
        "edit.undo" | "edit.redo" | "edit.cut" | "edit.copy"
        | "edit.paste" | "edit.delete" | "edit.selectAll" => {}
        _ => {}
    }
    Ok(())
}
