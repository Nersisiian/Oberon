use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermMemory {
    pub sessions: Vec<SessionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub goal: String,
    pub summary: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl LongTermMemory {
    pub async fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path).await?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self { sessions: vec![] })
        }
    }

    pub async fn save(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(path, content).await?;
        Ok(())
    }

    pub fn add_session(&mut self, summary: SessionSummary) {
        self.sessions.push(summary);
        if self.sessions.len() > 100 {
            self.sessions.remove(0);
        }
    }
}
