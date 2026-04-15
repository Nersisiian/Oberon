//! Shared application context providing dependency injection and configuration.

use std::sync::Arc;

use crate::core::config::Config;
use crate::llm::provider::LlmProvider;
use crate::safety::sandbox::Sandbox;
use crate::tools::registry::ToolRegistry;

#[derive(Clone)]
pub struct Context {
    pub config: Arc<Config>,
    pub llm: Arc<dyn LlmProvider>,
    pub tools: Arc<ToolRegistry>,
    pub sandbox: Arc<Sandbox>,
}

impl Context {
    pub fn new(config: Config, llm: Arc<dyn LlmProvider>) -> Self {
        let config = Arc::new(config);
        let tools = Arc::new(ToolRegistry::new());
        let sandbox = Arc::new(Sandbox::new(config.clone()));
        Self {
            config,
            llm,
            tools,
            sandbox,
        }
    }

    #[cfg(test)]
    pub fn test_context() -> Self {
        use crate::llm::ollama::OllamaProvider;
        Self::new(Config::default(), Arc::new(OllamaProvider::new()))
    }
}
