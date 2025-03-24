use crate::*;
use anyhow::Context;
use dnl_types::api::error::ApiResponseError;
use dnl_types::api::request::NewGameData;
use dnl_types::api::response::NewGameResponse;
use reqwest::Response;
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct NewGameCmd;

impl NewGameCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let client = reqwest::Client::new();

        let url = format!("{}/{}", DNL_API_URL.to_string(), "api/game/new");

        let body = NewGameData::from(cmd);

        let res = client.post(url).json(&body).send().await?;

        if res.status().is_success() {
            Self::handle_sucessful_response(res).await
        } else {
            Self::handle_error(res).await
        }
    }

    pub async fn handle_sucessful_response(
        res: Response,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let output: NewGameResponse = res
            .json()
            .await
            .context("Failed to parse into NewGameResponse")?;

        let content = format!("New game created - {}", output.game_id);

        let message = CreateInteractionResponseMessage::new().content(content);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }

    pub async fn handle_error(res: Response) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let error: ApiResponseError = res
            .json()
            .await
            .context("Failed to parse into ApiResponseError")?;

        let message = CreateInteractionResponseMessage::new().content(error.error);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
