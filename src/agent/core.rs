use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::context::Context;
use crate::executor::engine::ExecutionEngine;
use crate::memory::short_term::ShortTermMemory;
use crate::planner::plan::Planner;
use crate::Result;

pub struct Agent {
    context: Arc<Context>,
    planner: Planner,
    engine: ExecutionEngine,
    memory: ShortTermMemory,
}

impl Agent {
    pub fn new(context: Arc<Context>) -> Self {
        let planner = Planner::new(context.llm.clone());
        let engine = ExecutionEngine::new(context.tools.clone());
        let memory = ShortTermMemory::new(context.config.memory.short_term_capacity);

        Self {
            context,
            planner,
            engine,
            memory,
        }
    }

    pub async fn run(&mut self, user_input: &str) -> Result<String> {
        info!(user_input, "Starting agent execution");
        self.memory.add_user_message(user_input);

        // Generate plan
        let plan = self
            .planner
            .create_plan(user_input, &self.memory, &self.context)
            .await?;
        debug!(?plan, "Execution plan created");

        let mut final_result = String::new();
        for step in plan.steps {
            // Check safety before execution (if write operation)
            if self.is_destructive_tool(&step.tool) && self.context.sandbox.is_dry_run() {
                warn!(
                    tool = %step.tool,
                    "[DRY RUN] Would execute destructive operation"
                );
                final_result = format!("[DRY RUN] Would execute: {} - {}", step.tool, step.description);
                continue;
            }

            // Execute step
            let observation = self.engine.execute_step(&step, &mut self.memory).await?;

            // Evaluate and possibly replan
            if !self.evaluate_result(&observation) {
                warn!("Step failed, attempting recovery");
                if let Some(alt_step) = self
                    .planner
                    .replan_step(&step, &observation, &self.context)
                    .await?
                {
                    let alt_obs = self
                        .engine
                        .execute_step(&alt_step, &mut self.memory)
                        .await?;
                    final_result = alt_obs;
                    continue;
                }
            }

            final_result = observation;
        }

        info!("Agent execution completed");
        Ok(final_result)
    }

    fn evaluate_result(&self, observation: &str) -> bool {
        let lower = observation.to_lowercase();
        !lower.contains("error") && !lower.contains("failed") && !lower.contains("denied")
    }

    fn is_destructive_tool(&self, tool_name: &str) -> bool {
        matches!(
            tool_name,
            "write_file" | "git_commit" | "refactor"
        )
    }
}