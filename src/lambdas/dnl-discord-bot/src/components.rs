use lambda_http::tracing::info;
use serenity::builder::*;
use serenity::model::application::*;

pub async fn try_handle_component_interaction(
    interaction: ComponentInteraction,
) -> anyhow::Result<Option<CreateInteractionResponse>> {
    info!("CUSTOM_ID: {}", interaction.data.custom_id);
    // custom_id's need to become Enums
    let id = interaction.data.custom_id.as_str();
    match ComponentCustomId::from(id) {
        ComponentCustomId::BackGroundMenu(cmd) => cmd.execute(),
        ComponentCustomId::ClassSelectMenu(cmd) => cmd.execute().await,
        ComponentCustomId::RaceMenu(cmd) => cmd.execute(),
        ComponentCustomId::EquipItem(cmd) => cmd.execute().await,
        ComponentCustomId::BuyItem(cmd) => cmd.execute().await,
        ComponentCustomId::Unknown(cmd) => cmd.execute().await,
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

#[derive(Debug, PartialEq, Default)]
pub struct BuyItemCmd {
    id: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct EquipItemCmd {
    pub id: String,
}

#[derive(Debug, PartialEq, Default)]
pub struct UnknownCmd(String);

#[derive(Debug, PartialEq, strum::Display)]
pub enum Component {
    #[strum(to_string = "menu_{0}")]
    Menu(MenuType),
    #[strum(to_string = "button_{0}")]
    Button(ButtonType),
}

#[derive(Debug, PartialEq, strum::Display)]
pub enum MenuType {
    #[strum(to_string = "class")]
    Class,
    #[strum(to_string = "race")]
    Race,
    #[strum(to_string = "background")]
    BackGround,
}

#[derive(Debug, PartialEq, strum::Display)]
pub enum ButtonType {
    #[strum(to_string = "equip_item_{0}")]
    EquipItem(usize),
    #[strum(to_string = "buy_item_{0}")]
    BuyItem(usize),
}

#[derive(Debug, PartialEq)]
pub enum ComponentCustomId {
    ClassSelectMenu(ClassSelectMenuCmd),
    RaceMenu(RaceSelectMenuCmd),
    BackGroundMenu(BackGroundSelectMenuCmd),
    EquipItem(EquipItemCmd),
    BuyItem(BuyItemCmd),
    Unknown(UnknownCmd),
}

impl From<&str> for ComponentCustomId {
    fn from(id: &str) -> Self {
        let id = id.to_lowercase();

        // Try to strip the "menu_" prefix and match the remaining string
        if let Some(id_stripped) = id.strip_prefix("menu_") {
            match id_stripped {
                "class" => ComponentCustomId::ClassSelectMenu(ClassSelectMenuCmd),
                "race" => ComponentCustomId::RaceMenu(RaceSelectMenuCmd),
                "background" => ComponentCustomId::BackGroundMenu(BackGroundSelectMenuCmd),
                _ => ComponentCustomId::Unknown(UnknownCmd(id)),
            }
        } else if let Some(id_stripped) = id.strip_prefix("button_") {
            if let Some(item_id) = id_stripped.strip_prefix("buy_item_") {
                ComponentCustomId::BuyItem(BuyItemCmd {
                    id: item_id.to_string(),
                })
            } else if let Some(item_id) = id_stripped.strip_prefix("equip_item_") {
                ComponentCustomId::EquipItem(EquipItemCmd {
                    id: item_id.to_string(),
                })
            } else {
                ComponentCustomId::Unknown(UnknownCmd(id))
            }
        } else {
            ComponentCustomId::Unknown(UnknownCmd(id))
        }
    }
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

impl BuyItemCmd {
    pub async fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!("Buy item custom_id for component interaction: {}", self.id);
        let message = CreateInteractionResponseMessage::new().content(content);
        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}

impl EquipItemCmd {
    pub async fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!(
            "Equip item custom_id for component interaction: {}",
            self.id
        );
        let message = CreateInteractionResponseMessage::new().content(content);
        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}

impl UnknownCmd {
    pub async fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!("Unknown custom_id for component interaction: {}", self.0);
        let message = CreateInteractionResponseMessage::new().content(content);
        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
