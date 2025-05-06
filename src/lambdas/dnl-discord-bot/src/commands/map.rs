use crate::*;
use dnl_map::*;

use anyhow::Context;
use serenity::all::Http;
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct MapCmd;

#[async_trait::async_trait]
impl DiscordCmdResponse for MapCmd {}

impl MapCmd {
    pub async fn execute(
        &self,
        _cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let height = 10;
        let width = 8;
        let paths = 3;

        let map = match GameMap::generate(width, height, paths) {
            Ok(map) => map,
            Err(_) => {
                panic!("Map failed to generate.")
            }
        };

        // let token =
        //     std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

        // let http = Http::new(&token);
        //
        // http.set_application_id(cmd.application_id);
        //
        // cmd.defer(&http).await.context("Failed to defer command")?;

        let content = format!("{}", map.as_discord_msg());

        println!("{}", content);

        let message = CreateInteractionResponseMessage::new().content(content);
        let res = CreateInteractionResponse::Message(message);
        // let message = CreateInteractionResponseFollowup::new().content(content);

        // let res = cmd.create_followup(&http, message).await?;

        // println!("{:?}", res);

        Ok(Some(res))
    }
}
