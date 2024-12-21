use anyhow::{self, Context};
use aws_sdk_bedrockruntime::{
    operation::converse::ConverseOutput,
    types::{
        builders::ToolConfigurationBuilder, ContentBlock, ConversationRole, InferenceConfiguration,
        Message, SpecificToolChoice, SystemContentBlock, Tool, ToolConfiguration, ToolInputSchema,
        ToolSpecification, ToolUseBlock,
    },
    Client as BedrockClient,
};
use aws_smithy_types::Document;
use lambda_runtime::tracing::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::tool::{ToDocument, ToolJsonSchema};

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
        let json_schema = json!({
                "type": "object",
                "required": ["name", "summary", "terrain", "enemies", "enemy_type", "health", "attack_name", "attack_damage"],
                "properties":{
                    "name": {
                        "type":"string",
                        "description":"A name for the battle encounter"
                    },
                    "summary":{
                        "type":"string",
                        "description":"A 1-4 sentence description of the battle scenario"
                    },
                    "terrain":{
                        "type":"string",
                        "description":"A description of the terrain the battle is happening in",
                        "enum": ["Cave", "Desert", "Forest"]
                    },
                    "enemies": {
                        "type": "array",
                        "description": "A list of enemies to fight",
                        "items": {
                            "type": "object",
                            "description": "An Object that defines an Enemy",
                            "properties": {
                                "enemy_type":{
                                    "type": "string",
                                    "description": "Type of enemy"
                                },
                                "health":{
                                    "type": "integer",
                                    "description": "Total health of the enemy",
                                    "minimum": 1,
                                    "maximum": 20
                                },
                                "attack":{
                                    "type": "object",
                                    "description": "An Object that defines an enemies attack",
                                    "properties": {
                                        "attack_name": {
                                            "type": "string",
                                            "description": "Name of the attack"
                                        },
                                        "attack_damage": {
                                            "type": "string",
                                            "description": "Damage value of attack as a integer value",
                                        }
                                    }
                                }
                            }
                        }
                    }

                },
        });

        let document = ToolJsonSchema::new(json_schema)?.to_document();

        let tool_input_schema = ToolInputSchema::Json(document);

        let tool_spec = ToolSpecification::builder()
            .name("battle")
            .description("Creates the Battle scenario for A dungeons and dragons style text adventure in a json format.")
            .input_schema(tool_input_schema)
            .build()?;

        let tool = Tool::ToolSpec(tool_spec);

        let tool_config = ToolConfiguration::builder()
            .tools(tool)
            .tool_choice(aws_sdk_bedrockruntime::types::ToolChoice::Tool(
                SpecificToolChoice::builder().name("battle").build()?,
            ))
            .build()?;

        let response = self
            .client
            .converse()
            .model_id(&self.model)
            .set_inference_config(config)
            .tool_config(tool_config)
            .system(SystemContentBlock::Text(self.system.clone()))
            .set_messages(Some(self.messages.clone()))
            .send()
            .await?;
        println!("Converse Res: {:?}", response);

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
    fn get_text_output(&self) -> anyhow::Result<String>;
    fn get_tool_output(&self) -> anyhow::Result<Vec<ToolUseBlock>>;
}

impl ParseConverseOuput for ConverseOutput {
    fn get_text_output(&self) -> anyhow::Result<String> {
        let output = self.output();
        info!("OUTPUT: {:?}", output);
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
            Ok(message) => message,
            Err(_) => {
                panic!("Output is anot a message")
            }
        };

        let contents = message.content();
        info!("Contents count: {}", contents.len());
        let mut tool_results: Vec<ToolUseBlock> = vec![];
        for content in contents {
            match content {
                ContentBlock::Text(text) => {
                    info!("Text: {:?}", text)
                }
                ContentBlock::ToolUse(tool_use) => {
                    info!("Tool Use: {:?}", tool_use);
                    tool_results.push(tool_use.clone());
                }
                _ => break,
            }
        }
        Ok(tool_results)
    }
}
