use async_stream::stream;
use async_trait::async_trait;
use futures::StreamExt;
use serde_json::{json, Value};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};

use super::provider::{LlmProvider, ResponseStream};
use crate::agent::react::ActionResponse;
use crate::core::config::Config;
use crate::executor::schema::LlmAction;
use crate::memory::short_term::ShortTermMemory;
use crate::Result;

#[derive(Clone)]
pub struct OllamaProvider {
    endpoint: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
    timeout: Duration,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self::from_config(&Config::default().llm)
    }

    pub fn from_config(config: &crate::core::config::LlmConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            timeout: Duration::from_secs(config.timeout_seconds),
            client,
        }
    }

    async fn call_ollama(&self, prompt: &str, system: &str) -> Result<String> {
        let payload = json!({
            "model": self.model,
            "prompt": prompt,
            "system": system,
            "stream": false,
            "options": {
                "temperature": self.temperature,
                "num_predict": self.max_tokens,
            }
        });

        let response = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| crate::OberonError::llm(e.to_string()))?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| crate::OberonError::llm(e.to_string()))?;

        let text = json["response"]
            .as_str()
            .ok_or_else(|| crate::OberonError::llm("No response field"))?
            .to_string();

        debug!(length = text.len(), "Ollama response received");
        Ok(text)
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        self.call_ollama(prompt, "You are a helpful AI assistant.")
            .await
    }

    async fn generate_structured(&self, prompt: &str) -> Result<String> {
        let system = "You must respond with valid JSON only. No explanations, no markdown formatting.";
        let full_prompt = format!("{}\nRespond with pure JSON:", prompt);
        let response = self.call_ollama(&full_prompt, system).await?;

        // Clean up response
        let json_str = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        Ok(json_str.to_string())
    }

    async fn generate_stream(&self, prompt: &str) -> Result<ResponseStream> {
        let payload = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": true,
            "options": {
                "temperature": self.temperature,
                "num_predict": self.max_tokens,
            }
        });

        let response = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| crate::OberonError::llm(e.to_string()))?;

        let stream = response.bytes_stream();
        let stream = stream! {
            for await chunk in stream {
                match chunk {
                    Ok(bytes) => {
                        if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                            for line in text.lines() {
                                if let Some(json_str) = line.strip_prefix("data: ") {
                                    if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                                        if let Some(token) = json["response"].as_str() {
                                            yield Ok(token.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Stream error: {}", e);
                        yield Err(crate::OberonError::llm(e.to_string()));
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    async fn generate_action(
        &self,
        thought: &str,
        memory: &ShortTermMemory,
    ) -> Result<ActionResponse> {
        let system = r#"You are an AI agent that decides on actions. 
Respond in pure JSON format:
{
  "thought": "your reasoning",
  "action": {
    "type": "tool",
    "name": "tool_name",
    "input": { ... }
  }
}
OR
{
  "thought": "your reasoning",
  "action": {
    "type": "final_answer",
    "output": "final response"
  }
}
Available tools: read_file, write_file, list_dir, search_code, git_diff, git_commit, refactor.
"#;

        let context = memory.get_context();
        let prompt = format!(
            "Context:\n{:?}\n\nCurrent thought:\n{}\n\nWhat is your next action? Respond with JSON only.",
            context, thought
        );

        let response = self.call_ollama(&prompt, system).await?;
        let json_str = response
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        let action: LlmAction = serde_json::from_str(json_str)
            .map_err(|e| crate::OberonError::llm(format!("Invalid JSON: {}", e)))?;

        match action.action {
            crate::executor::schema::ActionType::Tool { name, input } => {
                Ok(ActionResponse::Action { tool: name, input })
            }
            crate::executor::schema::ActionType::FinalAnswer { output } => {
                Ok(ActionResponse::FinalAnswer(output))
            }
        }
    }

    fn name(&self) -> &'static str {
        "ollama"
    }
}