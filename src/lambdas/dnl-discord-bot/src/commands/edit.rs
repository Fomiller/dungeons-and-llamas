use crate::*;
use dnl_sort_keys::prelude::*;
use dnl_store::Store;
use lambda_http::tracing::info;
use serenity::builder::*;
use serenity::model::application::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct EditCmd;

impl EditCmd {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        cmd: ComponentInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let client = Store::new().await;
        let user_id = cmd.user.id.to_string();

        let query = client.try_get_last_message_token(&user_id).await?;
        let items = query.items.expect(
            format!(
                "Could not find {}",
                RootSortKeyBuilder::new()
                    .id(&user_id)
                    .message(MessageSortKey::LastMessageToken)
                    .build()
            )
            .as_str(),
        );

        info!("QUERY: {:?}", items);

        let token = items
            .first()
            .unwrap()
            .get_key_value("State")
            .expect("State for LastMessageToken not found")
            .1
            .as_s()
            .unwrap();

        let client = reqwest::Client::new();

        let mut map = HashMap::new();
        map.insert("content", "Generating...");
        let res = client
            .patch(format!(
                "https://discord.com/api/v10/webhooks/{}/{}/messages/{}",
                cmd.application_id, token, cmd.message.id
            ))
            .header(
                "Authorization",
                format!("Bot {}", std::env::var("DISCORD_BOT_TOKEN")?),
            )
            .json(&map)
            .send()
            .await?;
        info!("RES: {:?}", res);

        Ok(Some(format_interaction_response("".to_string())))
    }
}
