use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAction {
    pub thought: String,
    pub action: ActionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionType {
    #[serde(rename = "tool")]
    Tool {
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "final_answer")]
    FinalAnswer { output: String },
}

pub fn validate_action(json: &str) -> Result<LlmAction, serde_json::Error> {
    serde_json::from_str(json)
}
