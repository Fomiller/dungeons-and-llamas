use crate::errors::handle_error;
use crate::*;

use std::collections::HashMap;
use std::str::FromStr;

use dnl_store::Store;
use dnl_types::api::find_options_value;
use dnl_types::api::request::ScenarioRequest;
use dnl_types::generators::{GeneratorScenarioConfig, GeneratorType};
use dnl_types::scenarios::{Scenario, ScenarioModel};

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

        info!("HERE 1");
        cmd.defer(&http).await.context("Failed to defer command")?;
        info!("HERE 2");

        let user_id = cmd.user.id.to_string();

        let store = Store::new(&user_id).await;

        let game_id = match store
            .try_get_active_game_id()
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

        let scenario =
            Scenario::from_str(&find_options_value(&cmd.data.options, "scenario").unwrap())?;

        let config = match scenario {
            Scenario::Battle => GeneratorScenarioConfig::Battle,
            Scenario::Shop => GeneratorScenarioConfig::Shop,
            Scenario::Rest => GeneratorScenarioConfig::Rest,
            Scenario::NewGame => GeneratorScenarioConfig::NewGame,
        };

        let generator = GeneratorType::Scenario(config);

        let payload = ScenarioRequest {
            user_id,
            generator,
            options: cmd.data.options.clone(),
        };

        let url = format!("{}/{}", DNL_API_URL.to_string(), "api/llm/scenario");

        let response = client.post(url).json(&payload).send().await?;

        if response.status().is_success() {
            Self::handle_sucessful_response_with_followup::<ScenarioModel>(response, &cmd, &http)
                .await?
        } else {
            let text = response.text().await?;
            let _ = Self::handle_error_with_followup(&http, &cmd, text).await;
        }

        Ok(None)
    }
}
