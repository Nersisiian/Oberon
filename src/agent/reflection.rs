use crate::llm::provider::LlmProvider;
use crate::Result;

pub struct Reflection {
    llm: Box<dyn LlmProvider>,
}

impl Reflection {
    pub fn new(llm: Box<dyn LlmProvider>) -> Self {
        Self { llm }
    }

    pub async fn reflect(&self, action: &str, observation: &str) -> Result<String> {
        let prompt = format!(
            "You performed action: {}\n\
             You observed: {}\n\
             Was this successful? What could be improved?",
            action, observation
        );
        self.llm.generate(&prompt).await
    }
}