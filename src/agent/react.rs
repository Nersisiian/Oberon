use crate::executor::engine::ExecutionEngine;
use crate::llm::provider::LlmProvider;
use crate::memory::short_term::ShortTermMemory;
use crate::Result;
use std::sync::Arc;
use tracing::debug;

pub struct ReactLoop {
    llm: Arc<dyn LlmProvider>,
    max_iterations: usize,
}

impl ReactLoop {
    pub fn new(llm: Arc<dyn LlmProvider>) -> Self {
        Self {
            llm,
            max_iterations: 10,
        }
    }

    pub async fn run(
        &self,
        initial_prompt: &str,
        memory: &mut ShortTermMemory,
        engine: &mut ExecutionEngine,
    ) -> Result<String> {
        let mut current_thought = initial_prompt.to_string();

        for i in 0..self.max_iterations {
            debug!(iteration = i, "ReAct iteration");

            let response = self.llm.generate_action(&current_thought, memory).await?;

            match response {
                ActionResponse::FinalAnswer(answer) => return Ok(answer),
                ActionResponse::Action { tool, input } => {
                    let observation = engine.execute_tool(&tool, input).await?;
                    memory.add_observation(&observation);
                    current_thought = format!("Observation: {}", observation);
                }
            }
        }

        Err(crate::OberonError::Executor(
            "Max ReAct iterations reached".into(),
        ))
    }
}

pub enum ActionResponse {
    FinalAnswer(String),
    Action { tool: String, input: serde_json::Value },
}