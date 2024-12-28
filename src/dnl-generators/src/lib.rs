pub mod battle;

use aws_sdk_bedrockruntime::types::builders::*;
use dnl_llm::llm::{LlmHandler, ParseConverseOuput};
use dnl_llm::tool::Tools;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait Generator {
    fn generate(&self) -> String;
}

pub struct Prompt {
    text: String,
    variables: HashMap<String, String>,
}

impl Prompt {
    fn format(&self) -> String {
        let mut result = self.text.to_string();

        for (key, value) in &self.variables {
            result = result.replace(key, value);
        }

        result
    }
}

pub trait JsonResponseGeneratorConfig {
    fn json_prompt(&self) -> String;
    fn model(&self) -> String;
    fn schema(&self) -> serde_json::Value;
    fn system_prompt(&self) -> String;
    fn text_prompt(&self) -> String;
    fn tool(&self) -> Tools;
}

#[derive(Debug, Clone)]
pub struct JsonResponseGenerator<C: JsonResponseGeneratorConfig, T: DeserializeOwned> {
    pub config: C,
    pub context: Option<Vec<String>>,
    pub model: LlmHandler,
    pub output: Option<T>,
    pub text: Option<String>,
    pub max_retries: u8,
    pub attempts: u8,
}

impl<C: JsonResponseGeneratorConfig, T: DeserializeOwned> JsonResponseGenerator<C, T> {
    pub async fn new(config: C) -> Self {
        let model = LlmHandler::new(config.model(), config.system_prompt()).await;

        let context = None;
        let text = None;
        let output = None;
        let max_retries = 5;
        let attempts = 0;

        Self {
            config,
            context,
            model,
            output,
            text,
            max_retries,
            attempts,
        }
    }

    pub async fn generate_text(&mut self) -> anyhow::Result<()> {
        let prompt = self
            .model
            .create_prompt(self.context.clone(), &self.config.text_prompt());

        let message = self.model.create_user_message(&prompt);

        self.model.set_messages(vec![message]);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(1.0)
            .top_p(1.0)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        let res = self.model.converse(None, inference_cfg).await?;

        let text = res.get_text_output()?;

        self.text = Some(text);

        Ok(())
    }

    pub async fn to_json<V>(&mut self, ctxs: Vec<String>) -> anyhow::Result<()>
    where
        V: DeserializeOwned,
    {
        let mut contexts: Vec<String> = Vec::new();

        // using the text in generate_text as context
        let text = self.text.clone().expect("Text not found");
        contexts.push(text);

        // if additional contexts are provide to function, such as error messages,
        // add them to the context that is used to create prompt
        if !ctxs.is_empty() {
            for ctx in ctxs {
                contexts.push(ctx)
            }
        }

        let prompt = self.config.json_prompt();

        let input = self.model.create_prompt(Some(contexts), &prompt);

        let message = self.model.create_user_message(&input);

        self.model.set_messages(vec![message]);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(0.3)
            .top_p(0.8)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        let res = self
            .model
            .converse(Some(self.config.tool()), inference_cfg)
            .await?;

        let tool_output = res.get_tool_output()?;

        let tool_value = tool_output
            .first()
            .expect("Missing tool output, this should not happen")
            .input
            .clone();

        let value = serde_json::to_value(tool_value)?;

        println!("TOOL VALUE: {:?}", value);

        match serde_json::from_value(value) {
            Ok(output) => {
                self.output = Some(output);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }
}
