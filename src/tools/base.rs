use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub metadata: Option<serde_json::Value>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn validate_input(&self, input: &serde_json::Value) -> Result<()>;
    async fn execute(&self, input: serde_json::Value) -> Result<ToolResult>;

    fn is_destructive(&self) -> bool {
        false
    }
}
