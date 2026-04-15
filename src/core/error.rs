use thiserror::Error;

#[derive(Error, Debug)]
pub enum OberonError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("LLM error: {0}")]
    Llm(String),

    #[error("Tool execution error: {tool} - {message}")]
    Tool { tool: String, message: String },

    #[error("Safety violation: {0}")]
    Safety(String),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Execution engine error: {0}")]
    Executor(String),

    #[error("Planning error: {0}")]
    Planner(String),

    #[error("Unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, OberonError>;

// Convenience constructors
impl OberonError {
    pub fn tool(tool: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Tool {
            tool: tool.into(),
            message: message.into(),
        }
    }

    pub fn llm(message: impl Into<String>) -> Self {
        Self::Llm(message.into())
    }
}