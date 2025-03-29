pub mod attack;
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
pub mod settings;
pub mod text;

mod error;

pub use error::*;

use attack::*;
use buttons::*;
use class::*;
use list_games::*;
use menu::*;
use new_game::*;
use resume_game::*;
use roll::*;
use scenario::*;
use settings::*;
use text::*;

use dnl_types::api::error::ApiResponseError;
use dnl_types::traits::DiscordMsg;

use std::str::FromStr;

use anyhow::Context;
use lambda_http::tracing::info;
use reqwest::Response;
use serenity::all::Http;
use serenity::builder::*;
use serenity::model::application::*;
use strum::EnumString;

lazy_static::lazy_static! {
    pub static ref DNL_API_URL: String = format!("https://dnl-api.{}.aws.fomillercloud.com", std::env::var("ENVIRONMENT").unwrap());
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
    #[strum(serialize = "attack", ascii_case_insensitive)]
    Attack(AttackCmd),
    #[strum(serialize = "settings", ascii_case_insensitive)]
    Settings(SettingsCmd),
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
        SlashCommands::Attack(cmd) => cmd.execute(interaction).await,
        SlashCommands::Settings(cmd) => cmd.execute(interaction).await,
    }?;

    Ok(res)
}

pub fn format_interaction_response(content: String) -> CreateInteractionResponse {
    let message = CreateInteractionResponseMessage::new().content(content);

    CreateInteractionResponse::Message(message)
}
