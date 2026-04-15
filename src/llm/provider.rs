use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::agent::react::ActionResponse;
use crate::memory::short_term::ShortTermMemory;
use crate::Result;

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<String>> + Send>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
    async fn generate_structured(&self, prompt: &str) -> Result<String>;
    async fn generate_stream(&self, prompt: &str) -> Result<ResponseStream>;
    async fn generate_action(
        &self,
        thought: &str,
        memory: &ShortTermMemory,
    ) -> Result<ActionResponse>;
    fn name(&self) -> &'static str;
}
