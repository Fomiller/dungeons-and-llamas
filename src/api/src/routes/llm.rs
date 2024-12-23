use crate::error::ApiError;
use crate::models::llm::BattleToolResponse;
use crate::models::llm::LlmConverseInput;
use crate::models::llm::ScenarioInput;
use anyhow::Context;
use aws_sdk_bedrockruntime::types::builders::InferenceConfigurationBuilder;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::{routing::post, Router};
use axum_macros::debug_handler;
use game::generators::battle::BattleGenerator;
use lambda_http::tracing::info;
use llm::llm::*;
use llm::tool::*;
use serde_json::json;

#[allow(unused_imports)] // Only if warnings are related to unused imports.
use serde::Serialize;

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

    let mut llm = LlmHandler::new(payload.model, payload.system, payload.instructions).await;

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

            let res: BattleToolResponse = serde_json::from_value(value.clone())
                .context("Could not convert to BattleToolResponse")?;
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
    println!("Payload: {:?}", payload);

    let mut generator =
        BattleGenerator::new(payload.model, payload.system, 1, payload.theme, None).await?;

    generator.generate_scenario(payload.scenario_prompt).await?;
    info!("Scenario Created");

    generator.to_json(payload.json_prompt).await?;
    info!("JSON Created");

    let json = Json(json!({"detail": generator.output.unwrap()}));

    info!("Response: {:?}", json);
    Ok((StatusCode::OK, json).into_response())
}
