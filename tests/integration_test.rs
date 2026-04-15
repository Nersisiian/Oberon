use oberon::agent::Agent;
use oberon::llm::ollama::OllamaProvider;
use oberon::Result;

#[tokio::test]
#[ignore] // Requires Ollama running locally
async fn test_agent_basic_task() -> Result<()> {
    let llm = Box::new(OllamaProvider::new());
    let mut agent = Agent::new(llm);
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