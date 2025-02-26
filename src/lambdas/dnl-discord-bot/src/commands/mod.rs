pub mod buttons;
pub mod class;
pub mod edit;
pub mod embed;
pub mod list_games;
pub mod menu;
pub mod new_game;
pub mod resume_game;
pub mod roll;
pub mod scenario;
pub mod text;

use buttons::*;
use class::*;
use list_games::*;
use menu::*;
use new_game::*;
use resume_game::*;
use roll::*;
use scenario::*;
use text::*;

use dnl_llm::tool::BattleToolOutput;
use lambda_http::tracing::info;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serenity::builder::*;
use serenity::http::Http;
use serenity::model::application::*;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize)]
struct ApiScenarioResponse {
    data: BattleToolOutput,
}

lazy_static::lazy_static! {
    pub static ref DNL_API_URL: String = format!("https://dnl-api.{}.aws.fomillercloud.com", std::env::var("ENVIRONMENT").unwrap());
}

pub async fn try_handle_command_interaction(
    interaction: CommandInteraction,
) -> anyhow::Result<Option<CreateInteractionResponse>> {
    info!("NAME: {:?}", &interaction.data.name);

    let command_name = SlashCommands::from_str(&interaction.data.name).unwrap();
    info!("COMMAND NAME: {:?}", command_name);

    let res = match command_name {
        SlashCommands::Class(cmd) => cmd.execute(interaction),
        SlashCommands::Roll(cmd) => cmd.execute(interaction),
        SlashCommands::NewGame(cmd) => cmd.execute(interaction).await,
        SlashCommands::ResumeGame(cmd) => cmd.execute(interaction).await,
        SlashCommands::ListGames(cmd) => cmd.execute(),
        SlashCommands::Buttons(cmd) => cmd.execute(),
        SlashCommands::Menu(cmd) => cmd.execute(),
        SlashCommands::Text(cmd) => cmd.execute(interaction).await,
        SlashCommands::Scenario(cmd) => cmd.execute(interaction).await,
    }?;

    Ok(res)
}

#[derive(Debug, PartialEq, EnumString)]
pub enum SlashCommands {
    #[strum(ascii_case_insensitive)]
    Class(ClassCmd),
    #[strum(ascii_case_insensitive)]
    Roll(RollCmd),
    #[strum(serialize = "new-game", ascii_case_insensitive)]
    NewGame(NewGameCmd),
    #[strum(serialize = "resume-game", ascii_case_insensitive)]
    ResumeGame(ResumeGameCmd),
    #[strum(serialize = "list-games", ascii_case_insensitive)]
    ListGames(ListGamesCmd),
    #[strum(serialize = "buttons", ascii_case_insensitive)]
    Buttons(ButtonsCmd),
    #[strum(serialize = "menu", ascii_case_insensitive)]
    Menu(MenuCmd),
    #[strum(serialize = "text", ascii_case_insensitive)]
    Text(TextCmd),
    #[strum(serialize = "scenario", ascii_case_insensitive)]
    Scenario(ScenarioCmd),
}

pub fn format_interaction_response(content: String) -> CreateInteractionResponse {
    let message = CreateInteractionResponseMessage::new().content(content);

    CreateInteractionResponse::Message(message)
}

pub async fn handle_sucessful_response(
    res: Response,
    cmd: CommandInteraction,
    http: Http,
) -> anyhow::Result<()> {
    match res.status() {
        reqwest::StatusCode::OK => {
            let data = res.json::<ApiScenarioResponse>().await?.data;

            info!("Data: {:?}", data);

            let content = format_battle_scenario(data);

            let message = CreateInteractionResponseFollowup::new().content(content);

            let res = cmd.create_followup(&http, message).await;

            info!("FOLLOW: {:?}", res);
            Ok(())
        }
        reqwest::StatusCode::SERVICE_UNAVAILABLE => {
            let followup = CreateInteractionResponseFollowup::new();
            let message = followup.content("Server error, Try again.");
            cmd.create_followup(&http, message).await?;
            Ok(())
        }
        _ => Ok(()),
    }
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
