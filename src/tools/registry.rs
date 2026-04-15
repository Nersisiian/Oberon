use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;

use super::base::{Tool, ToolResult};
use super::*;
use crate::Result;

/// Thread-safe registry using DashMap for concurrent access.
#[derive(Clone)]
pub struct ToolRegistry {
    tools: Arc<DashMap<String, Arc<dyn Tool>>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let registry = Self {
            tools: Arc::new(DashMap::new()),
        };
        // Register built-in tools
        registry.register(Arc::new(file_read::FileReadTool::new()));
        registry.register(Arc::new(file_write::FileWriteTool::new()));
        registry.register(Arc::new(list_dir::ListDirTool::new()));
        registry.register(Arc::new(search_code::SearchCodeTool::new()));
        registry.register(Arc::new(git_diff::GitDiffTool::new()));
        registry.register(Arc::new(git_commit::GitCommitTool::new()));
        registry.register(Arc::new(refactor::RefactorTool::new()));
        registry
    }

    pub fn register(&self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).map(|r| r.value().clone())
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.iter().map(|entry| entry.key().clone()).collect()
    }

    pub async fn execute(&self, name: &str, input: serde_json::Value) -> Result<ToolResult> {
        let tool = self.get(name).ok_or_else(|| {
            crate::OberonError::Tool {
                tool: name.to_string(),
                message: "Tool not found".to_string(),
            }
        })?;

        tool.validate_input(&input)?;
        tool.execute(input).await
    }
}