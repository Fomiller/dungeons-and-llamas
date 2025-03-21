use crate::error::handle_error;

use dnl_store::Store;

use anyhow::Context;
use lambda_http::tracing::debug;
use serenity::builder::*;
use serenity::http::Http;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct AttackCmd;
impl AttackCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        // let token = cmd.token;
        let client = Store::new().await;
        let user_id = cmd.user.id.to_string();

        let token =
            std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

        let http = Http::new(&token);

        http.set_application_id(cmd.application_id);

        // cmd.defer(&http).await.context("Failed to defer command")?;

        let _ = client
            .try_save_message_token(&user_id, &token)
            .await
            .context("failed to save message_token");

        let game_id = match client
            .try_get_active_game_id(&user_id)
            .await
            .context("Failed to get active game id")
        {
            Ok(game_id) => game_id,
            Err(err) => return handle_error(&http, &cmd, err).await,
        };

        let state = client.try_get_state(&user_id).await?;

        let enemies = client
            .try_get_enemies(
                &game_id,
                &user_id,
                state.round.unwrap(),
                state.level.unwrap(),
            )
            .await?;

        let mut enemy_embeds = Vec::new();
        for enemy in enemies {
            enemy_embeds.push(
                CreateEmbed::new()
                    .color(serenity::model::Colour::BLUE)
                    .title(enemy.enemy_type)
                    .field("Health", enemy.health.current.to_string(), false)
                    .field("Attack", enemy.attack.attack_name, true)
                    .field("Damage", enemy.attack.attack_expression.to_string(), true),
            )
        }

        // // client.get_enemies();
        // let enemies = vec![
        //     CreateSelectMenuOption::new("Dragonborn", "dragonborn"),
        //     CreateSelectMenuOption::new("Dwarf", "dwarf"),
        //     CreateSelectMenuOption::new("Elf", "elf"),
        //     CreateSelectMenuOption::new("Goliath", "goliath"),
        //     CreateSelectMenuOption::new("Halfling", "halfling"),
        //     CreateSelectMenuOption::new("Human", "Human"),
        //     CreateSelectMenuOption::new("Orc", "orc"),
        //     CreateSelectMenuOption::new("Tiefling", "tiefling"),
        // ];
        //
        // let enemy_menu = CreateActionRow::SelectMenu(
        //     CreateSelectMenu::new(
        //         "enemy_menu",
        //         CreateSelectMenuKind::String { options: enemies },
        //     )
        //     .placeholder("Select a race"),
        // );
        //
        // let class_menu = CreateActionRow::SelectMenu(
        //     CreateSelectMenu::new(
        //         "class_menu",
        //         CreateSelectMenuKind::String {
        //             options: class_options,
        //         },
        //     )
        //     .min_values(1)
        //     .max_values(3)
        //     .placeholder("Select a class"),
        // );
        //
        // let background_menu = CreateActionRow::SelectMenu(
        //     CreateSelectMenu::new(
        //         "background_menu",
        //         CreateSelectMenuKind::String {
        //             options: background_options,
        //         },
        //     )
        //     .min_values(1)
        //     .max_values(3)
        //     .placeholder("Select a background"),
        // );
        //
        // let menu_action_rows = vec![class_menu, race_menu, background_menu];
        // // let action_rows = vec![character_name];
        //
        // let embed = CreateEmbed::new()
        //     .color(serenity::model::Colour::BLUE)
        //     .title("My Embed")
        //     .field("Name", "Forrest", false);

        let message = CreateInteractionResponseMessage::new().add_embeds(enemy_embeds);

        // let modal = CreateModal::new("my_modal", "My Modal").components(action_rows);
        // debug!("{:?}", modal);

        debug!("EMBED {:?}", message);
        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
