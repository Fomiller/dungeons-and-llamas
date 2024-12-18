use crate::commands;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::ModalInteraction;

pub fn try_handle_modal_interaction(
    interaction: ModalInteraction,
) -> anyhow::Result<Option<CreateInteractionResponse>> {
    let content = format!(
        "custom_id: {:?}, kind: {:?}",
        interaction.data.custom_id, interaction.data.components
    );
    Ok(Some(commands::format_interaction_response(content)))
}
