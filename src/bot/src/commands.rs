use dice::Dice;
use game::client::Client;
use serenity::all::ReactionType;
use serenity::builder::*;
use serenity::model::application::*;
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum SlashCommands {
    #[strum(ascii_case_insensitive)]
    Class,
    #[strum(ascii_case_insensitive)]
    Roll,
    #[strum(serialize = "new-game", ascii_case_insensitive)]
    NewGame,
    #[strum(serialize = "resume-game", ascii_case_insensitive)]
    ResumeGame,
    #[strum(serialize = "list-games", ascii_case_insensitive)]
    ListGames,
    #[strum(serialize = "buttons", ascii_case_insensitive)]
    Buttons,
    #[strum(serialize = "menu", ascii_case_insensitive)]
    Menu,
    #[strum(serialize = "text", ascii_case_insensitive)]
    Text,
}

pub struct Roll;
impl Roll {
    pub fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
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

pub struct NewGame;
impl NewGame {
    pub async fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
        let client = Client::new().await;
        let user_id = cmd.user.id.to_string();
        let name = cmd.user.name.to_string();
        client.try_new_game_state(&user_id, &name).await?;

        let content = format!("New game created.");
        Ok(format_interaction_response(content))
    }
}

pub struct ResumeGame;
impl ResumeGame {
    pub async fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
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

pub struct ListGames;
impl ListGames {
    pub fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
        let content = format!("No games found.");

        Ok(format_interaction_response(content))
    }
}

pub struct Class;
impl Class {
    pub fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
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

pub struct Buttons;
impl Buttons {
    pub fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
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

pub struct Menu;
impl Menu {
    pub fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
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

pub struct Text;
impl Text {
    pub fn command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
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
