use dnl_store::Store;
use lambda_http::tracing::debug;
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct TextCmd;
impl TextCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let token = cmd.token;

        let client = Store::new().await;
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
        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
