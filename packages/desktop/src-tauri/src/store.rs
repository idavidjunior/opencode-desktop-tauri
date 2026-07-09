use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

// ponytail: recovers from poisoned mutex by ignoring the poison bit
pub fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

pub struct Store {
    path: PathBuf,
    data: Mutex<HashMap<String, serde_json::Value>>,
    loaded: Mutex<bool>,
}

impl Store {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            data: Mutex::new(HashMap::new()),
            loaded: Mutex::new(false),
        }
    }

    fn ensure_loaded(&self) {
        let mut loaded = lock(&self.loaded);
        if *loaded {
            return;
        }
        if let Ok(content) = fs::read_to_string(&self.path) {
            if let Ok(map) =
                serde_json::from_str::<HashMap<String, serde_json::Value>>(&content)
            {
                *lock(&self.data) = map;
            }
        }
        *loaded = true;
    }

    fn flush(&self) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let data = lock(&self.data);
        let json = serde_json::to_string(&*data).map_err(|e| e.to_string())?;
        fs::write(&self.path, json).map_err(|e| e.to_string())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.ensure_loaded();
        let data = lock(&self.data);
        let val = data.get(key)?;
        Some(match val {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), String> {
        self.ensure_loaded();
        {
            let mut data = lock(&self.data);
            data.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
        self.flush()
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        self.ensure_loaded();
        {
            let mut data = lock(&self.data);
            data.remove(key);
        }
        self.flush()
    }

    pub fn clear(&self) -> Result<(), String> {
        {
            let mut data = lock(&self.data);
            data.clear();
        }
        self.flush()
    }

    pub fn keys(&self) -> Vec<String> {
        self.ensure_loaded();
        let data = lock(&self.data);
        data.keys().cloned().collect()
    }

    pub fn length(&self) -> usize {
        self.ensure_loaded();
        let data = lock(&self.data);
        data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    fn tmp_store() -> (Store, PathBuf) {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("store-test-{}-{}", std::process::id(), id));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.json");
        (Store::new(path.clone()), path)
    }

    #[test]
    fn test_set_and_get() {
        let (store, _) = tmp_store();
        store.set("key1", "value1").unwrap();
        assert_eq!(store.get("key1"), Some("value1".into()));
    }

    #[test]
    fn test_get_missing() {
        let (store, _) = tmp_store();
        assert_eq!(store.get("nonexistent"), None);
    }

    #[test]
    fn test_delete() {
        let (store, _) = tmp_store();
        store.set("key1", "value1").unwrap();
        store.delete("key1").unwrap();
        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn test_clear() {
        let (store, _) = tmp_store();
        store.set("a", "1").unwrap();
        store.set("b", "2").unwrap();
        store.clear().unwrap();
        assert_eq!(store.length(), 0);
    }

    #[test]
    fn test_keys() {
        let (store, _) = tmp_store();
        store.set("x", "10").unwrap();
        store.set("y", "20").unwrap();
        let mut keys = store.keys();
        keys.sort();
        assert_eq!(keys, vec!["x", "y"]);
    }

    #[test]
    fn test_persistence() {
        let dir = std::env::temp_dir().join(format!("store-persist-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("persist.json");

        let store = Store::new(path.clone());
        store.set("persist", "ok").unwrap();
        drop(store);

        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, r#"{"persist":"ok"}"#);

        let store2 = Store::new(path);
        assert_eq!(store2.get("persist"), Some("ok".into()));
    }

    #[test]
    fn test_overwrite() {
        let (store, _) = tmp_store();
        store.set("key", "old").unwrap();
        store.set("key", "new").unwrap();
        assert_eq!(store.get("key"), Some("new".into()));
    }

    #[test]
    fn test_reload_after_drop() {
        let (store, path) = tmp_store();
        store.set("num", "42").unwrap();
        drop(store);
        let s = Store::new(path);
        assert_eq!(s.get("num"), Some("42".into()));
    }

    #[test]
    fn test_empty_store() {
        let (store, _) = tmp_store();
        assert_eq!(store.length(), 0);
        assert!(store.keys().is_empty());
    }
}
