use crate::error::{handle_error, ApiError};

use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use dnl_db::*;
use dnl_generators::JsonResponseGenerator;
use dnl_generators::*;
use dnl_llm::embedding::{EmbeddingConfig, EmbeddingEngine};
use dnl_store::Store;
use dnl_types::generators::*;
use dnl_types::llm::*;
use dnl_types::scenarios::*;
use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::*;
use scenarios::battle::{BATTLE_JSON_PROMPT, BATTLE_SYSTEM_PROMPT, BATTLE_TEXT_PROMPT};
use scenarios::rest::{REST_JSON_PROMPT, REST_SYSTEM_PROMPT, REST_TEXT_PROMPT};
use scenarios::shop::{SHOP_JSON_PROMPT, SHOP_SYSTEM_PROMPT, SHOP_TEXT_PROMPT};

use anyhow::Context;
use aws_sdk_bedrockruntime::types::builders::InferenceConfigurationBuilder;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::{routing::post, Router};
use axum_macros::debug_handler;
use lambda_http::tracing::info;
use serde_json::json;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ScenarioRequest {
    pub user_id: String,
    pub generator: GeneratorType,
}

pub fn llm_router() -> Router {
    let router: Router = Router::new()
        .route("/converse", post(post_llm_converse))
        .route("/scenario", post(post_scenario));
    Router::new().nest("/llm", router)
}

#[debug_handler]
pub async fn post_llm_converse(
    Json(payload): Json<LlmConverseInput>,
) -> Result<Response, ApiError> {
    println!("Payload: {:?}", payload);

    let mut llm = LlmHandler::new(payload.model, payload.system).await;

    let input = llm.create_prompt(None, &payload.prompt);

    let message = llm.create_user_message(&input);

    llm.set_messages(vec![message]);

    let cfg = Some(
        InferenceConfigurationBuilder::default()
            .temperature(1.0)
            .build(),
    );

    let con_res = llm.converse(Some(Tool::Battle), cfg).await;
    info!("CON-RES: {:?}", con_res);

    match con_res?.get_tool_output() {
        Ok(tool) => {
            let status = StatusCode::OK;
            let input = &tool[0].input;
            let value = serde_json::to_value(input)?;
            info!("Value: {}", value);

            let res: BattleToolOutput = serde_json::from_value(value.clone())
                .context("Could not convert to BattleToolOutput")?;
            info!("BTR: {:?}", res);

            let json = Json(json!({"detail": res}));

            info!("Response: {:?}", json);
            Ok((status, json).into_response())
        }
        Err(e) => {
            let status = StatusCode::BAD_REQUEST;
            let json = Json(json!({"error": format!("{}",e)}));
            Ok((status, json).into_response())
        }
    }
}

