use crate::*;
use anyhow::Context;
use dnl_types::api::request::{NewGameData, ScenarioRequest};
use dnl_types::generators::{GeneratorScenarioConfig, GeneratorType};
use dnl_types::scenarios::ScenarioModel;
use dnl_types::traits::DiscordMsg;
use reqwest::Response;
use serenity::all::Http;
use serenity::builder::*;
use serenity::model::application::*;

#[derive(Debug, PartialEq, Default)]
pub struct NewGameCmd;

#[async_trait::async_trait]
impl DiscordCmdResponse for NewGameCmd {
    async fn handle_sucessful_response<T>(
        res: Response,
    ) -> anyhow::Result<Option<CreateInteractionResponse>>
    where
        T: serde::de::DeserializeOwned,
    {
        let output: ScenarioModel = res.json().await.context("Failed to parse API Response")?;
        info!("OUTPUT {:?}", output);

        match output {
            ScenarioModel::NewGame { items, .. } => {
                let mut item_embeds = Vec::new();
                let mut buttons = Vec::new();

                for (i, item) in items.iter().enumerate() {
                    item_embeds.push(
                        CreateEmbed::new()
                            .color(serenity::model::Colour::BLUE)
                            .title(&item.name)
                            .field("Stats", item.stats.to_string(), false)
                            .field("Price", &item.price, true),
                    );
                    buttons.push(
                        CreateButton::new(format!("button_{}", i))
                            .style(ButtonStyle::Primary)
                            .label(&item.name),
                    )
                }

                let components = CreateActionRow::Buttons(buttons);

                let message = CreateInteractionResponseMessage::new()
                    .add_embeds(item_embeds)
                    .components(vec![components]);

                info!("EMBED {:?}", message);
                Ok(Some(CreateInteractionResponse::Message(message)))
            }
            _ => Err(anyhow::anyhow!(
                "Variant did not match ScenarioModel::NewGame"
            )),
        }
    }
    async fn handle_sucessful_response_with_followup<T>(
        res: Response,
        cmd: &CommandInteraction,
        http: &Http,
    ) -> anyhow::Result<()>
    where
        T: serde::de::DeserializeOwned + DiscordMsg + Send,
    {
        let output: ScenarioModel = res.json().await.context("Failed to parse API Response")?;
        info!("OUTPUT {:?}", output);

        let message = match output {
            ScenarioModel::NewGame { ref items, .. } => {
                let mut item_embeds = Vec::new();
                let mut buttons = Vec::new();

                let content = output.to_message();

                for (i, item) in items.iter().enumerate() {
                    item_embeds.push(
                        CreateEmbed::new()
                            .color(serenity::model::Colour::BLUE)
                            .title(&item.name)
                            .field("Stats", item.stats.to_string(), true)
                            .field("Price", &item.price, true),
                    );
                    buttons.push(
                        CreateButton::new(format!("button_{}", i))
                            .style(ButtonStyle::Primary)
                            .label(&item.name),
                    )
                }

                let components = CreateActionRow::Buttons(buttons);

                let message = CreateInteractionResponseFollowup::new()
                    .content(content)
                    .add_embeds(item_embeds)
                    .components(vec![components]);

                Some(message)
            }
            _ => None,
        };

        if let Some(message) = message {
            let _ = cmd.create_followup(&http, message).await;
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Variant did not match ScenarioModel::NewGame"
            ))
        }
        // let message = CreateInteractionResponseFollowup::new().content(output.to_message());
        //
        //
        // info!("Follow up: {:?}", res);
        //     _ => Err(anyhow::anyhow!(
        //         "Variant did not match ScenarioModel::NewGame"
        //     )),
        //
        // Ok(())
    }
}

impl NewGameCmd {
    pub async fn execute(
        &self,
        cmd: CommandInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let client = reqwest::Client::new();
        let token =
            std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

        let http = Http::new(&token);

        http.set_application_id(cmd.application_id);

        let url = format!("{}/{}", DNL_API_URL.to_string(), "api/game/new");

        info!("Defer pre");

        cmd.defer(&http).await.context("Failed to defer command")?;

        info!("Defer Post");

        let body = NewGameData::from(cmd.clone());

        match client.post(url).json(&body).send().await {
            Ok(res) => info!("new game response {:?}", res),
            Err(e) => return Err(e.into()),
        };

        let config = GeneratorScenarioConfig::NewGame;

        let generator = GeneratorType::Scenario(config);

        let payload = ScenarioRequest {
            user_id: cmd.user.id.to_string(),
            generator,
            options: vec![],
        };

        let url = format!("{}/{}", DNL_API_URL.to_string(), "api/llm/scenario");

        let response = client.post(url).json(&payload).send().await?;

        if response.status().is_success() {
            Self::handle_sucessful_response_with_followup::<ScenarioModel>(response, &cmd, &http)
                .await?
        } else {
            let text = response.text().await?;
            Self::handle_error_with_followup(&http, &cmd, text).await?;
        };

        Ok(None)
    }
}
