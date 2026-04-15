use super::base::{Tool, ToolResult};
use crate::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;

pub struct FileReadTool;

impl FileReadTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Read contents of a file"
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        if !input.is_object() || input.get("path").is_none() {
            return Err(crate::OberonError::Tool {
                tool: self.name().to_string(),
                message: "Input must have 'path' field".into(),
            });
        }
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let path = input["path"].as_str().unwrap();
        let content = fs::read_to_string(path).await?;

        Ok(ToolResult {
            success: true,
            output: content,
            metadata: Some(json!({ "path": path })),
        })
    }
}
