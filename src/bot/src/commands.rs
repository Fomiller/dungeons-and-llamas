use dice::Dice;
use game::client::Client;
use game::state::message::MessageSortKey;
use game::state::{RootSortKeyBuilder, SortKeyBuildable};
use lambda_http::tracing::debug;
use lambda_http::tracing::info;
use serenity::builder::*;
use serenity::model::application::*;
use std::collections::HashMap;
use std::str::FromStr;
use strum::EnumString;

pub async fn try_handle_command_interaction(
    interaction: CommandInteraction,
) -> anyhow::Result<CreateInteractionResponse> {
    info!("NAME: {:?}", &interaction.data.name);

    let command_name = SlashCommands::from_str(&interaction.data.name).unwrap();
    info!("COMMAND NAME: {:?}", command_name);

    match command_name {
        SlashCommands::Class(cmd) => cmd.execute(interaction),
        SlashCommands::Roll(cmd) => cmd.execute(interaction),
        SlashCommands::NewGame(cmd) => cmd.execute(interaction).await,
        SlashCommands::ResumeGame(cmd) => cmd.execute(interaction).await,
        SlashCommands::ListGames(cmd) => cmd.execute(),
        SlashCommands::Buttons(cmd) => cmd.execute(),
        SlashCommands::Menu(cmd) => cmd.execute(),
        SlashCommands::Text(cmd) => cmd.execute(interaction).await,
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct ClassCmd;

#[derive(Debug, PartialEq, Default)]
pub struct RollCmd;

#[derive(Debug, PartialEq, Default)]
pub struct NewGameCmd;

#[derive(Debug, PartialEq, Default)]
pub struct ResumeGameCmd;

#[derive(Debug, PartialEq, Default)]
pub struct ListGamesCmd;

#[derive(Debug, PartialEq, Default)]
pub struct ButtonsCmd;

#[derive(Debug, PartialEq, Default)]
pub struct MenuCmd;

#[derive(Debug, PartialEq, Default)]
pub struct TextCmd;

#[derive(Debug, PartialEq, Default)]
pub struct EmbedCmd;

#[derive(Debug, PartialEq, Default)]
pub struct EditCmd;

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
}

impl RollCmd {
    pub fn execute(&self, cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
        let count = Self::get_count(&cmd);

        let sides = Self::get_sides(&cmd);

        let modifier = match &cmd.data.options.iter().find(|o| o.name == "modifier") {
            Some(m) => m
                .value
                .as_i64()
                .expect("could not convert modifier.value to integer")
                .try_into()
                .expect("could not convert to usize"),
            None => 0,
        };

        let dice: Vec<Dice> = (0..count).map(|_| Dice::new(sides)).collect();

        let dice_values: Vec<usize> = dice.into_iter().map(|d| d.roll()).collect::<Vec<usize>>();

        let roll: usize = dice_values.iter().sum();

        let content = format!(
            "Roll: {}\nYou rolled {}!",
            Self::create_roll_text(dice_values, modifier),
            roll
        );

        let message = CreateInteractionResponseMessage::new().content(content);

        Ok(CreateInteractionResponse::Message(message))
    }

    fn create_roll_text(dice_values: Vec<usize>, modifier: usize) -> String {
        if modifier > 0 {
            format!(
                "[{}{}] + {}",
                dice_values[0],
                dice_values[1..]
                    .iter()
                    .map(|v| format!(" + {}", v))
                    .collect::<Vec<String>>()
                    .join(""),
                modifier
            )
        } else {
            format!(
                "[{}{}]",
                dice_values[0],
                dice_values[1..]
                    .iter()
                    .map(|v| format!(" + {}", v))
                    .collect::<Vec<String>>()
                    .join("")
            )
        }
    }

    fn get_sides(cmd: &CommandInteraction) -> usize {
        let sides: usize = cmd
            .data
            .options
            .iter()
            .find(|o| o.name == "sides")
            .expect("Could not find 'sides' option for /roll")
            .value
            .as_i64()
            .expect("could not convert sides.value to integer")
            .try_into()
            .expect("could not convert to usize");
        sides
    }

    fn get_count(cmd: &CommandInteraction) -> usize {
        let count: usize = cmd
            .data
            .options
            .iter()
            .find(|o| o.name == "count")
            .expect("could not find 'count' option for /roll")
            .value
            .as_i64()
            .expect("could not convert count.value to integer")
            .try_into()
            .expect("could not convert to usize");
        count
    }
}

impl NewGameCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let client = Client::new().await;
        let user_id = cmd.user.id.to_string();
        let name = cmd.user.name.to_string();
        client.try_new_game_state(&user_id, &name).await?;

        let content = format!("New game created.");
        Ok(format_interaction_response(content))
    }
}

