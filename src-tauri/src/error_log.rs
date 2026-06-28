use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub timestamp: String,
    pub source: String,
    pub level: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
}

pub struct ErrorLog {
    paths: Vec<PathBuf>,
    entries: Mutex<Vec<ErrorEntry>>,
}

impl ErrorLog {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let mut paths = vec![app_data_dir.join("error-log.jsonl")];

        if cfg!(debug_assertions) {
            if let Ok(cwd) = std::env::current_dir() {
                paths.push(cwd.join("logs").join("app-errors.jsonl"));
            }
        }

        for path in &paths {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
        }

        let mut loaded = Vec::new();
        if let Some(primary) = paths.first() {
            loaded = Self::read_file(primary);
        }

        Self {
            paths,
            entries: Mutex::new(loaded),
        }
    }

    pub fn report(
        &self,
        source: &str,
        level: &str,
        message: &str,
        context: Option<serde_json::Value>,
        stack: Option<String>,
    ) -> Result<ErrorEntry, String> {
        let entry = ErrorEntry {
            timestamp: Utc::now().to_rfc3339(),
            source: source.to_string(),
            level: level.to_string(),
            message: message.to_string(),
            context,
            stack,
        };

        let line = serde_json::to_string(&entry).map_err(|e| e.to_string())?;

        for path in &self.paths {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| e.to_string())?;
            writeln!(file, "{line}").map_err(|e| e.to_string())?;
        }

        self.entries
            .lock()
            .map_err(|e| e.to_string())?
            .push(entry.clone());

        eprintln!("[QPO][{}] {}: {}", entry.source, entry.level, entry.message);

        Ok(entry)
    }

    pub fn list(&self) -> Result<Vec<ErrorEntry>, String> {
        Ok(self
            .entries
            .lock()
            .map_err(|e| e.to_string())?
            .clone())
    }

    pub fn clear(&self) -> Result<(), String> {
        for path in &self.paths {
            if path.exists() {
                fs::write(path, "").map_err(|e| e.to_string())?;
            }
        }
        self.entries
            .lock()
            .map_err(|e| e.to_string())?
            .clear();
        Ok(())
    }

    pub fn log_path(&self) -> String {
        self.paths
            .last()
            .or_else(|| self.paths.first())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    fn read_file(path: &PathBuf) -> Vec<ErrorEntry> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    return None;
                }
                serde_json::from_str(trimmed).ok()
            })
            .collect()
    }
}

pub type SharedErrorLog = Mutex<ErrorLog>;
