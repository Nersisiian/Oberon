use super::base::{Tool, ToolResult};
use crate::llm::ollama::OllamaProvider;
use crate::llm::provider::LlmProvider;
use crate::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::debug;

pub struct RefactorTool {
    llm: Arc<OllamaProvider>,
}

impl Default for RefactorTool {
    fn default() -> Self {
        Self::new()
    }
}

impl RefactorTool {
    pub fn new() -> Self {
        Self {
            llm: Arc::new(OllamaProvider::new()),
        }
    }
}

#[async_trait]
impl Tool for RefactorTool {
    fn name(&self) -> &'static str {
        "refactor"
    }

    fn description(&self) -> &'static str {
        "Refactor code according to instructions"
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        if !input.is_object() || input.get("path").is_none() || input.get("instruction").is_none() {
            return Err(crate::OberonError::tool(
                self.name(),
                "Input must have 'path' and 'instruction' fields",
            ));
        }
        Ok(())
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let path = input["path"].as_str().unwrap();
        let instruction = input["instruction"].as_str().unwrap();
        debug!(path, instruction, "Refactoring code");

        let code = tokio::fs::read_to_string(path).await?;

        let prompt = format!(
            "Refactor the following code according to this instruction: {}\n\n\
             Return ONLY the refactored code, no explanations.\n\n\
             Code:\n{}",
            instruction, code
        );

        let refactored = self.llm.generate(&prompt).await?;

        tokio::fs::write(path, &refactored).await?;

        Ok(ToolResult {
            success: true,
            output: format!("Refactored {} successfully", path),
            metadata: Some(json!({ "path": path, "instruction": instruction })),
        })
    }

    fn is_destructive(&self) -> bool {
        true
    }
}
