use crate::*;
use dnl_store::Store;
use lambda_http::tracing::info;
use serenity::builder::*;
use serenity::http::Http;
use serenity::model::application::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct ScenarioCmd;

impl ScenarioCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let token =
            std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

        let http = Http::new(&token);
        http.set_application_id(cmd.application_id);

        let res = cmd.defer(&http).await?;
        info!("DEFER: {:?}", res);

        let user_id = cmd.user.id.to_string();

        let store = Store::new().await;

        let game_id = store.try_get_active_game_id(&user_id).await?;

        // :TODO: make this into ApiClient
        let client = reqwest::Client::new();

        let mut json = HashMap::<&str, &str>::new();
        json.insert("user_id", &user_id);
        json.insert("game_id", &game_id);

        let options = &cmd.data.options;
        for option in options {
            json.insert(
                &option.name,
                option.value.as_str().expect("Option value as not a string"),
            );
        }

        let url = format!("{}/{}", DNL_API_URL.to_string(), "/api/llm/scenario");

        let res = match client.post(url).json(&json).send().await {
            Ok(response) => {
                handle_sucessful_response(response, cmd, http).await?;
                Ok(None)
            }
            Err(err) => {
                let message = CreateInteractionResponseFollowup::new().content(err.to_string());
                let _ = cmd.create_followup(&http, message).await;
                Err(anyhow::anyhow!(err))
            }
        };
        res
    }
}
