use dnl_types::traits::DiscordMsg;

use anyhow::Context;
use lambda_http::tracing::info;
use reqwest::Response;
use serde_json::Value;
use serenity::all::Http;
use serenity::builder::*;
use serenity::model::application::*;

#[async_trait::async_trait]
pub trait DiscordCmdResponse {
    async fn handle_sucessful_response<T>(
        res: Response,
    ) -> anyhow::Result<Option<CreateInteractionResponse>>
    where
        T: serde::de::DeserializeOwned,
    {
        let _output: T = res.json().await.context("Failed to parse API Response")?;

        let content = format!("Success");

        let message = CreateInteractionResponseMessage::new().content(content);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }

    async fn handle_error_response(
        res: Response,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let error: Value = res
            .json()
            .await
            .unwrap_or_else(|_| serde_json::json!({ "error": "Failed to parse JSON response in handle_error_response" }));

        let message = error
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| "Error unwrapping error message in handle_error_response")
            .to_string();

        let message = CreateInteractionResponseMessage::new().content(message);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }

    async fn handle_sucessful_response_with_followup<T>(
        res: Response,
        cmd: &CommandInteraction,
        http: &Http,
    ) -> anyhow::Result<()>
    where
        T: serde::de::DeserializeOwned + DiscordMsg + Send,
    {
        info!("API Res: {:?}", res);
        let output: T = res
            .json()
            .await
            .context(format!("Failed to parse ApiResponse"))?;

        let message = CreateInteractionResponseFollowup::new().content(output.to_message());

        let res = cmd.create_followup(&http, message).await;

        info!("Follow up: {:?}", res);

        Ok(())
    }

    async fn handle_error_with_followup(
        http: &Http,
        cmd: &CommandInteraction,
        err: String,
    ) -> anyhow::Result<()> {
        let message = CreateInteractionResponseFollowup::new().content(format!("Error: {:?}", err));
        let _ = cmd.create_followup(http, message).await;
        Ok(())
    }
}
