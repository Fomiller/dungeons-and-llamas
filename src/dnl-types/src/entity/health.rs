use crate::dice::DiceExpression;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityHealth {
    pub current: u8,
    pub max: u8,
    pub expression: DiceExpression,
}
