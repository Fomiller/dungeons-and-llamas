use crate::*;
use serenity::builder::*;
use serenity::model::application::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct NewGameCmd;

impl NewGameCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let user_id = cmd.user.id.to_string();
        let client = reqwest::Client::new();

        let mut json = HashMap::new();
        json.insert("id", user_id);

        let url = format!("{}/{}", DNL_API_URL.to_string(), "/api/game/new");

        match client.post(url).json(&json).send().await {
            Ok(_) => {
                let content = format!("New game created.");
                Ok(Some(format_interaction_response(content)))
            }
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
