pub mod error;
pub mod request;
pub mod response;

use serenity::model::application::*;

fn find_options_value(options: &[CommandDataOption], name: &str) -> String {
    options
        .iter()
        .find(|option| option.name == name)
        .map(|option| {
            option.value.as_str().expect(&format!(
                "CommandDataOption.name '{}' should be a string value",
                name
            ))
        })
        .expect(&format!(
            "CommandDataOption.name '{}' should be present in CommandInteraction",
            name
        ))
        .to_string()
}
