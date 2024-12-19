use anyhow;
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{Message, SystemContentBlock},
    Client as BedrockClient,
};

#[derive(Debug, Clone)]
pub struct LlmHandler {
    pub client: aws_sdk_bedrockruntime::Client,
    pub model: String,
    pub system: String,
    pub messages: Vec<Message>,
}

impl LlmHandler {
    pub fn new(client: BedrockClient, model: String, system: String) -> Self {
        Self {
            client,
            model,
            system,
            messages: vec![],
        }
    }

    pub async fn converse(self) -> anyhow::Result<ConverseOutput> {
        let response = self
            .client
            .converse()
            .model_id(self.model)
            .system(SystemContentBlock::Text(self.system))
            .set_messages(Some(self.messages))
            .send()
            .await?;

        Ok(response)
    }

    pub fn get_converse_output_text(output: ConverseOutput) -> anyhow::Result<String> {
        let text = output
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

    pub fn create_input(contexts: Vec<String>, instructions: String, prompt: String) -> String {
        let mut input_context = String::new();
        for context in contexts {
            input_context.push_str(&context)
        }
        let input = format!(
            "{}\n<Context>{}</Context>\n<Prompt>\n{}\n</Prompt>",
            instructions, input_context, prompt
        );

        input
    }
}
