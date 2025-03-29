use crate::*;
use dnl_types::api::request::NewGameData;
use dnl_types::api::response::NewGameResponse;
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct NewGameCmd;

#[async_trait::async_trait]
impl DiscordCmdResponse for NewGameCmd {}

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
            Self::handle_sucessful_response::<NewGameResponse>(res).await
        } else {
            Self::handle_error(res).await
        }
    }
}
