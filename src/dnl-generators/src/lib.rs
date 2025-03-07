pub mod battle;
pub mod shop;
pub mod rest;

use std::marker::PhantomData;
use std::collections::HashMap;

use dnl_store::Store;
use dnl_sort_keys::encounter::EncounterSortKey;
use dnl_types::scenario::ScenarioInput;
use dnl_types::llm::ParseConverseOutput;
use dnl_types::llm::LlmHandler;
use dnl_types::tools::Tools;
use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::shop::ShopToolOutput;
use dnl_types::tools::rest::RestToolOutput;
use battle::{BattleGenerator, BattleJsonGeneratorConfig};
use shop::{ShopGenerator, ShopJsonGeneratorConfig};
use rest::{RestGenerator, RestJsonGeneratorConfig};

use anyhow::Context;
use aws_sdk_bedrockruntime::types::builders::*;
use lambda_http::tracing::info;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::DeserializeOwned;

pub trait Generator {
    fn generate(&self) -> String;
}

pub enum GeneratorConfig {
    Battle(BattleJsonGeneratorConfig),
    Shop(ShopJsonGeneratorConfig),
    Rest(RestJsonGeneratorConfig),
}

pub enum JsonGenerator {
    Battle(BattleGenerator),
    Shop(ShopGenerator),
    Rest(RestGenerator),
}

#[derive(Clone)]
pub enum ToolOutput {
    Battle(BattleToolOutput),
    Shop(ShopToolOutput),
    Rest(RestToolOutput),
}

impl Serialize for ToolOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ToolOutput::Battle(inner) => inner.serialize(serializer),
            ToolOutput::Shop(inner) => inner.serialize(serializer),
            ToolOutput::Rest(inner) => inner.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ToolOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: BattleToolOutput = Deserialize::deserialize(deserializer)?;
        Ok(ToolOutput::Battle(inner))
    }
}

impl JsonGenerator 
    {
    pub async fn generate_text(&mut self) -> anyhow::Result<()> {
        match self {
            JsonGenerator::Battle(gen) => gen.generate_text().await,
            JsonGenerator::Shop(gen) => gen.generate_text().await,
            JsonGenerator::Rest(gen) => gen.generate_text().await,
        }
    }
    pub async fn generate_json(&mut self) -> anyhow::Result<()> {
        match self {
            JsonGenerator::Battle(gen) => gen.generate_json().await,
            JsonGenerator::Shop(gen) => gen.generate_json().await,
            JsonGenerator::Rest(gen) => gen.generate_json().await,
        }
    }
    pub async fn save_json(&mut self) -> anyhow::Result<()> {
        match self {
            JsonGenerator::Battle(gen) => gen.save_json().await,
            JsonGenerator::Shop(gen) => gen.save_json().await,
            JsonGenerator::Rest(gen) => gen.save_json().await,
        }
    }
    pub fn data(&mut self) -> Option<ToolOutput> {
        match self {
            JsonGenerator::Battle(gen) => gen.data(),
            JsonGenerator::Shop(gen) => gen.data(),
            JsonGenerator::Rest(gen) => gen.data(),
        }
    }
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
    fn scenario_input(&self) -> ScenarioInput;
    fn json_prompt(&self) -> String;
    fn model(&self) -> String;
    fn schema(&self) -> serde_json::Value;
    fn system_prompt(&self) -> String;
    fn text_prompt(&self) -> String;
    fn tool(&self) -> Tools;
}

pub struct JsonResponseGenerator<C: JsonResponseGeneratorConfig, D: DeserializeOwned + Serialize> {
    pub config: C,
    pub context: Option<Vec<String>>,
    pub model: LlmHandler,
    pub text: Option<String>,
    pub data: Option<ToolOutput>,
    pub max_retries: u8,
    pub attempts: u8,
    _phantom_data: PhantomData<D>,
}

