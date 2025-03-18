pub mod battle;
pub mod rest;
pub mod shop;

use std::collections::HashMap;

use battle::{BattleJsonGenerator, BattleJsonGeneratorConfig};
use dnl_sort_keys::encounter::EncounterSortKey;
use dnl_store::Store;
use dnl_types::llm::LlmHandler;
use dnl_types::llm::ParseConverseOutput;
use dnl_types::scenario::ScenarioInput;
use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::rest::RestToolOutput;
use dnl_types::tools::shop::ShopToolOutput;
use dnl_types::tools::ToolOutputEnum;
use dnl_types::tools::Tools;
use rest::{RestJsonGenerator, RestJsonGeneratorConfig};
use shop::{ShopJsonGenerator, ShopJsonGeneratorConfig};

use anyhow::Context;
use aws_sdk_bedrockruntime::types::builders::*;
use lambda_http::tracing::info;

pub enum JsonGeneratorConfigEnum {
    Battle(BattleJsonGeneratorConfig),
    Shop(ShopJsonGeneratorConfig),
    Rest(RestJsonGeneratorConfig),
}

#[derive(Clone)]
pub enum JsonGeneratorEnum {
    Battle(BattleJsonGenerator),
    Shop(ShopJsonGenerator),
    Rest(RestJsonGenerator),
}

impl JsonGeneratorEnum {
    pub async fn generate_text(&mut self) -> anyhow::Result<()> {
        match self {
            JsonGeneratorEnum::Battle(gen) => gen.generate_text().await,
            JsonGeneratorEnum::Shop(gen) => gen.generate_text().await,
            JsonGeneratorEnum::Rest(gen) => gen.generate_text().await,
        }
    }
    pub async fn generate_json(&mut self) -> anyhow::Result<()> {
        match self {
            JsonGeneratorEnum::Battle(gen) => gen.generate_json().await,
            JsonGeneratorEnum::Shop(gen) => gen.generate_json().await,
            JsonGeneratorEnum::Rest(gen) => gen.generate_json().await,
        }
    }
    pub async fn save_json(&mut self) -> anyhow::Result<()> {
        match self {
            JsonGeneratorEnum::Battle(gen) => gen.save_json().await,
            JsonGeneratorEnum::Shop(gen) => gen.save_json().await,
            JsonGeneratorEnum::Rest(gen) => gen.save_json().await,
        }
    }
    pub fn data(&mut self) -> Option<ToolOutputEnum> {
        match self {
            JsonGeneratorEnum::Battle(gen) => gen.data(),
            JsonGeneratorEnum::Shop(gen) => gen.data(),
            JsonGeneratorEnum::Rest(gen) => gen.data(),
        }
    }
}

#[derive(Clone)]
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
    fn scenario_input(&self) -> ScenarioInput;
    fn json_prompt(&self) -> String;
    fn model(&self) -> String;
    fn schema(&self) -> serde_json::Value;
    fn system_prompt(&self) -> String;
    fn text_prompt(&self) -> String;
    fn tool(&self) -> Tools;
}

#[derive(Clone)]
pub struct JsonResponseGenerator<C: JsonResponseGeneratorConfig> {
    pub config: C,
    pub context: Option<Vec<String>>,
    pub model: LlmHandler,
    pub text: Option<String>,
    pub data: Option<ToolOutputEnum>,
    pub max_retries: u8,
    pub attempts: u8,
}

