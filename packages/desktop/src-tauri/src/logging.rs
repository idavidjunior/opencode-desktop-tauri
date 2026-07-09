use std::fs;
use std::io::Write;
use std::path::PathBuf;

// ponytail: basic JSON-per-line logger — add rotation, levels, and tail when diagnostics need it
#[allow(dead_code)]
pub struct Logger {
    _dir: PathBuf,
    current: PathBuf,
}

#[allow(dead_code)]
impl Logger {
    pub fn new(base: PathBuf) -> Self {
        let stamp = chrono_stamp();
        let dir = base.join("logs");
        let current = dir.join(format!("{}.log", stamp));
        let _ = fs::create_dir_all(&dir);
        let mut s = Self { _dir: dir, current };
        s.write("main", "logging initialized", None, "info");
        s
    }

    pub fn write(&mut self, scope: &str, msg: &str, extra: Option<&serde_json::Value>, level: &str) {
        let entry = serde_json::json!({
            "timestamp": chrono_stamp(),
            "scope": scope,
            "level": level,
            "message": msg,
            "extra": extra,
        });
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(&self.current) {
            let _ = writeln!(f, "{}", entry);
        }
    }
}

#[allow(dead_code)]
fn chrono_stamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let ms = now.subsec_millis();
    format!("{}.{:03}", secs, ms)
}
