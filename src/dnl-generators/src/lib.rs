pub mod prompt;
pub mod scenarios;
pub mod tools;

use std::collections::HashMap;
use std::str::FromStr;

use dnl_sort_keys::encounter::EncounterSortKey;
use dnl_store::Store;
use dnl_types::generators::*;
use dnl_types::llm::LlmHandler;
use dnl_types::llm::ParseConverseOutput;

use anyhow::anyhow;
use anyhow::Context;
use aws_sdk_bedrockruntime::types::builders::*;
use lambda_http::tracing::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct JsonResponseGenerator {
    pub user_id: String,
    pub gen_type: GeneratorType,
    pub llm: LlmHandler,

    pub text: Option<String>, // text output from llm after calling generate_text

    pub max_retries: u8,
    pub attempts: u8,

    pub store: Store,

    pub tool_config: GeneratorToolConfig,
    pub prompt_config: GeneratorPromptConfig,
}

impl JsonResponseGenerator {
    pub async fn new(
        user_id: &str,
        gen_type: GeneratorType,
        tool_config: GeneratorToolConfig,
        prompt_config: GeneratorPromptConfig,
    ) -> anyhow::Result<Self> {
        let store = Store::new(user_id).await;

        let model = store.try_get_llm_model().await?;

        let llm = LlmHandler::new(model, prompt_config.system.format()).await;

        let text = None;

        let max_retries = 5;
        let attempts = 0;

        Ok(Self {
            gen_type,
            user_id: user_id.to_string(),
            llm,
            text,
            max_retries,
            attempts,
            prompt_config,
            tool_config,
            store,
        })
    }

    // :TODO: this will have to be custom logic based on the GeneratorConfig
    pub async fn generate_text(&mut self) -> anyhow::Result<()> {
        info!("GEN TEXT 1");

        let scenario_config = match &self.gen_type {
            GeneratorType::Scenario(config) => Some(config),
            _ => None,
        }
        .expect("GeneratorType should be a scenario if calling generate_text");

        info!("GEN TEXT 2");
        info!("Scenario Config: {:?}", scenario_config);

        // :TODO: this should probably be made into a get_text_context function
        let resp = match scenario_config {
            &GeneratorScenarioConfig::Battle => {
                let state = self.store.try_get_state().await?;

                let game_id = self.store.try_get_active_game_id().await?;

                let level = &state.level.expect("state.level should be set").to_string();
                info!("Battle1");

                // THIS DOES NOT HANDLE NEW GAME
                let encounter = EncounterSortKey::from_str(&state.curr_encounter.unwrap())
                    .context("Encounter Sort Key variant not found")?;

                self.store
                    .try_get_encounters(&game_id, level, encounter)
                    .await
                    .context("try_get_encounters failed")?
            }
            GeneratorScenarioConfig::Shop => {
                let state = self.store.try_get_state().await?;

                let game_id = self.store.try_get_active_game_id().await?;

                let level = &state.level.unwrap().to_string();

                let encounter = EncounterSortKey::from_str(&state.curr_encounter.unwrap())?;

                self.store
                    .try_get_encounters(&game_id, level, encounter)
                    .await
                    .context("try_get_encounters failed")?
            }
            GeneratorScenarioConfig::Rest => {
                let state = self.store.try_get_state().await?;

                let game_id = self.store.try_get_active_game_id().await?;

                let level = &state.level.unwrap().to_string();

                let encounter = EncounterSortKey::from_str(&state.curr_encounter.unwrap())?;

                self.store
                    .try_get_encounters(&game_id, level, encounter)
                    .await
                    .context("try_get_encounters failed")?
            }
        };
        info!("GEN TEXT 3");

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

        let text_prompt = &self.prompt_config.clone().text.expect("Expected a prompt text, but found None. Ensure that prompt_config.text is set before calling generate_text.");

        let prompt = self.llm.create_prompt(Some(ctxs), &text_prompt.format());

        let message = self.llm.create_user_message(&prompt);

        self.llm.set_messages(vec![message]);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(1.0)
            .top_p(1.0)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        let res = self.llm.converse(None, inference_cfg).await?;

        let text = res.get_text_output().context("get_text_output failed")?;

        self.text = Some(text);

        Ok(())
    }

    pub async fn generate_json<T: Serialize + for<'a> Deserialize<'a>>(
        &mut self,
    ) -> anyhow::Result<T> {
        let mut ctxs: Vec<String> = vec![];

        loop {
            self.attempts += 1;

            match self.to_json::<T>(ctxs.clone()).await {
                Ok(data) => {
                    return Ok(data);
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

                    let fail_ctx = format!(
                        "The previous attempt to deserialize your response failed with the error: {}",
                        err.to_string()
                    );
                    ctxs.push(fail_ctx);
                }
            }
        }
    }

    pub async fn to_json<T: Serialize + for<'a> Deserialize<'a>>(
        &mut self,
        ctxs: Vec<String>,
    ) -> anyhow::Result<T> {
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

        let input = self
            .llm
            .create_prompt(Some(contexts), &self.prompt_config.json.format());

        let message = self.llm.create_user_message(&input);

        self.llm.set_messages(vec![message]);

        let inference_cfg_builder = InferenceConfigurationBuilder::default()
            .temperature(0.3)
            .top_p(0.8)
            .max_tokens(1000)
            .build();

        let inference_cfg = Some(inference_cfg_builder);

        info!("TOOL: {:?}", self.tool_config.tool);

        let res = self
            .llm
            .converse(Some(self.tool_config.tool), inference_cfg)
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

        info!("VALUE: {:?}", value);

        match serde_json::from_value::<T>(value) {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    pub async fn save_json<T: Serialize + for<'a> Deserialize<'a>>(
        &mut self,
        value: T,
    ) -> anyhow::Result<()> {
        let mut state = HashMap::new();

        let map: HashMap<String, aws_sdk_dynamodb::types::AttributeValue> =
            serde_dynamo::to_item(value)?;

        state.extend(map);

        if let Some(text) = self.text.clone() {
            state.insert(
                "text".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(text),
            );
        }

        let game_id = self.store.try_get_active_game_id().await?;

        let game_state = self.store.try_get_state().await?;

        let level = game_state
            .level
            .expect("GameState.level should not be None")
            .parse()?;

        let round = game_state
            .round
            .expect("GameState.round should not be None")
            .parse()?;

        let curr_encounter = game_state
            .curr_encounter
            .expect("GameState.curr_encounter should not be None");

        let encounter = EncounterSortKey::from_str(&curr_encounter)?;

        let _ = self
            .store
            .try_save_encounter(&game_id, encounter, level, round, state)
            .await;

        Ok(())
    }
}
