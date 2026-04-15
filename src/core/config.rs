use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub llm: LlmConfig,
    pub safety: SafetyConfig,
    pub memory: MemoryConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub allowlist: Vec<PathBuf>,
    pub require_confirmation: bool,
    pub dry_run_default: bool,
    pub max_file_size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub short_term_capacity: usize,
    pub long_term_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub json_output: bool,
    pub with_timestamps: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LlmConfig {
                provider: "ollama".to_string(),
                model: "llama3".to_string(),
                endpoint: "http://localhost:11434/api/generate".to_string(),
                temperature: 0.2,
                max_tokens: 4096,
                timeout_seconds: 120,
            },
            safety: SafetyConfig {
                allowlist: vec![
                    PathBuf::from("."),
                    dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")),
                ],
                require_confirmation: true,
                dry_run_default: false,
                max_file_size_bytes: 10 * 1024 * 1024, // 10 MB
            },
            memory: MemoryConfig {
                short_term_capacity: 20,
                long_term_path: dirs::cache_dir().map(|p| p.join("oberon/memory.json")),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                json_output: false,
                with_timestamps: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> crate::Result<Self> {
        let config_path = dirs::config_dir()
            .map(|p| p.join("oberon/config.toml"))
            .ok_or_else(|| crate::OberonError::Config("No config dir".into()))?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }
}