impl<C, D> JsonResponseGenerator<C, D> 
where 
    C: JsonResponseGeneratorConfig,
    D: DeserializeOwned + Serialize + Clone
{
    pub fn data(&mut self) -> Option<ToolOutput> {
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
            _phantom_data: PhantomData,
        }
    }

    pub async fn generate_text(&mut self) -> anyhow::Result<()> {
        let store = Store::new().await;
        // mpQtCe0qleQ#Game#Level#5#Encounter#Battle#Round#6
        let scenario_input = self.config.scenario_input();
        let user_id = scenario_input.user_id; 
        let game_id = scenario_input.game_id;
        let level = scenario_input.level;
        let scenario = scenario_input.scenario;
        let resp = store.try_get_encounters(&user_id, &game_id, &level, &scenario).await.context("try_get_encounters failed")?;
        
        let ctxs: Vec<String> = resp.iter().map(|v| format!("name: {}\ntext: {}\n\n", v.name, v.text)).collect();
        
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
    
    pub async fn generate_json(&mut self) -> anyhow::Result<()>
    {
        let mut ctxs: Vec<String> = vec![];
        loop {
            self.attempts += 1;

            match self.to_json(ctxs.clone()).await {
                Ok(data) => {
                    self.data = Some(data);
                    return Ok(());
                }
                Err(err) => {
                    if self.attempts >= self.max_retries {
                        info!("Reached max retry limit of {}", self.max_retries);
                        return Err(anyhow::anyhow!("Exceeded max retry limit of {} when trying to create json", self.max_retries))
                    }

                    info!("Retrying creating JSON output");
                    info!("Attempt {} failed: {}", self.attempts, err);

                    let ctx = format!(
                        "The previous attempt to deserialize your response failed with the error: {}",
                        err.to_string()
                    );
                    ctxs.push(ctx)
                }
            }
        };
    }

    pub async fn to_json(&mut self, ctxs: Vec<String>) -> anyhow::Result<ToolOutput> {
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
            .await.context("Failed to call converse api")?;

        let tool_output = res.get_tool_output().context("Failed to get tool output")?;

        let tool_value = tool_output
            .first()
            .expect("Missing tool output, this should not happen")
            .input
            .clone();

        let value = serde_json::to_value(tool_value).context("Failed to parese tool value to serde_json::Value")?;

        match self.config.tool() {
            Tools::Shop => {
                serde_json::from_value::<ShopToolOutput>(value)
                    .map(ToolOutput::Shop)
                    .map_err(Into::into) // Convert serde_json error into your custom error type
            }
            Tools::Battle => {
                serde_json::from_value::<BattleToolOutput>(value)
                    .map(ToolOutput::Battle)
                    .map_err(Into::into)
            }
            Tools::Rest => {
                serde_json::from_value::<RestToolOutput>(value)
                    .map(ToolOutput::Rest)
                    .map_err(Into::into)
            }
        }
        // let output = serde_json::from_value(value);
        // match output {
        //     Ok(output) =>  match &self.config.tool() {
        //         Tools::Shop => Ok(ToolOutput::Shop(output)),
        //         Tools::Battle =>Ok(ToolOutput::Battle(output)),
        //         Tools::Rest => Ok(ToolOutput::Rest(output)),
        //     }
        //     Err(err) => Err(err.into()),
        // }
    }
    
    
    pub async fn save_json(&mut self) -> anyhow::Result<()> {
        let store = Store::new().await;

        let encounter = EncounterSortKey::Battle;
        
        let scenario_input = self.config.scenario_input();
        
        let level = scenario_input.level.parse::<u8>()?;
        let round = scenario_input.round.parse::<u8>()?;
        let user_id = &scenario_input.user_id;
        let game_id = &scenario_input.game_id;
 
        let mut state = HashMap::new();
        
        state.insert("text".to_string(), aws_sdk_dynamodb::types::AttributeValue::S(self.text.clone().unwrap_or("".to_string())));
        
        let data = self.data.clone().unwrap();
        
        let map: HashMap<String, aws_sdk_dynamodb::types::AttributeValue> = match data {
            ToolOutput::Battle(item) => {
                serde_dynamo::to_item(item)?
            }
            ToolOutput::Shop(item) => {
                serde_dynamo::to_item(item)?
            }
            ToolOutput::Rest(item) => {
                serde_dynamo::to_item(item)?
            }
        };
        
        state.extend(map);
        
        store
            .try_save_encounter(
                user_id,
                game_id,
                encounter,
                level,
                round,
                state,
            )
            .await
    }
}
