use crate::*;
use dnl_store::Store;
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct ResumeGameCmd;
impl ResumeGameCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let user_id = cmd.user.id.to_string();
        let client = Store::new(&user_id).await;
        let result = client.try_get_game_state().await?;

        if let Some(state) = result {
            let content = format!("{:?}", state);
            Ok(Some(format_interaction_response(content)))
        } else {
            Ok(Some(format_interaction_response(
                "No games available to resume.\nUse '/new-game' to start a new game.".to_string(),
            )))
        }
    }
}
