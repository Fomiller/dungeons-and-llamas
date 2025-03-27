pub mod error;
pub mod request;
pub mod response;

use serenity::model::application::*;

pub fn find_options_value(options: &[CommandDataOption], name: &str) -> Option<String> {
    Some(
        options
            .iter()
            .find(|option| option.name == name)
            .map(|option| {
                option.value.as_str().expect(&format!(
                    "CommandDataOption.name '{}' should be a string value",
                    name
                ))
            })?
            .to_string(),
    )
}
