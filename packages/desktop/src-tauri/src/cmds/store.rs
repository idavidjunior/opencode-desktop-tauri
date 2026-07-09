use crate::state::AppState;
use crate::store::Store;

#[tauri::command]
pub fn store_get(
    state: tauri::State<'_, AppState>,
    name: String,
    key: String,
) -> Result<Option<String>, String> {
    let store = Store::new(state.inner().data_dir.join(&name));
    Ok(store.get(&key))
}

#[tauri::command]
pub fn store_set(
    state: tauri::State<'_, AppState>,
    name: String,
    key: String,
    value: String,
) -> Result<(), String> {
    let store = Store::new(state.inner().data_dir.join(&name));
    store.set(&key, &value)
}

#[tauri::command]
pub fn store_delete(
    state: tauri::State<'_, AppState>,
    name: String,
    key: String,
) -> Result<(), String> {
    let store = Store::new(state.inner().data_dir.join(&name));
    store.delete(&key)
}

#[tauri::command]
pub fn store_clear(state: tauri::State<'_, AppState>, name: String) -> Result<(), String> {
    let store = Store::new(state.inner().data_dir.join(&name));
    store.clear()
}

#[tauri::command]
pub fn store_keys(
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Vec<String>, String> {
    let store = Store::new(state.inner().data_dir.join(&name));
    Ok(store.keys())
}

#[tauri::command]
pub fn store_length(
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<usize, String> {
    let store = Store::new(state.inner().data_dir.join(&name));
    Ok(store.length())
}
