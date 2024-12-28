use serenity::builder::*;

#[derive(Debug, PartialEq, Default)]
pub struct MenuCmd;
impl MenuCmd {
    pub fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
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

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