impl ResumeGameCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let client = Client::new().await;
        let user_id = cmd.user.id.to_string();
        let result = client.try_get_game_state(&user_id).await?;

        if let Some(state) = result {
            let content = format!("{:?}", state);
            Ok(format_interaction_response(content))
        } else {
            Ok(format_interaction_response(
                "No games available to resume.\nUse '/new-game' to start a new game.".to_string(),
            ))
        }
    }
}

impl ListGamesCmd {
    pub fn execute(&self) -> anyhow::Result<CreateInteractionResponse> {
        let content = format!("No games found.");

        Ok(format_interaction_response(content))
    }
}

impl ClassCmd {
    pub fn execute(&self, cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
        let class = &cmd
            .data
            .options
            .first()
            .expect("No options available")
            .value;

        let content = format!("You chose the {} class", class.as_str().unwrap());

        Ok(format_interaction_response(content))
    }
}

pub fn format_interaction_response(content: String) -> CreateInteractionResponse {
    let message = CreateInteractionResponseMessage::new().content(content);

    CreateInteractionResponse::Message(message)
}

impl ButtonsCmd {
    pub fn execute(&self) -> anyhow::Result<CreateInteractionResponse> {
        let content = format!("My Button!");
        let button = CreateButton::new("my_button")
            .style(ButtonStyle::Primary)
            .label("Click me!");
        // let action_row = CreateActionRow::Buttons(vec![button]);
        // let components = vec![action_row];
        let message = CreateInteractionResponseMessage::new()
            .content(content)
            .button(button);

        Ok(CreateInteractionResponse::Message(message))
    }
}

impl MenuCmd {
    pub fn execute(&self) -> anyhow::Result<CreateInteractionResponse> {
        let content = format!("My Menu!");
        let options = vec![
            CreateSelectMenuOption::new("Pizza", "pizza"),
            CreateSelectMenuOption::new("Ice cream", "ice cream"),
            CreateSelectMenuOption::new("Burger", "Burger"),
        ];
        let menu = CreateSelectMenu::new("my_menu", CreateSelectMenuKind::String { options })
            .placeholder("select something");

        let action_row = CreateActionRow::SelectMenu(menu);
        let components = vec![action_row];
        let message = CreateInteractionResponseMessage::new()
            .content(content)
            .components(components);

        Ok(CreateInteractionResponse::Message(message))
    }
}

