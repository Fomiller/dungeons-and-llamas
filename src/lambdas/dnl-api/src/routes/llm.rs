use crate::error::{handle_error, ApiError};

use std::collections::HashMap;
use std::str::FromStr;

use aws_sdk_dynamodb::types::AttributeValue;
use dnl_db::*;
use dnl_generators::battle::BattleJsonGeneratorConfig;
use dnl_generators::rest::RestJsonGeneratorConfig;
use dnl_generators::shop::ShopJsonGeneratorConfig;
use dnl_generators::{JsonGeneratorConfigEnum, JsonGeneratorEnum, JsonResponseGenerator};
use dnl_llm::embedding::{EmbeddingConfig, EmbeddingEngine};
use dnl_store::Store;
use dnl_types::llm::*;
use dnl_types::scenarios::*;
use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::rest::RestToolOutput;
use dnl_types::tools::shop::ShopToolOutput;
use dnl_types::tools::*;

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

    let con_res = llm.converse(Some(Tools::Battle), cfg).await;
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
pub async fn post_scenario(Json(payload): Json<ScenarioInput>) -> Result<Response, ApiError> {
    info!("Payload: {:?}", payload);

    let system_vars = HashMap::new();
    let mut text_vars = HashMap::new();
    let mut json_vars = HashMap::new();

    let scenario = Scenario::from_str(&payload.scenario).context("Unable to match scenario")?;

    let example = match scenario {
        Scenario::Battle => serde_json::to_string(&BattleToolOutput::mock())?,
        Scenario::Shop => serde_json::to_string(&ShopToolOutput::mock())?,
        Scenario::Rest => serde_json::to_string(&RestToolOutput::mock())?,
    };

    text_vars.insert("theme".to_string(), payload.theme.clone());
    json_vars.insert("level".to_string(), payload.level.clone());
    json_vars.insert("example".to_string(), example);

    let config = match Scenario::from_str(&payload.scenario).context("Unable to match scenario")? {
        Scenario::Battle => JsonGeneratorConfigEnum::Battle(BattleJsonGeneratorConfig::new(
            payload.clone(),
            system_vars,
            text_vars,
            json_vars,
        )),
        Scenario::Shop => JsonGeneratorConfigEnum::Shop(ShopJsonGeneratorConfig::new(
            payload.clone(),
            system_vars,
            text_vars,
            json_vars,
        )),
        Scenario::Rest => JsonGeneratorConfigEnum::Rest(RestJsonGeneratorConfig::new(
            payload.clone(),
            system_vars,
            text_vars,
            json_vars,
        )),
    };

    let mut generator = match config {
        JsonGeneratorConfigEnum::Battle(config) => {
            JsonGeneratorEnum::Battle(JsonResponseGenerator::new(config).await)
        }
        JsonGeneratorConfigEnum::Shop(config) => {
            JsonGeneratorEnum::Shop(JsonResponseGenerator::new(config).await)
        }
        JsonGeneratorConfigEnum::Rest(config) => {
            JsonGeneratorEnum::Rest(JsonResponseGenerator::new(config).await)
        }
    };

    match generator
        .generate_text()
        .await
        .context("Failed to generate text")
    {
        Ok(_) => {
            info!("Text Created");
        }
        Err(err) => return Ok(handle_error(format!("{:?}", err.to_string()))),
    };

    match generator
        .generate_json()
        .await
        .context("Failed to generate json")
    {
        Ok(_) => {
            generator.save_json().await.context("Failed to save_json")?;

            let data = generator.data().unwrap().clone();
            info!("DATA: {:?}", data);

            let value = serde_json::to_value(data)?;
            info!("Value: {:?}", value);

            let mut embedding_engine = EmbeddingEngine::new(EmbeddingConfig::default()).await;

            let db = VectorDatabase::new();

            let text = generator.text().unwrap().clone();
            let vector = embedding_engine.try_create_vector(&text).await?;

            let embedding = models::NewEmbedding {
                user_id: &payload.user_id,
                game_id: &payload.game_id,
                vector,
                text: &text,
                type_: "output".to_string(),
            };

            let store = Store::new().await;

            let mut state = HashMap::new();
            state.insert("round".to_string(), AttributeValue::N(payload.round));
            state.insert("level".to_string(), AttributeValue::N(payload.level));
            state.insert(
                "curr_encounter".to_string(),
                AttributeValue::S(payload.scenario),
            );

            let _ = store.try_update_state(&payload.user_id, state).await?;

            let db_res = db
                .await
                .try_insert_new_embedding(&embedding)
                .await
                .context("failed to insert embedding")?;

            info!("Database NewEmbedding response: {:?}", db_res);

            let res = (StatusCode::OK, Json(value)).into_response();
            info!("Response: {:?}", res);

            return Ok(res);
        }
        Err(err) => return Ok(handle_error(format!("{:?}", err.to_string()))),
    };
}
