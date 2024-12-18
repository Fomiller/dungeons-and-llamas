use crate::commands::*;
use lambda_http::tracing::info;
use serenity::builder::*;
use serenity::model::application::*;
use std::str::FromStr;
use strum::EnumString;

pub async fn try_handle_component_interaction(
    interaction: ComponentInteraction,
) -> anyhow::Result<Option<CreateInteractionResponse>> {
    // custom_id's need to become Enums
    match CustomId::from_str(&interaction.data.custom_id)? {
        CustomId::BackGroundMenu(cmd) => cmd.execute(interaction),
        CustomId::ClassMenu(cmd) => cmd.execute(interaction).await,
        CustomId::RaceMenu(cmd) => cmd.execute(interaction),
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct ClassCmd;

#[derive(Debug, PartialEq, Default)]
pub struct ClassMenuCmd;

#[derive(Debug, PartialEq, Default)]
pub struct RaceMenuCmd;

#[derive(Debug, PartialEq, Default)]
pub struct BackGroundMenuCmd;

#[derive(Debug, PartialEq, EnumString)]
pub enum CustomId {
    #[strum(serialize = "class_menu", ascii_case_insensitive)]
    ClassMenu(ClassMenuCmd),
    #[strum(serialize = "race_menu", ascii_case_insensitive)]
    RaceMenu(RaceMenuCmd),
    #[strum(serialize = "background_menu", ascii_case_insensitive)]
    BackGroundMenu(BackGroundMenuCmd),
}

impl ClassMenuCmd {
    pub async fn execute(
        &self,
        interaction: ComponentInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        info!("Class CMD");
        EditCmd::new().execute(interaction).await?;
        let duration = std::time::Duration::from_secs(2);
        std::thread::sleep(duration);
        let message = CreateInteractionResponseMessage::new()
            .embeds(vec![])
            .content("LLM RESPONSE")
            .components(vec![]);
        Ok(Some(CreateInteractionResponse::UpdateMessage(message)))
    }
}

impl RaceMenuCmd {
    pub fn execute(
        &self,
        interaction: ComponentInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!(
            "custom_id: {:?}, kind: {:?}",
            interaction.data.custom_id, interaction.data.kind
        );
        let message = CreateInteractionResponseMessage::new().content(content);
        Ok(Some(CreateInteractionResponse::UpdateMessage(message)))
    }
}

impl BackGroundMenuCmd {
    pub fn execute(
        &self,
        interaction: ComponentInteraction,
    ) -> anyhow::Result<Option<CreateInteractionResponse>> {
        let content = format!(
            "custom_id: {:?}, kind: {:?}",
            interaction.data.custom_id, interaction.data.kind
        );
        let message = CreateInteractionResponseMessage::new().content(content);
        Ok(Some(CreateInteractionResponse::UpdateMessage(message)))
    }
}
