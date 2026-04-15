use std::sync::Arc;
use tracing::{debug, info};

use crate::memory::short_term::ShortTermMemory;
use crate::planner::plan::Step;
use crate::tools::registry::ToolRegistry;
use crate::Result;
use serde_json::Value;

pub struct ExecutionEngine {
    tool_registry: Arc<ToolRegistry>,
}

impl ExecutionEngine {
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    pub async fn execute_step(
        &mut self,
        step: &Step,
        memory: &mut ShortTermMemory,
    ) -> Result<String> {
        info!(tool = %step.tool, "Executing step");
        let result = self
            .tool_registry
            .execute(&step.tool, step.input.clone())
            .await?;

        if result.success {
            memory.add_observation(&result.output);
            Ok(result.output)
        } else {
            Err(crate::OberonError::tool(&step.tool, &result.output))
        }
    }

    pub async fn execute_tool(&mut self, tool: &str, input: Value) -> Result<String> {
        debug!(tool, "Executing tool directly");
        let result = self.tool_registry.execute(tool, input).await?;
        Ok(result.output)
    }
}
