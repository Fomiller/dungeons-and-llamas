use crate::*;
use serenity::builder::*;
use serenity::model::application::*;
#[derive(Debug, PartialEq, Default)]
pub struct ClassCmd;
impl ClassCmd {
    pub fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let class = &cmd
            .data
            .options
            .first()
            .expect("No options available")
            .value;

        let content = format!("You chose the {} class", class.as_str().unwrap());

        Ok(Some(format_interaction_response(content)))
    }
}
