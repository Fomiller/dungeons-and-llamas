use crate::*;
use dnl_map::*;

use anyhow::Context;
use dnl_store::Store;
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
        cmd: CommandInteraction,
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

        let store = Store::new(&cmd.user.id.to_string()).await;

        let state = store.try_get_state().await?;

        let round = state.round.unwrap().parse()?;
        println!("ROUND: {}", round);

        let events = map.get_row_values(round);

        println!("EVENTS: {:?}", events);

        let mut buttons = Vec::new();

        for event in events.clone() {
            buttons.push(
                CreateButton::new(
                    Component::Button(ButtonType::MapEvent(event.id.to_string())).to_string(),
                )
                .style(ButtonStyle::Primary)
                .label(format!("{:?}", &event.encounter_type).to_lowercase()),
            );
        }

        let components = CreateActionRow::Buttons(buttons);

        let events_2 = map.get_row_values(round + 1);

        println!("EVENTS_2: {:?}", events_2);

        let mut buttons_2 = Vec::new();

        for event in events_2 {
            let current_location = events.clone()[0].location;
            if event.parent.unwrap() == current_location {
                println!("Has parent");
                buttons_2.push(
                    CreateButton::new(
                        Component::Button(ButtonType::MapEvent(event.id.to_string())).to_string(),
                    )
                    .style(ButtonStyle::Primary)
                    .label(format!("{:?}", &event.encounter_type).to_lowercase()),
                );
            } else if [
                current_location.col - 1,
                current_location.col,
                current_location.col + 1,
            ]
            .contains(&event.location.col)
            {
                println!("valid next position");
                buttons_2.push(
                    CreateButton::new(
                        Component::Button(ButtonType::MapEvent(event.id.to_string())).to_string(),
                    )
                    .style(ButtonStyle::Primary)
                    .label(format!("{:?}", &event.encounter_type).to_lowercase()),
                );
            }
        }

        let components_2 = CreateActionRow::Buttons(buttons_2);

        let message = CreateInteractionResponseMessage::new()
            .content(content)
            .components(vec![components, components_2]);

        let res = CreateInteractionResponse::Message(message);

        // let message = CreateInteractionResponseFollowup::new().content(content);

        // let res = cmd.create_followup(&http, message).await?;

        // println!("{:?}", res);

        Ok(Some(res))
    }
}