#[debug_handler]
pub async fn post_scenario(Json(payload): Json<ScenarioRequest>) -> Result<Response, ApiError> {
    info!("Payload: {:?}", payload);

    // let system_vars = HashMap::new();
    // let mut text_vars = HashMap::new();
    // let mut json_vars = HashMap::new();

    // let scenario = Scenario::from_str(&payload.scenario)?;

    //     ToolOutput::from_str(&scenario.to_string()).context("Unable to match scenario")?;
    // let tool_output =

    let tool_config = match &payload.generator {
        GeneratorType::Scenario(config) => match config {
            GeneratorScenarioConfig::Battle => GeneratorToolConfig { tool: Tool::Battle },
            GeneratorScenarioConfig::Shop => GeneratorToolConfig { tool: Tool::Shop },
            GeneratorScenarioConfig::Rest => GeneratorToolConfig { tool: Tool::Rest },
        },
        GeneratorType::Object(config) => match config {
            GeneratorObjectConfig::Weapon => GeneratorToolConfig { tool: Tool::Battle },
            GeneratorObjectConfig::Spell => GeneratorToolConfig { tool: Tool::Battle },
        },
    };

    let prompt_config = match &payload.generator {
        GeneratorType::Scenario(config) => match config {
            GeneratorScenarioConfig::Battle => GeneratorPromptConfig {
                system: BATTLE_SYSTEM_PROMPT.to_string(),
                text: Some(BATTLE_TEXT_PROMPT.to_string()),
                json: BATTLE_JSON_PROMPT.to_string(),
            },
            GeneratorScenarioConfig::Shop => GeneratorPromptConfig {
                system: SHOP_SYSTEM_PROMPT.to_string(),
                text: Some(SHOP_TEXT_PROMPT.to_string()),
                json: SHOP_JSON_PROMPT.to_string(),
            },
            GeneratorScenarioConfig::Rest => GeneratorPromptConfig {
                system: REST_SYSTEM_PROMPT.to_string(),
                text: Some(REST_TEXT_PROMPT.to_string()),
                json: REST_JSON_PROMPT.to_string(),
            },
        },
        GeneratorType::Object(config) => match config {
            GeneratorObjectConfig::Weapon => GeneratorPromptConfig {
                system: BATTLE_SYSTEM_PROMPT.to_string(),
                text: Some(BATTLE_TEXT_PROMPT.to_string()),
                json: BATTLE_JSON_PROMPT.to_string(),
            },
            GeneratorObjectConfig::Spell => GeneratorPromptConfig {
                system: BATTLE_SYSTEM_PROMPT.to_string(),
                text: Some(BATTLE_TEXT_PROMPT.to_string()),
                json: BATTLE_JSON_PROMPT.to_string(),
            },
        },
    };

    // let example = serde_json::to_string(&tool_output.mock())?;

    // json_vars.insert("example".to_string(), example);

    info!("HERE4");

    let mut generator = JsonResponseGenerator::new(
        &payload.user_id,
        payload.generator.clone(),
        tool_config,
        prompt_config,
    )
    .await?;

    let text_res = generator
        .generate_text()
        .await
        .context("Failed to generate text");

    match text_res {
        Ok(_) => {
            info!("Text Created");
        }
        Err(err) => return Ok(handle_error(format!("{:?}", err.to_string()))),
    };

    let json_res = generator.generate_json::<ToolOutput>().await;

    match json_res {
        Ok(output) => {
            let data = ToolOutputModel::from(output);

            generator
                .save_json(data.clone())
                .await
                .context("Failed to save_json")?;

            info!("DATA: {:?}", data);

            let value = serde_json::to_value(data)?;

            info!("Value: {:?}", value);

            let store = Store::new(&payload.user_id).await;
            let game_id = store.try_get_active_game_id().await?;

            if let Some(text) = generator.text {
                let mut embedding_engine = EmbeddingEngine::new(EmbeddingConfig::default()).await;

                let db = VectorDatabase::new();

                let vector = embedding_engine.try_create_vector(&text).await?;

                let embedding = models::NewEmbedding {
                    user_id: &payload.user_id,
                    game_id: &game_id,
                    vector,
                    text: &text,
                    type_: "output".to_string(),
                };

                let db_res = db
                    .await
                    .try_insert_new_embedding(&embedding)
                    .await
                    .context("failed to insert embedding")?;

                info!("Database NewEmbedding response: {:?}", db_res);
            }

            let mut state = HashMap::new();

            //:TODO: this needs conditional logic on updating the current gamestate
            if generator.gen_type.is_scenario() {
                let game_state = store.try_get_state().await?;
                let round = game_state
                    .round
                    .expect("GameState.round should not be None");
                let level = game_state
                    .level
                    .expect("GameState.level should not be None");

                state.insert("round".to_string(), AttributeValue::N(round.to_string()));
                state.insert("level".to_string(), AttributeValue::N(level.to_string()));

                match payload.generator {
                    GeneratorType::Scenario(config) => match config {
                        GeneratorScenarioConfig::Battle => {
                            state.insert(
                                "curr_encounter".to_string(),
                                AttributeValue::S(Scenario::Battle.to_string()),
                            );
                        }
                        GeneratorScenarioConfig::Shop => {
                            state.insert(
                                "curr_encounter".to_string(),
                                AttributeValue::S(Scenario::Shop.to_string()),
                            );
                        }
                        GeneratorScenarioConfig::Rest => {
                            state.insert(
                                "curr_encounter".to_string(),
                                AttributeValue::S(Scenario::Rest.to_string()),
                            );
                        }
                    },
                    _ => (),
                }
            };

            let _ = store.try_update_state(state).await?;

            let res = (StatusCode::OK, Json(value)).into_response();
            info!("Response: {:?}", res);

            return Ok(res);
        }

        Err(err) => return Ok(handle_error(format!("{:?}", err.to_string()))),
    };
}
