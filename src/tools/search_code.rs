use super::base::{Tool, ToolResult};
use crate::Result;
use async_trait::async_trait;
use regex::Regex;
use serde_json::{json, Value};
use walkdir::WalkDir;

pub struct SearchCodeTool;

impl SearchCodeTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for SearchCodeTool {
    fn name(&self) -> &'static str {
        "search_code"
    }

    fn description(&self) -> &'static str {
        "Search for a regex pattern in files"
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        if !input.is_object() || input.get("pattern").is_none() || input.get("path").is_none() {
            return Err(crate::OberonError::Tool {
                tool: self.name().to_string(),
                message: "Input must have 'pattern' and 'path' fields".into(),
            });
        }
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let pattern = input["pattern"].as_str().unwrap();
        let path = input["path"].as_str().unwrap();
        let re = Regex::new(pattern)
            .map_err(|e| crate::OberonError::Tool { tool: self.name().to_string(), message: format!("Invalid regex: {}", e) })?;

        let mut matches = vec![];
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    for (line_num, line) in content.lines().enumerate() {
                        if re.is_match(line) {
                            matches.push(format!(
                                "{}:{}: {}",
                                entry.path().display(),
                                line_num + 1,
                                line
                            ));
                        }
                    }
                }
            }
        }

        Ok(ToolResult {
            success: true,
            output: if matches.is_empty() {
                "No matches found".to_string()
            } else {
                matches.join("\n")
            },
            metadata: Some(json!({ "pattern": pattern, "matches": matches.len() })),
        })
    }
}
