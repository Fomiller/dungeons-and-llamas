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
        let store = Store::new(&cmd.user.id.to_string()).await;

        let state = store.try_get_state().await?;

        println!("Got state");

        let round = state.round.unwrap().parse()?;

        let map = store.try_get_map().await?;

        println!("Got state");

        let content = format!("{}", map.as_discord_msg());

        println!("{}", content);

        println!("ROUND: {}", round);

        let mut buttons = Vec::new();

        if let Some(curr_encounter_id) = state.curr_encounter_id {
            let events = map.get_row_values(round);

            println!("EVENTS: {:?}", events);

            for event in events {
                let current_location = map
                    .get_row_values(round - 1)
                    .iter()
                    .find(|e| e.id.to_string() == curr_encounter_id)
                    .unwrap()
                    .location;

                if event.parent.unwrap() == current_location {
                    println!("Has parent");
                    buttons.push(
                        CreateButton::new(
                            Component::Button(ButtonType::MapEvent(event.id.to_string()))
                                .to_string(),
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
                    println!("EVENT: {:?}", event);

                    let button = CreateButton::new(
                        Component::Button(ButtonType::MapEvent(event.id.to_string())).to_string(),
                    )
                    .style(ButtonStyle::Primary)
                    .label(format!("{:?}", &event.encounter_type).to_lowercase());

                    println!("BUTTON: {:?}", button);

                    buttons.push(button);
                    println!("hello")
                }
            }
        } else {
            let events = map.get_row_values(round);

            println!("EVENTS: {:?}", events);

            for event in events.clone() {
                buttons.push(
                    CreateButton::new(
                        Component::Button(ButtonType::MapEvent(event.id.to_string())).to_string(),
                    )
                    .style(ButtonStyle::Primary)
                    .label(format!("{:?}", &event.encounter_type).to_lowercase()),
                );
            }
        }
        println!("BUTTONs: {:?}", buttons);

        let components = CreateActionRow::Buttons(buttons);

        println!("Components {:?}", components);

        let message = CreateInteractionResponseMessage::new()
            .content(content)
            .components(vec![components]);

        println!("Message {:?}", message);

        let res = CreateInteractionResponse::Message(message);

        Ok(Some(res))
    }
}
