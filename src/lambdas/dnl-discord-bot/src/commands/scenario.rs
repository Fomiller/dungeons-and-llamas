use crate::error::handle_error;
use crate::*;
use std::collections::HashMap;

use dnl_store::Store;
use dnl_types::scenarios::ScenarioModel;

use serenity::builder::*;
use serenity::http::Http;

use anyhow::Context;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct ScenarioCmd;

#[async_trait::async_trait]
impl DiscordCmdResponse for ScenarioCmd {}

impl ScenarioCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let token =
            std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

        let http = Http::new(&token);

        http.set_application_id(cmd.application_id);

        cmd.defer(&http).await.context("Failed to defer command")?;

        let user_id = cmd.user.id.to_string();

        let store = Store::new().await;

        let game_id = match store
            .try_get_active_game_id(&user_id)
            .await
            .context("Failed to get active game id")
        {
            Ok(game_id) => game_id,
            Err(err) => return handle_error(&http, &cmd, err).await,
        };

        // :TODO: make this into ApiClient
        let client = reqwest::Client::new();

        let mut json = HashMap::<&str, &str>::new();
        json.insert("user_id", &user_id);
        json.insert("game_id", &game_id);

        for option in &cmd.data.options {
            json.insert(
                &option.name,
                option.value.as_str().expect("Option value as not a string"),
            );
        }

        let url = format!("{}/{}", DNL_API_URL.to_string(), "api/llm/scenario");

        match client.post(url).json(&json).send().await {
            Ok(response) => {
                if let Err(err) = Self::handle_sucessful_response_with_followup::<ScenarioModel>(
                    response, &cmd, &http,
                )
                .await
                {
                    return Self::handle_error_with_followup(&http, &cmd, err).await;
                }
                Ok(None)
            }
            Err(err) => return error::handle_error(&http, &cmd, err.into()).await,
        }
    }
}
