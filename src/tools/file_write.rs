use super::base::{Tool, ToolResult};
use crate::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;
use tracing::debug;

pub struct FileWriteTool;

impl FileWriteTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn description(&self) -> &'static str {
        "Write content to a file"
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        if !input.is_object() || input.get("path").is_none() || input.get("content").is_none() {
            return Err(crate::OberonError::tool(
                self.name(),
                "Input must have 'path' and 'content' fields",
            ));
        }
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let path = input["path"].as_str().unwrap();
        let content = input["content"].as_str().unwrap();

        debug!(path, "Writing file");
        fs::write(path, content).await?;

        Ok(ToolResult {
            success: true,
            output: format!("File written: {}", path),
            metadata: Some(json!({ "path": path, "bytes": content.len() })),
        })
    }

    fn is_destructive(&self) -> bool {
        true
    }
}
