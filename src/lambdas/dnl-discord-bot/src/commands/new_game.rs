use crate::*;
use dnl_types::api::request::NewGameRequest;
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

        let url = format!("{}/{}", DNL_API_URL.to_string(), "/api/game/new");

        let body = NewGameRequest::from(cmd);

        match client.post(url).json(&body).send().await {
            Ok(_) => {
                let content = format!("New game created.");
                Ok(Some(format_interaction_response(content)))
            }
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
