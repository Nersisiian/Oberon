use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyPolicy {
    pub blocked_commands: Vec<String>,
    pub max_file_size_mb: u64,
    pub allowed_file_extensions: Vec<String>,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        Self {
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "dd if=/dev/zero".to_string(),
            ],
            max_file_size_mb: 100,
            allowed_file_extensions: vec![
                "rs".into(), "toml".into(), "md".into(), "json".into(),
                "yaml".into(), "yml".into(), "py".into(), "js".into(),
                "ts".into(), "html".into(), "css".into(),
            ],
        }
    }
}