impl TextCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let token = cmd.token;

        let client = Client::new().await;
        let user_id = cmd.user.id.to_string();
        client.try_save_message_token(&user_id, &token).await?;

        let _character_name = CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, "Name", "name")
                .placeholder("Legolas")
                .required(true),
        );

        let race_options = vec![
            CreateSelectMenuOption::new("Dragonborn", "dragonborn"),
            CreateSelectMenuOption::new("Dwarf", "dwarf"),
            CreateSelectMenuOption::new("Elf", "elf"),
            CreateSelectMenuOption::new("Goliath", "goliath"),
            CreateSelectMenuOption::new("Halfling", "halfling"),
            CreateSelectMenuOption::new("Human", "Human"),
            CreateSelectMenuOption::new("Orc", "orc"),
            CreateSelectMenuOption::new("Tiefling", "tiefling"),
        ];

        let background_options = vec![
            CreateSelectMenuOption::new("Soldier", "soldier"),
            CreateSelectMenuOption::new("Athlete", "athlete"),
            CreateSelectMenuOption::new("Artisan", "artisan"),
            CreateSelectMenuOption::new("Criminal", "criminal"),
            CreateSelectMenuOption::new("Entertainer", "entertainer"),
            CreateSelectMenuOption::new("Farmer", "farmer"),
            CreateSelectMenuOption::new("Hermit", "hermit"),
            CreateSelectMenuOption::new("Gambler", "gambler"),
            CreateSelectMenuOption::new("Noble", "noble"),
            CreateSelectMenuOption::new("Merchant", "merchant"),
        ];

        let class_options = vec![
            CreateSelectMenuOption::new("Barbarian", "barbarian"),
            CreateSelectMenuOption::new("Bard", "bard"),
            CreateSelectMenuOption::new("Cleric", "cleric"),
            CreateSelectMenuOption::new("Druid", "druid"),
            CreateSelectMenuOption::new("Fighter", "fighter"),
            CreateSelectMenuOption::new("Mage", "mage"),
            CreateSelectMenuOption::new("Monk", "monk"),
            CreateSelectMenuOption::new("Paladin", "paladin"),
            CreateSelectMenuOption::new("Ranger", "ranger"),
            CreateSelectMenuOption::new("Rouge", "rouge"),
            CreateSelectMenuOption::new("Sorcerer", "sorcerer"),
            CreateSelectMenuOption::new("Warlock", "warlock"),
            CreateSelectMenuOption::new("Wizard", "wizard"),
        ];

        let race_menu = CreateActionRow::SelectMenu(
            CreateSelectMenu::new(
                "race_menu",
                CreateSelectMenuKind::String {
                    options: race_options,
                },
            )
            .placeholder("Select a race"),
        );

        let class_menu = CreateActionRow::SelectMenu(
            CreateSelectMenu::new(
                "class_menu",
                CreateSelectMenuKind::String {
                    options: class_options,
                },
            )
            .placeholder("Select a class"),
        );

        let background_menu = CreateActionRow::SelectMenu(
            CreateSelectMenu::new(
                "background_menu",
                CreateSelectMenuKind::String {
                    options: background_options,
                },
            )
            .placeholder("Select a background"),
        );

        let menu_action_rows = vec![class_menu, race_menu, background_menu];
        // let action_rows = vec![character_name];

        let embed = CreateEmbed::new()
            .color(serenity::model::Colour::BLUE)
            .title("My Embed")
            .field("Name", "Forrest", false);

        let message = CreateInteractionResponseMessage::new()
            .embed(embed)
            .components(menu_action_rows);

        // let modal = CreateModal::new("my_modal", "My Modal").components(action_rows);
        // debug!("{:?}", modal);

        debug!("EMBED {:?}", message);
        Ok(CreateInteractionResponse::Message(message))
        // Ok(CreateInteractionResponse::Modal(modal))
    }
}

impl EditCmd {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        cmd: ComponentInteraction,
    ) -> anyhow::Result<CreateInteractionResponse> {
        let client = Client::new().await;
        let user_id = cmd.user.id.to_string();

        let query = client.try_get_last_message_token(&user_id).await?;
        let items = query.items.expect(
            format!(
                "Could not find {}",
                RootSortKeyBuilder::new()
                    .id(&user_id)
                    .message(MessageSortKey::LastMessageToken)
                    .build()
            )
            .as_str(),
        );

        info!("QUERY: {:?}", items);

        let token = items
            .first()
            .unwrap()
            .get_key_value("State")
            .expect("State for LastMessageToken not found")
            .1
            .as_s()
            .unwrap();

        let client = reqwest::Client::new();

        let mut map = HashMap::new();
        map.insert("content", "EDITED");
        let res = client
            .patch(format!(
                "https://discord.com/api/v10/webhooks/{}/{}/messages/{}",
                cmd.application_id, token, cmd.message.id
            ))
            .header(
                "Authorization",
                format!("Bot {}", std::env::var("DISCORD_BOT_TOKEN")?),
            )
            .json(&map)
            .send()
            .await?;
        info!("RES: {:?}", res);

        Ok(format_interaction_response("".to_string()))
    }
}
