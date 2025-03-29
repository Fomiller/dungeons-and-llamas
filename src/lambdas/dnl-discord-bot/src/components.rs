use serenity::builder::*;
use serenity::model::application::*;
use std::str::FromStr;
use strum::EnumString;

pub async fn try_handle_component_interaction(
    interaction: ComponentInteraction,
) -> anyhow::Result<Option<CreateInteractionResponse>> {
    // custom_id's need to become Enums
    match ComponentCustomId::from_str(&interaction.data.custom_id)? {
        ComponentCustomId::BackGroundMenu(cmd) => cmd.execute(),
        ComponentCustomId::ClassSelectMenu(cmd) => cmd.execute().await,
        ComponentCustomId::RaceMenu(cmd) => cmd.execute(),
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct ClassCmd;

#[derive(Debug, PartialEq, Default)]
pub struct ClassSelectMenuCmd;

#[derive(Debug, PartialEq, Default)]
pub struct RaceSelectMenuCmd;

#[derive(Debug, PartialEq, Default)]
pub struct BackGroundSelectMenuCmd;

#[derive(Debug, PartialEq, EnumString)]
pub enum ComponentCustomId {
    #[strum(serialize = "class_menu", ascii_case_insensitive)]
    ClassSelectMenu(ClassSelectMenuCmd),
    #[strum(serialize = "race_menu", ascii_case_insensitive)]
    RaceMenu(RaceSelectMenuCmd),
    #[strum(serialize = "background_menu", ascii_case_insensitive)]
    BackGroundMenu(BackGroundSelectMenuCmd),
}

impl ClassSelectMenuCmd {
    pub async fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let options = vec![
            CreateSelectMenuOption::new("Dragonborn", "dragonborn"),
            CreateSelectMenuOption::new("Dwarf", "dwarf"),
            CreateSelectMenuOption::new("Elf", "elf"),
            CreateSelectMenuOption::new("Goliath", "goliath"),
            CreateSelectMenuOption::new("Halfling", "halfling"),
            CreateSelectMenuOption::new("Human", "Human"),
            CreateSelectMenuOption::new("Orc", "orc"),
            CreateSelectMenuOption::new("Tiefling", "tiefling"),
        ];

        let menu = CreateActionRow::SelectMenu(
            CreateSelectMenu::new("race_menu", CreateSelectMenuKind::String { options })
                .placeholder("Select a race"),
        );

        let message = CreateInteractionResponseMessage::new().components(vec![menu]);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}

impl RaceSelectMenuCmd {
    pub fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let options = vec![
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

        let menu = CreateActionRow::SelectMenu(
            CreateSelectMenu::new("background_menu", CreateSelectMenuKind::String { options })
                .placeholder("Select a class"),
        );

        let message = CreateInteractionResponseMessage::new().components(vec![menu]);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}

impl BackGroundSelectMenuCmd {
    pub fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!("CHARACTER CREATED",);
        let message = CreateInteractionResponseMessage::new().content(content);
        Ok(Some(CreateInteractionResponse::UpdateMessage(message)))
    }
}
