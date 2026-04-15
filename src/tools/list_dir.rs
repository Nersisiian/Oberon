use super::base::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::Result;
use tokio::fs;

pub struct ListDirTool;

impl ListDirTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ListDirTool {
    fn name(&self) -> &'static str {
        "list_dir"
    }

    fn description(&self) -> &'static str {
        "List contents of a directory"
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        if !input.is_object() || input.get("path").is_none() {
            return Err(crate::OberonError::Tool(
                "Input must have 'path' field".into(),
            ));
        }
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let path = input["path"].as_str().unwrap();
        let mut entries = vec![];
        let mut dir = fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            let file_type = if entry.file_type().await?.is_dir() { "dir" } else { "file" };
            entries.push(format!("{} ({})", name, file_type));
        }

        Ok(ToolResult {
            success: true,
            output: entries.join("\n"),
            metadata: Some(json!({ "path": path, "count": entries.len() })),
        })
    }
}