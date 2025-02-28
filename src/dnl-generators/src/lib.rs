pub mod battle;
use std::marker::PhantomData;
use dnl_store::Store;
use dnl_sort_keys::encounter::EncounterSortKey;
use dnl_llm::llm::ScenarioInput;

use aws_sdk_bedrockruntime::types::builders::*;
use dnl_llm::llm::ParseConverseOuput;
use dnl_llm::llm::LlmHandler;
use dnl_llm::tool::Tools;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use lambda_http::tracing::info;
use serde_json::json;
use serde::Serialize;
use serde::Deserialize;
use strum::EnumString;

#[derive(Debug, Deserialize, Serialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Scenario {
    Battle,
    Shop,
    Rest
}

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
    fn scenario_input(&self) -> ScenarioInput;
    fn json_prompt(&self) -> String;
    fn model(&self) -> String;
    fn schema(&self) -> serde_json::Value;
    fn system_prompt(&self) -> String;
    fn text_prompt(&self) -> String;
    fn tool(&self) -> Tools;
}

#[derive(Debug)]
pub struct JsonResponseGenerator<C: JsonResponseGeneratorConfig, D:DeserializeOwned + Serialize> {
    pub config: C,
    pub context: Option<Vec<String>>,
    pub model: LlmHandler,
    pub text: Option<String>,
    pub max_retries: u8,
    pub attempts: u8,
    _phantom_data: PhantomData<D>,
}

impl<C, D> JsonResponseGenerator<C, D> 
where 
    C: JsonResponseGeneratorConfig,
    D: DeserializeOwned + Serialize
{
    pub async fn new(config: C) -> Self {
        let model = LlmHandler::new(config.model(), config.system_prompt()).await;

        let context = None;
        let text = None;
        let max_retries = 5;
        let attempts = 0;

        Self {
            config,
            context,
            model,
            text,
            max_retries,
            attempts,
            _phantom_data: PhantomData
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
        let resp = store.try_get_encounters(&user_id, &game_id, &level, &scenario).await?;
        
        let ctxs: Vec<String> = resp.iter().map(|v| format!("name: {}\ntext: {}\n\n", v.state.data.name, v.state.text)).collect();
        
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

        let text = res.get_text_output()?;

        self.text = Some(text);

        Ok(())
    }
    
    pub async fn generate_json(&mut self) -> anyhow::Result<Option<D>>
    {
        let mut ctxs: Vec<String> = vec![];
        let response = loop {
            self.attempts += 1;

            match self.to_json(ctxs.clone()).await {
                Ok(data) => {
                    break Some(data);
                }
                Err(err) => {
                    // early return if max_retries exceeded
                    if self.attempts >= self.max_retries {
                        info!("Reached max retry limit of {}", self.max_retries);
                        break None;
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
        Ok(response)
    }

    pub async fn to_json(&mut self, ctxs: Vec<String>) -> anyhow::Result<D>
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

        match serde_json::from_value(value) {
            Ok(output) =>  Ok(output),
            Err(err) => Err(err.into()),
        }
    }
    
    
    pub async fn save_json(&mut self, data: &serde_json::Value) -> anyhow::Result<()> {
        let store = Store::new().await;

        let value = json!({"text": self.text, "data": data});

        let state = serde_json::to_value(value)?;

        let encounter = EncounterSortKey::Battle;
        
        let scenario_input = self.config.scenario_input();
        
        let level = scenario_input.level.parse::<u8>()?;
        let round = scenario_input.round.parse::<u8>()?;
        let user_id = &scenario_input.user_id;
        let game_id = &scenario_input.game_id;

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
