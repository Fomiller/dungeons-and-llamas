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
    pub instructions: String,
    pub output: Option<ConverseOutput>,
}

impl LlmHandler {
    pub fn new(client: BedrockClient, model: String, system: String, instructions: String) -> Self {
        Self {
            client,
            model,
            system,
            messages: vec![],
            instructions,
            output: None,
        }
    }

    pub async fn converse(&mut self) -> anyhow::Result<()> {
        let response = self
            .client
            .converse()
            .model_id(&self.model)
            .system(SystemContentBlock::Text(self.system.clone()))
            .set_messages(Some(self.messages.clone()))
            .send()
            .await?;

        self.output = Some(response);
        Ok(())
    }

    pub fn get_converse_output_text(&self) -> anyhow::Result<String> {
        let text = self
            .output
            .clone()
            .expect("No ouput available")
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

    pub fn create_input(&mut self, contexts: Vec<String>, prompt: &str) -> String {
        let mut input_context = String::new();
        for context in contexts {
            input_context.push_str(&context)
        }
        let input = format!(
            "{}\n<Context>{}</Context>\n<Prompt>\n{}\n</Prompt>",
            self.instructions, input_context, prompt
        );

        input
    }
}
