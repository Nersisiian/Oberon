use super::base::{Tool, ToolResult};
use crate::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::process::Command;

pub struct GitDiffTool;

impl GitDiffTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GitDiffTool {
    fn name(&self) -> &'static str {
        "git_diff"
    }

    fn description(&self) -> &'static str {
        "Show git diff of current changes"
    }

    fn validate_input(&self, _input: &Value) -> Result<()> {
        Ok(())
    }

    async fn execute(&self, _input: Value) -> Result<ToolResult> {
        let output = Command::new("git").args(["diff", "--no-color"]).output()?;

        let diff = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(ToolResult {
            success: true,
            output: diff,
            metadata: Some(json!({ "staged": false })),
        })
    }
}