impl<C> JsonResponseGenerator<C>
where
    C: JsonResponseGeneratorConfig,
{
    pub fn data(&mut self) -> Option<ToolOutputEnum> {
        self.data.clone()
    }

    pub async fn new(config: C) -> Self {
        let model = LlmHandler::new(config.model(), config.system_prompt()).await;

        let context = None;
        let text = None;
        let data = None;
        let max_retries = 5;
        let attempts = 0;

        Self {
            config,
            context,
            model,
            text,
            data,
            max_retries,
            attempts,
        }
    }

    pub async fn generate_text(&mut self) -> anyhow::Result<()> {
        let store = Store::new().await;
        let scenario_input = self.config.scenario_input();
        let user_id = scenario_input.user_id;
        let game_id = scenario_input.game_id;
        let level = scenario_input.level;
        let scenario = scenario_input.scenario;
        let resp = store
            .try_get_encounters(&user_id, &game_id, &level, &scenario)
            .await
            .context("try_get_encounters failed")?;

        let ctxs: Vec<String> = resp
            .iter()
            .map(|v| {
                let mut ctx = String::new();

                if let Some(name) = &v.name {
                    ctx.push_str(&format!("name: {}\n", name))
                };

                let txt_str = &format!("text: {}\n\n", &v.text);

                ctx.push_str(txt_str);

                ctx
            })
            .collect();

        info!("Ctx Count: {:?}", ctxs.len());
        info!("CTXS: {:?}", ctxs);

        let prompt = self
            .model
            .create_prompt(Some(ctxs), &self.config.text_prompt());

        let message = self.model.create_user_message(&prompt);

        self.model.set_messages(vec![message]);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(1.0)
            .top_p(1.0)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        let res = self.model.converse(None, inference_cfg).await?;

        let text = res.get_text_output().context("get_text_output failed")?;

        self.text = Some(text);

        Ok(())
    }

    pub async fn generate_json(&mut self) -> anyhow::Result<()> {
        let ctxs: Vec<String> = vec![];
        let mut fail_ctx: Option<String> = None;

        loop {
            self.attempts += 1;

            let mut temp_ctxs = ctxs.clone();

            if let Some(fail) = fail_ctx {
                temp_ctxs.push(fail)
            }

            match self.to_json(temp_ctxs).await {
                Ok(data) => {
                    self.data = Some(data);
                    return Ok(());
                }
                Err(err) => {
                    if self.attempts >= self.max_retries {
                        info!("Reached max retry limit of {}", self.max_retries);
                        return Err(anyhow::anyhow!(
                            "Exceeded max retry limit of {} when trying to create json",
                            self.max_retries
                        ));
                    }

                    info!("Retrying creating JSON output");
                    info!("Attempt {} failed: {}", self.attempts, err);

                    fail_ctx = Some(format!(
                        "The previous attempt to deserialize your response failed with the error: {}",
                        err.to_string()
                    ));
                }
            }
        }
    }

    pub async fn to_json(&mut self, ctxs: Vec<String>) -> anyhow::Result<ToolOutputEnum> {
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
            .await
            .context("Failed to call converse api")?;

        let tool_output = res.get_tool_output().context("Failed to get tool output")?;

        let tool_value = tool_output
            .first()
            .expect("Missing tool output, this should not happen")
            .input
            .clone();

        let value = serde_json::to_value(tool_value)
            .context("Failed to parese tool value to serde_json::Value")?;

        match self.config.tool() {
            Tools::Shop => {
                serde_json::from_value::<ShopToolOutput>(value)
                    .map(ToolOutputEnum::Shop)
                    .map_err(Into::into) // Convert serde_json error into your custom error type
            }
            Tools::Battle => serde_json::from_value::<BattleToolOutput>(value)
                .map(ToolOutputEnum::Battle)
                .map_err(Into::into),
            Tools::Rest => serde_json::from_value::<RestToolOutput>(value)
                .map(ToolOutputEnum::Rest)
                .map_err(Into::into),
        }
    }

    pub async fn save_json(&mut self) -> anyhow::Result<()> {
        let store = Store::new().await;

        let encounter = match self.config.tool() {
            Tools::Battle => EncounterSortKey::Battle,
            Tools::Shop => EncounterSortKey::Shop,
            Tools::Rest => EncounterSortKey::Rest,
        };

        let scenario_input = self.config.scenario_input();

        let level = scenario_input.level.parse::<u8>()?;
        let round = scenario_input.round.parse::<u8>()?;
        let user_id = &scenario_input.user_id;
        let game_id = &scenario_input.game_id;

        let mut state = HashMap::new();

        state.insert(
            "text".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::S(self.text.clone().unwrap_or("".to_string())),
        );

        let data = self.data.clone().unwrap();

        let map: HashMap<String, aws_sdk_dynamodb::types::AttributeValue> = match data {
            ToolOutputEnum::Battle(item) => serde_dynamo::to_item(item)?,
            ToolOutputEnum::Shop(item) => serde_dynamo::to_item(item)?,
            ToolOutputEnum::Rest(item) => serde_dynamo::to_item(item)?,
        };

        state.extend(map);

        store
            .try_save_encounter(user_id, game_id, encounter, level, round, state)
            .await
    }
}
