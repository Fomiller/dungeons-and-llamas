use crate::tool::{ToolError, Tools};
use anyhow::{self, Context};
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{
        ContentBlock, ConversationRole, InferenceConfiguration, Message, SystemContentBlock,
        ToolUseBlock,
    },
    Client as BedrockClient,
};
use lambda_runtime::tracing::{debug, info};

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
        tool: Option<Tools>,
        config: Option<InferenceConfiguration>,
    ) -> anyhow::Result<ConverseOutput> {
        let system = SystemContentBlock::Text(self.system.clone());

        let messages = self.messages.clone();

        let mut converse = self
            .client
            .converse()
            .model_id(&self.model)
            .set_inference_config(config)
            .system(system)
            .set_messages(Some(messages));

        if let Some(tool) = tool {
            converse = converse.tool_config(tool.config()?);
        }

        let response = converse.send().await?;

        debug!("Converse Response: {:?}", response);

        Ok(response)
    }

    pub fn create_user_message(&mut self, input: &str) -> Message {
        Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(input.to_string()))
            .build()
            .expect("User message creation failed. This should never happen.")
    }

    pub fn create_assistant_message(&mut self, input: &str) -> Message {
        Message::builder()
            .role(ConversationRole::Assistant)
            .content(ContentBlock::Text(input.to_string()))
            .build()
            .expect("Assistant message creation failed. This should never happen.")
    }

    pub fn set_messages(&mut self, messages: Vec<Message>) {
        self.messages = messages;
    }

    pub fn create_prompt(&mut self, contexts: Option<Vec<String>>, prompt: &str) -> String {
        let mut output = String::new();

        if let Some(contexts) = contexts {
            output.push_str("<Context>\n");
            for context in contexts {
                output.push_str(&format!("{}\n", &context))
            }
            output.push_str("</Context>\n");
        }

        let prompt = format!("<Prompt>\n{}\n</Prompt>", prompt);

        output.push_str(&prompt);

        output
    }
}

pub trait ParseConverseOuput {
    fn get_text_output(&self) -> anyhow::Result<String>;
    fn get_tool_output(&self) -> anyhow::Result<Vec<ToolUseBlock>>;
}

impl ParseConverseOuput for ConverseOutput {
    fn get_text_output(&self) -> anyhow::Result<String> {
        let output = self.output();

        let text = output
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

    fn get_tool_output(&self) -> anyhow::Result<Vec<ToolUseBlock>> {
        let output = self.output().context("Error getting output")?;

        let message = match output.as_message() {
            Ok(message) => Ok(message),
            Err(_) => Err(ToolError::NoOutput),
        }?;

        let contents = message.content();

        let mut tool_results: Vec<ToolUseBlock> = vec![];

        for content in contents {
            match content {
                ContentBlock::ToolUse(tool_use) => {
                    tool_results.push(tool_use.clone());
                }
                _ => break,
            }
        }
        Ok(tool_results)
    }
}
