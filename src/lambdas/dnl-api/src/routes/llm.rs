use crate::error::ApiError;
use dnl_llm::llm::{BattleToolResponse, LlmConverseInput, ScenarioInput};
use anyhow::Context;
use aws_sdk_bedrockruntime::types::builders::InferenceConfigurationBuilder;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::{routing::post, Router};
use axum_macros::debug_handler;
use dnl_generators::battle::BattleGenerator;
use dnl_generators::battle::BattleJsonGeneratorConfig;
use dnl_generators::JsonResponseGenerator;
use dnl_llm::llm::*;
use dnl_llm::tool::*;
use dnl_generators::Scenario;
use lambda_http::tracing::info;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;

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

    let system_vars = HashMap::new();
    let mut text_vars = HashMap::new();
    let mut json_vars = HashMap::new();
    
    let example = match Scenario::from_str(&payload.scenario)? {
        Scenario::Battle => serde_json::to_string(&BattleToolOutput::mock())?,
        Scenario::Shop => serde_json::to_string(&BattleToolOutput::mock())?,
        Scenario::Rest => serde_json::to_string(&RestToolOutput::mock())?,
    };

    text_vars.insert("theme".to_string(), payload.theme.clone());

    json_vars.insert("level".to_string(), payload.level.clone());
    json_vars.insert("example".to_string(), example);

    let config = BattleJsonGeneratorConfig::new(payload, system_vars, text_vars, json_vars);

    let mut generator: BattleGenerator = JsonResponseGenerator::new(config).await;

    generator.generate_text().await?;
    info!("Text Created");

    let response = generator.generate_json().await?;
    info!("Json Created");

    if let Some(data) = response {
        generator.save_json(&data).await?;
        
        info!("Response: {:?}", data);
        
        let res =Some((StatusCode::OK, Json(json!({"data": data})))).expect("this should never happen").into_response();
        return Ok(res)
    } else {
        // this could potential be a cache fetch for a previous successful response.
        let err = format!("Reached max retry limit of {}", generator.max_retries);
        let json = Json(json!({"error": err}));
        let res = (StatusCode::SERVICE_UNAVAILABLE, json).into_response();
        return Ok(res);
    }
}
