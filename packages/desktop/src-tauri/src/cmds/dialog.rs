use std::path::PathBuf;
use tauri_plugin_dialog::DialogExt;

fn file_info(path: &PathBuf) -> serde_json::Value {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    serde_json::json!({
        "path": path.to_string_lossy().to_string(),
        "name": name,
        "size": size,
    })
}

fn picker_from_opts(opts: &Option<serde_json::Value>, dlg: tauri_plugin_dialog::FileDialogBuilder<tauri::Wry>) -> tauri_plugin_dialog::FileDialogBuilder<tauri::Wry> {
    let mut dlg = dlg;
    if let Some(o) = opts {
        if let Some(t) = o.get("title").and_then(|v| v.as_str()) {
            dlg = dlg.set_title(t);
        }
        if let Some(p) = o.get("defaultPath").and_then(|v| v.as_str()) {
            dlg = dlg.set_directory(PathBuf::from(p));
        }
    }
    dlg
}

#[tauri::command]
pub async fn open_directory_picker(
    opts: Option<serde_json::Value>,
    app: tauri::AppHandle,
) -> Result<Option<serde_json::Value>, String> {
    let multiple = opts
        .as_ref()
        .and_then(|o| o.get("multiple").and_then(|v| v.as_bool()))
        .unwrap_or(false);

    let dlg = picker_from_opts(&opts, app.dialog().file());
    let dir = dlg.blocking_pick_folder();
    match dir {
        Some(f) => {
            let path = f.into_path().map_err(|e| e.to_string())?;
            let result = if multiple {
                serde_json::json!([path.to_string_lossy().to_string()])
            } else {
                serde_json::json!(path.to_string_lossy().to_string())
            };
            Ok(Some(result))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn open_file_picker(
    opts: Option<serde_json::Value>,
    app: tauri::AppHandle,
) -> Result<Option<serde_json::Value>, String> {
    let multiple = opts
        .as_ref()
        .and_then(|o| o.get("multiple").and_then(|v| v.as_bool()))
        .unwrap_or(false);

    let dlg = picker_from_opts(&opts, app.dialog().file());
    let dlg = if let Some(exts) = opts.as_ref().and_then(|o| o.get("extensions").and_then(|v| v.as_array())) {
        let filters: Vec<&str> = exts.iter().filter_map(|v| v.as_str()).collect();
        if !filters.is_empty() {
            dlg.add_filter("Allowed files", &filters)
        } else {
            dlg
        }
    } else {
        dlg
    };

    if multiple {
        let files = dlg.blocking_pick_files();
        match files {
            Some(files) => {
                let mut out = Vec::new();
                for f in files {
                    if let Ok(p) = f.into_path() {
                        out.push(file_info(&p));
                    }
                }
                Ok(Some(serde_json::json!({
                    "token": "tauri",
                    "files": out
                })))
            }
            None => Ok(None),
        }
    } else {
        let file = dlg.blocking_pick_file();
        match file {
            Some(f) => {
                let path = f.into_path().map_err(|e| e.to_string())?;
                let info = file_info(&path);
                Ok(Some(serde_json::json!({
                    "token": "tauri",
                    "files": [info]
                })))
            }
            None => Ok(None),
        }
    }
}

#[tauri::command]
pub async fn save_file_picker(
    opts: Option<serde_json::Value>,
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    let dlg = picker_from_opts(&opts, app.dialog().file());
    let file = dlg.blocking_save_file();
    match file {
        Some(f) => {
            let path = f.into_path().map_err(|e| e.to_string())?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}
