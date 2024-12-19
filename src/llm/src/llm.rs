use anyhow;
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{ContentBlock, ConversationRole, InferenceConfiguration, Message, SystemContentBlock},
    Client as BedrockClient,
};

#[derive(Debug, Clone)]
pub struct LlmHandler {
    pub client: aws_sdk_bedrockruntime::Client,
    pub model: String,
    pub system: String,
    pub messages: Vec<Message>,
    pub instructions: Option<String>,
}

impl LlmHandler {
    pub async fn new(model: String, system: String, instructions: Option<String>) -> Self {
        let config = aws_config::load_from_env().await;
        let client = BedrockClient::new(&config);
        Self {
            client,
            model,
            system,
            messages: vec![],
            instructions,
        }
    }

    pub async fn converse(
        &mut self,
        config: Option<InferenceConfiguration>,
    ) -> anyhow::Result<ConverseOutput> {
        let response = self
            .client
            .converse()
            .model_id(&self.model)
            .set_inference_config(config)
            .system(SystemContentBlock::Text(self.system.clone()))
            .set_messages(Some(self.messages.clone()))
            .send()
            .await?;

        Ok(response)
    }

    pub fn set_messages(&mut self, input: &str) -> anyhow::Result<()> {
        self.messages = vec![Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(input.to_string()))
            .build()?];
        Ok(())
    }

    pub fn create_input(&mut self, contexts: Option<Vec<String>>, prompt: &str) -> String {
        let mut input = String::new();

        if let Some(instructions) = &self.instructions {
            input.push_str(&format!("{}\n", instructions))
        }

        if let Some(contexts) = contexts {
            input.push_str("<Context>\n");
            for context in contexts {
                input.push_str(&format!("{}\n", &context))
            }
            input.push_str("</Context>\n");
        }

        let prompt = format!("<Prompt>\n{}\n</Prompt>", prompt);

        input.push_str(&prompt);

        input
    }
}

pub trait ParseConverseOuput {
    fn get_text(&self) -> anyhow::Result<String>;
}

impl ParseConverseOuput for ConverseOutput {
    fn get_text(&self) -> anyhow::Result<String> {
        let text = self
            .output()
            .ok_or_else(|| anyhow::anyhow!("no output"))?
            .as_message()
            .map_err(|_| anyhow::anyhow!("output not a message"))?
            .content()
            .first()
            .ok_or_else(|| anyhow::anyhow!("no content in message"))?
            .as_text()
            .map_err(|_| anyhow::anyhow!("content is not text"))?
            .to_string();
        Ok(text)
    }
}
