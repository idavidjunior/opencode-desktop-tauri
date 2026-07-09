// ponytail: uses tauri-plugin-updater — requires production pubkey in tauri.conf.json
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::UpdaterExt;

#[derive(Clone, Serialize)]
pub struct UpdaterState {
    pub status: String,
    pub version: Option<String>,
    pub note: Option<String>,
}

#[tauri::command]
pub async fn updater_check(app: AppHandle) -> Result<UpdaterState, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await {
        Ok(Some(update)) => {
            let state = UpdaterState {
                status: "update-available".into(),
                version: Some(update.version.clone()),
                note: update.body.clone(),
            };
            app.emit("updater-state", state.clone()).ok();
            Ok(state)
        }
        Ok(None) => {
            let state = UpdaterState {
                status: "up-to-date".into(),
                version: None,
                note: None,
            };
            app.emit("updater-state", state.clone()).ok();
            Ok(state)
        }
        Err(e) => {
            let state = UpdaterState {
                status: "error".into(),
                version: None,
                note: Some(e.to_string()),
            };
            app.emit("updater-state", state.clone()).ok();
            Ok(state)
        }
    }
}

#[tauri::command]
pub async fn updater_install(app: AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;
    match updater.check().await {
        Ok(Some(update)) => {
            update
                .download_and_install(
                    |_, _| {},
                    || {},
                )
                .await
                .map_err(|e| e.to_string())
        }
        Ok(None) => Err("no update available".into()),
        Err(e) => Err(e.to_string()),
    }
}
