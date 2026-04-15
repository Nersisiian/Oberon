use oberon::agent::Agent;
use oberon::context::Context;
use oberon::core::config::Config;
use oberon::llm::ollama::OllamaProvider;
use oberon::Result;
use std::sync::Arc;

#[tokio::test]
#[ignore]
async fn test_agent_basic_task() -> Result<()> {
    let config = Arc::new(Config::default());
    let llm = Arc::new(OllamaProvider::from_config(&config.llm));
    let context = Arc::new(Context::new((*config).clone(), llm));
    let mut agent = Agent::new(context);
    let result = agent.run("List files in current directory").await?;
    assert!(!result.is_empty());
    Ok(())
}

#[test]
fn test_tool_registry() {
    let registry = oberon::tools::ToolRegistry::new();
    let tools = registry.list_tools();
    assert!(tools.contains(&"read_file".to_string()));
    assert!(tools.contains(&"write_file".to_string()));
}
