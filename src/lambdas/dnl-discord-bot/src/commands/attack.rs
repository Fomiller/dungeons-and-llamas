use crate::DiscordCmdResponse;

use dnl_store::Store;

use anyhow::Context;
use lambda_http::tracing::debug;
use serenity::builder::*;
use serenity::http::Http;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct AttackCmd;

impl DiscordCmdResponse for AttackCmd {}

impl AttackCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        // let token = cmd.token;
        let user_id = cmd.user.id.to_string();
        let client = Store::new(&user_id).await;

        let token =
            std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

        let http = Http::new(&token);

        http.set_application_id(cmd.application_id);

        let _ = client
            .try_save_message_token(&token)
            .await
            .context("failed to save message_token");

        let game_id = match client
            .try_get_active_game_id()
            .await
            .context("Failed to get active game id")
        {
            Ok(game_id) => game_id,
            Err(err) => return Self::handle_error_with_followup(&http, &cmd, err).await,
        };

        let state = client.try_get_state().await?;

        let enemies = client
            .try_get_enemies(&game_id, state.round.unwrap(), state.level.unwrap())
            .await?;

        let mut enemy_embeds = Vec::new();
        for enemy in enemies {
            enemy_embeds.push(
                CreateEmbed::new()
                    .color(serenity::model::Colour::BLUE)
                    .title(enemy.r#type)
                    .field("Health", enemy.health.current.to_string(), false)
                    .field("Attack", enemy.attacks.name, true)
                    .field("Damage", enemy.attacks.expression.to_string(), true),
            )
        }

        let message = CreateInteractionResponseMessage::new().add_embeds(enemy_embeds);

        debug!("EMBED {:?}", message);
        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
