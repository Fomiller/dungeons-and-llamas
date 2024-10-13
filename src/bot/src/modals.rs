use crate::commands;
use serenity::builder::CreateInteractionResponse;
use serenity::model::application::ModalInteraction;

pub fn handle_modal_interaction(interaction: ModalInteraction) -> CreateInteractionResponse {
    let content = format!(
        "custom_id: {:?}, kind: {:?}",
        interaction.data.custom_id, interaction.data.components
    );
    commands::format_interaction_response(content)
}
