use crate::*;
use serenity::builder::*;

#[derive(Debug, PartialEq, Default)]
pub struct ListGamesCmd;
impl ListGamesCmd {
    pub fn execute(&self) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!("No games found.");

        Ok(Some(format_interaction_response(content)))
    }
}
