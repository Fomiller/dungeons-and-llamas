use crate::*;
use dnl_sort_keys::buildable::SortKeyBuildable;
use dnl_sort_keys::prelude::SortKeyFactory;
use dnl_store::Store;
use dnl_types::settings::Settings;

use serde_dynamo::{self};
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct SettingsCmd;

#[async_trait::async_trait]
impl DiscordCmdResponse for SettingsCmd {}

impl SettingsCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let user_id = cmd.user.id.to_string();

        let settings = Settings::from(cmd);
        info!("settings {:?}", settings);

        //:TODO: this should be api driven instead of calling the store from the bot
        let store = Store::new(&user_id).await;

        let game_id = match store.try_get_active_game_id().await {
            Ok(game_id) => game_id,
            Err(_) => {
                let content = "Error retrieving game id".to_string();
                let message = CreateInteractionResponseMessage::new().content(content);
                return Ok(Some(CreateInteractionResponse::Message(message)));
            }
        };

        let item = match serde_dynamo::to_item(settings) {
            Ok(item) => {
                println!("ITEM: {:?}", item);
                item
            }
            Err(_) => {
                let content = "Error converting settings to item".to_string();
                let message = CreateInteractionResponseMessage::new().content(content);
                return Ok(Some(CreateInteractionResponse::Message(message)));
            }
        };

        let sk = SortKeyFactory::new(&user_id)
            .create_game_settings_sk(&game_id)
            .build();

        match store.try_generic_update(&user_id, &sk, item).await {
            Ok(_) => (),
            Err(_) => {
                let content = "Error updating settings".to_string();
                let message = CreateInteractionResponseMessage::new().content(content);
                return Ok(Some(CreateInteractionResponse::Message(message)));
            }
        };

        let content = format!("Updated Settings");

        let message = CreateInteractionResponseMessage::new().content(content);

        Ok(Some(CreateInteractionResponse::Message(message)))
    }
}
