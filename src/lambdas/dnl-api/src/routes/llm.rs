use crate::error::ApiError;

use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use dnl_db::*;
use dnl_generators::JsonResponseGenerator;
use dnl_generators::*;
use dnl_llm::embedding::{EmbeddingConfig, EmbeddingEngine};
use dnl_store::Store;
use dnl_types::api::request::ScenarioRequest;
use dnl_types::generators::prompt::Prompt;
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
pub async fn post_scenario(Json(payload): Json<ScenarioRequest>) -> impl IntoResponse {
    info!("Payload: {:?}", payload);

    let store = Store::new(&payload.user_id).await;

    info!("HERE 0");

    let game_state = match store.try_get_state().await {
        Ok(store) => store,
        Err(e) => return ApiError(e).into_response(),
    };

    info!("HERE 00");

    let settings = match store.try_get_settings().await {
        Ok(settings) => settings,
        Err(e) => return ApiError(e).into_response(),
    };

    info!("HERE 1");

    let system_vars = HashMap::new();
    let mut text_vars = HashMap::new();
    let mut json_vars = HashMap::new();

    text_vars.insert(
        "theme".to_string(),
        settings
            .theme
            .clone()
            .expect("settings.theme should not be None"),
    );

    json_vars.insert(
        "level".to_string(),
        game_state
            .level
            .clone()
            .expect("game_state.level should not be None")
            .to_string(),
    );

    json_vars.insert(
        "round".to_string(),
        game_state
            .round
            .clone()
            .expect("game_state.round should not be None")
            .to_string(),
    );

    info!("HERE 2");

    let example = match &payload.generator {
        GeneratorType::Scenario(config) => match config {
            GeneratorScenarioConfig::Battle => ToolOutput::mock_battle(),
            GeneratorScenarioConfig::Shop => ToolOutput::mock_shop(),
            GeneratorScenarioConfig::Rest => ToolOutput::mock_rest(),
        },
        GeneratorType::Object(config) => match config {
            GeneratorObjectConfig::Weapon => ToolOutput::mock_battle(),
            GeneratorObjectConfig::Spell => ToolOutput::mock_battle(),
        },
    };

    match serde_json::to_string(&example) {
        Ok(example) => json_vars.insert("example".to_string(), example),
        Err(e) => return ApiError(e.into()).into_response(),
    };

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

    info!("HERE 3");

    let prompt_config = match &payload.generator {
        GeneratorType::Scenario(config) => match config {
            GeneratorScenarioConfig::Battle => GeneratorPromptConfig {
                system: Prompt {
                    text: BATTLE_SYSTEM_PROMPT.to_string(),
                    variables: system_vars,
                },
                text: Some(Prompt {
                    text: BATTLE_TEXT_PROMPT.to_string(),
                    variables: text_vars,
                }),
                json: Prompt {
                    text: BATTLE_JSON_PROMPT.to_string(),
                    variables: json_vars,
                },
            },
            GeneratorScenarioConfig::Shop => GeneratorPromptConfig {
                system: Prompt {
                    text: SHOP_SYSTEM_PROMPT.to_string(),
                    variables: system_vars,
                },
                text: Some(Prompt {
                    text: SHOP_TEXT_PROMPT.to_string(),
                    variables: text_vars,
                }),
                json: Prompt {
                    text: SHOP_JSON_PROMPT.to_string(),
                    variables: json_vars,
                },
            },
            GeneratorScenarioConfig::Rest => GeneratorPromptConfig {
                system: Prompt {
                    text: REST_SYSTEM_PROMPT.to_string(),
                    variables: system_vars,
                },
                text: Some(Prompt {
                    text: REST_TEXT_PROMPT.to_string(),
                    variables: text_vars,
                }),
                json: Prompt {
                    text: REST_JSON_PROMPT.to_string(),
                    variables: json_vars,
                },
            },
        },

        //:TODO: create unique prompts for these
        GeneratorType::Object(config) => match config {
            GeneratorObjectConfig::Weapon => GeneratorPromptConfig {
                system: Prompt {
                    text: BATTLE_SYSTEM_PROMPT.to_string(),
                    variables: system_vars,
                },
                text: Some(Prompt {
                    text: BATTLE_TEXT_PROMPT.to_string(),
                    variables: text_vars,
                }),
                json: Prompt {
                    text: BATTLE_JSON_PROMPT.to_string(),
                    variables: json_vars,
                },
            },
            GeneratorObjectConfig::Spell => GeneratorPromptConfig {
                system: Prompt {
                    text: BATTLE_SYSTEM_PROMPT.to_string(),
                    variables: system_vars,
                },
                text: Some(Prompt {
                    text: BATTLE_TEXT_PROMPT.to_string(),
                    variables: text_vars,
                }),
                json: Prompt {
                    text: BATTLE_JSON_PROMPT.to_string(),
                    variables: json_vars,
                },
            },
        },
    };

    info!("HERE4");

    let mut generator = match JsonResponseGenerator::new(
        &payload.user_id,
        payload.generator.clone(),
        tool_config,
        prompt_config,
    )
    .await
    {
        Ok(generator) => generator,
        Err(e) => return ApiError(e.into()).into_response(),
    };

    match generator.generate_text().await {
        Ok(_) => {
            info!("Text Created");
        }
        Err(e) => return ApiError(e).into_response(),
    };

    info!("HERE6");

    match generator.generate_json::<ToolOutput>().await {
        Ok(output) => {
            let data = ToolOutputModel::from(output);

            match generator
                .save_json(data.clone())
                .await
                .context("Failed to save_json")
            {
                Ok(_) => (),
                Err(e) => return ApiError(e).into_response(),
            };

            info!("DATA: {:?}", data);

            let value = match serde_json::to_value(data) {
                Ok(value) => value,
                Err(e) => return ApiError(e.into()).into_response(),
            };

            info!("Value: {:?}", value);

            let store = Store::new(&payload.user_id).await;

            let game_id = match store.try_get_active_game_id().await {
                Ok(game_id) => game_id,
                Err(e) => return ApiError(e).into_response(),
            };

            if let Some(text) = generator.text {
                let mut embedding_engine = EmbeddingEngine::new(EmbeddingConfig::default()).await;

                let db = VectorDatabase::new();

                let vector = match embedding_engine.try_create_vector(&text).await {
                    Ok(vector) => vector,
                    Err(e) => return ApiError(e).into_response(),
                };

                let embedding = models::NewEmbedding {
                    user_id: &payload.user_id,
                    game_id: &game_id,
                    vector,
                    text: &text,
                    type_: "output".to_string(),
                };

                let db_res = match db
                    .await
                    .try_insert_new_embedding(&embedding)
                    .await
                    .context("failed to insert embedding")
                {
                    Ok(res) => res,
                    Err(e) => return ApiError(e).into_response(),
                };

                info!("Database NewEmbedding response: {:?}", db_res);
            }

            let mut state = HashMap::new();

            //:TODO: this needs conditional logic on updating the current gamestate
            if generator.gen_type.is_scenario() {
                let game_state = match store.try_get_state().await {
                    Ok(game_state) => game_state,
                    Err(e) => return ApiError(e).into_response(),
                };

                let round = match game_state.round {
                    Some(round) => round,
                    None => {
                        let err = anyhow::anyhow!("GameState.round should not be None").into();
                        return ApiError(err).into_response();
                    }
                };

                let level = match game_state.level {
                    Some(level) => level,
                    None => {
                        let err = anyhow::anyhow!("GameState.level should not be None").into();
                        return ApiError(err).into_response();
                    }
                };

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

            let _ = match store.try_update_state(state).await {
                Ok(_) => (),
                Err(e) => return ApiError(e).into_response(),
            };

            let res = (StatusCode::OK, Json(value)).into_response();
            info!("Response: {:?}", res);
            res
        }

        Err(e) => return ApiError(e).into_response(),
    }
}
