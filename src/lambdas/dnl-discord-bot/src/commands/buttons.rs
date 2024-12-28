use serenity::builder::*;
use serenity::model::application::*;
#[derive(Debug, PartialEq, Default)]
pub struct ButtonsCmd;
impl ButtonsCmd {
    pub fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!("My Button!");
        let button = CreateButton::new("my_button")
            .style(ButtonStyle::Primary)
            .label("Click me!");
        // let action_row = CreateActionRow::Buttons(vec![button]);
        // let components = vec![action_row];
        let message = CreateInteractionResponseMessage::new()
            .content(content)
            .button(button);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
