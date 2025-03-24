use crate::*;
use anyhow::{anyhow, Context};
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

        match res.error_for_status() {
            Ok(_res) => Self::handle_sucessful_response(_res).await,
            Err(err) => {
                info!("New Game Error: {}", err);

                let content = format!("Error: {}", err);

                let message = CreateInteractionResponseMessage::new().content(content);

                Ok(Some(CreateInteractionResponse::Message(message)))
            }
        }
    }

    pub async fn handle_sucessful_response(
        res: Response,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        info!("API Res: {:?}", res);

        let output: NewGameResponse = res
            .json()
            .await
            .context("Failed to parse into NewGameResponse")?;

        let content = format!("New game created - {}", output.game_id);

        let message = CreateInteractionResponseMessage::new().content(content);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
