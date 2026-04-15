use crate::Result;
use crate::llm::provider::LlmProvider;
use std::process::Command;

pub struct GitIntelligence {
    llm: Box<dyn LlmProvider>,
}

impl GitIntelligence {
    pub fn new(llm: Box<dyn LlmProvider>) -> Self {
        Self { llm }
    }

    pub async fn suggest_commit_message(&self) -> Result<String> {
        let diff = Command::new("git")
            .args(["diff", "--staged"])
            .output()?
            .stdout;
        let diff_str = String::from_utf8_lossy(&diff);

        if diff_str.is_empty() {
            return Ok("No staged changes".to_string());
        }

        let prompt = format!(
            "Generate a concise conventional commit message for this diff:\n{}",
            diff_str
        );

        self.llm.generate(&prompt).await
    }

    pub async fn analyze_changes(&self) -> Result<String> {
        let status = Command::new("git").arg("status").output()?;
        let status_str = String::from_utf8_lossy(&status.stdout);

        let prompt = format!(
            "Summarize the current git repository state:\n{}",
            status_str
        );

        self.llm.generate(&prompt).await
    }
}