use crate::*;
use crate::error::handle_error;
use std::collections::HashMap;

use dnl_store::Store;
use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::shop::ShopToolOutput;
use dnl_types::tools::rest::RestToolOutput;
use dnl_types::api::*;

use lambda_http::tracing::info;
use reqwest::Response;
use serenity::builder::*;
use serenity::http::Http;

use anyhow::Context;
use serenity::model::application::*;

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

        cmd.defer(&http).await.context("Failed to defer command")?;

        let user_id = cmd.user.id.to_string();

        let store = Store::new().await;

        let game_id = match store.try_get_active_game_id(&user_id).await.context("Failed to get active game id") {
            Ok(game_id) => game_id,
            Err(err) => return handle_error(&http, &cmd, err).await
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

        let url = format!("{}/{}", DNL_API_URL.to_string(), "/api/llm/scenario");

        match client.post(url).json(&json).send().await {
            Ok(response) => {
                Self::handle_sucessful_response(response, cmd, http).await?;
                Ok(None)
            }
            Err(err) => return error::handle_error(&http, &cmd, err.into()).await
        }
    }
    
    pub async fn handle_sucessful_response(
        res: Response,
        cmd: CommandInteraction,
        http: Http,
    ) -> anyhow::Result<()> {
        let res: ApiScenarioResponse = res.json().await.context("Failed to parse into ApiScenarioResponse")?;
        
        let content = match res.data {
            ApiScenarioResponseData::Battle(data) => Self::format_battle_scenario(data),
            ApiScenarioResponseData::Shop(data) => Self::format_shop_scenario(data),
            ApiScenarioResponseData::Rest(data) => Self::format_rest_scenario(data),
        };

        let message = CreateInteractionResponseFollowup::new().content(content);

        let res = cmd.create_followup(&http, message).await;
        info!("Follow up: {:?}", res);

        Ok(())
    }

    fn format_battle_scenario(data: BattleToolOutput) -> String {
        let mut enemy_description = String::new();

        for enemy in data.enemies {
            let description = format!(
                "- **{}**\n  - Attack: {}\n  - Damage: {}\n  - Health: {}\n",
                enemy.enemy_type, enemy.attack.attack_name, enemy.attack.attack_damage, enemy.health
            );
            enemy_description.push_str(&description);
        }

        format!(
            "# *{}*\n## Description:\n{}\n\n## Terrain:\n{}\n\n## Enemies:\n{}\n\n## Summary:\n{}",
            data.name, data.summary, data.terrain, enemy_description, data.summary
        )
    }
    
    fn format_shop_scenario(data: ShopToolOutput) -> String {
        let mut item_descriptions = String::new();

        for item in data.items {
            let description = format!(
                "- **{}**\n{}\n - Stats: {}\n  - Price: {}\n",
                item.name, item.description, item.stats, item.price
            );
            item_descriptions.push_str(&description);
        }

        format!(
            "# *{}*\n## Description:\n{}\n\n## Items:\n{}",
            data.merchant.name, data.merchant.description, item_descriptions
        )
    }

    fn format_rest_scenario(_data: RestToolOutput) -> String {"".to_string()}
}

