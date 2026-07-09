use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn open_link(url: String, app: tauri::AppHandle) -> Result<(), String> {
    app.opener().open_url(&url, None::<&str>).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_path(path: String, _app_opt: Option<String>, app: tauri::AppHandle) -> Result<(), String> {
    app.opener().open_path(&path, None::<&str>).map_err(|e| e.to_string())
}

// ponytail: clipboard image not exposed by Tauri clipboard-manager plugin — add when plugin supports it
#[tauri::command]
pub fn read_clipboard_image() -> Result<Option<serde_json::Value>, String> {
    Ok(None)
}

#[tauri::command]
pub fn show_notification(title: String, body: Option<String>, app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_notification::NotificationExt;
    app.notification()
        .builder()
        .title(&title)
        .body(&body.unwrap_or_default())
        .show()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_picked_file(path: String) -> Result<Vec<u8>, String> {
    std::fs::read(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn release_picked_files(_token: String) {}
