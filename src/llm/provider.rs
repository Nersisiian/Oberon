use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::agent::react::ActionResponse;
use crate::memory::short_term::ShortTermMemory;
use crate::Result;

/// Stream of response chunks for streaming generation
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<String>> + Send>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Generate a complete response
    async fn generate(&self, prompt: &str) -> Result<String>;

    /// Generate a response with structured output (JSON)
    async fn generate_structured(&self, prompt: &str) -> Result<String>;

    /// Stream a response chunk by chunk
    async fn generate_stream(&self, prompt: &str) -> Result<ResponseStream>;

    /// Generate an action decision for the ReAct loop
    async fn generate_action(
        &self,
        thought: &str,
        memory: &ShortTermMemory,
    ) -> Result<ActionResponse>;

    /// Get the name of the provider
    fn name(&self) -> &'static str;
}