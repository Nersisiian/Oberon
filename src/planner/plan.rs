use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::context::Context;
use crate::llm::provider::LlmProvider;
use crate::memory::short_term::ShortTermMemory;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub description: String,
    pub tool: String,
    pub input: serde_json::Value,
}

pub struct Planner {
    llm: Arc<dyn LlmProvider>,
}

impl Planner {
    pub fn new(llm: Arc<dyn LlmProvider>) -> Self {
        Self { llm }
    }

    pub async fn create_plan(
        &self,
        user_input: &str,
        memory: &ShortTermMemory,
        context: &Context,
    ) -> Result<Plan> {
        debug!("Creating plan for: {}", user_input);

        let available_tools = context.tools.list_tools().join(", ");
        let prompt = format!(
            "You are an AI planning agent. Create a step-by-step plan to accomplish the user's request.\n\
             Context: {:?}\n\
             Request: {}\n\
             Available tools: {}\n\
             Respond with a JSON object containing 'goal' and 'steps' array.\n\
             Each step has 'description', 'tool', and 'input' fields.",
            memory.get_context(),
            user_input,
            available_tools
        );

        let response = self.llm.generate_structured(&prompt).await?;
        let plan: Plan = serde_json::from_str(&response)
            .map_err(|e| crate::OberonError::Planner(format!("Invalid plan JSON: {}", e)))?;
        Ok(plan)
    }

    pub async fn replan_step(
        &self,
        failed_step: &Step,
        observation: &str,
        _context: &Context,
    ) -> Result<Option<Step>> {
        let prompt = format!(
            "The step '{}' using tool '{}' failed with observation: {}\n\
             Suggest an alternative step (tool and input) in JSON format, or reply with {{}} if no alternative.",
            failed_step.description, failed_step.tool, observation
        );

        let response = self.llm.generate_structured(&prompt).await?;
        if response.trim() == "{}" {
            return Ok(None);
        }
        let step: Step = serde_json::from_str(&response)
            .map_err(|e| crate::OberonError::Planner(format!("Invalid alternative step JSON: {}", e)))?;
        Ok(Some(step))
    }
}