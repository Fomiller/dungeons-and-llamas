use super::MockData;
use crate::traits::DiscordMsg;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestToolOutput {}

impl MockData for RestToolOutput {
    fn mock() -> Self {
        Self {}
    }
}

impl DiscordMsg for RestToolOutput {
    fn to_message(&self) -> String {
        String::from("")
    }
}
