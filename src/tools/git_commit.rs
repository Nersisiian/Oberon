use super::base::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::Result;
use std::process::Command;
use tracing::debug;

pub struct GitCommitTool;

impl GitCommitTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GitCommitTool {
    fn name(&self) -> &'static str {
        "git_commit"
    }

    fn description(&self) -> &'static str {
        "Create a git commit with given message"
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        if !input.is_object() || input.get("message").is_none() {
            return Err(crate::OberonError::tool(
                self.name(),
                "Input must have 'message' field",
            ));
        }
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let message = input["message"].as_str().unwrap();
        debug!(message, "Creating git commit");

        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Ok(ToolResult {
                success: false,
                output: format!("Git commit failed: {}", err),
                metadata: None,
            });
        }

        Ok(ToolResult {
            success: true,
            output: format!("Committed with message: {}", message),
            metadata: Some(json!({ "message": message })),
        })
    }

    fn is_destructive(&self) -> bool {
        true
    }
}