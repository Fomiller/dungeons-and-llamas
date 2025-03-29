use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiceExpression {
    pub die_count: u8,
    pub die_size: u8,
    pub modifier: u8,
}

impl DiceExpression {
    pub fn roll(&self) -> u8 {
        let mut roll = 0;

        for _ in 0..self.die_count {
            roll += rand::thread_rng().gen_range(1..self.die_size)
        }

        if self.modifier != 0 {
            roll += self.modifier
        }

        roll
    }
}

impl std::fmt::Display for DiceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut expression = format!("{}d{}", self.die_count, self.die_size);
        if self.modifier != 0 {
            let mod_expression = format!("+{}", self.modifier);
            expression.push_str(&mod_expression)
        }
        write!(f, "{}", expression)
    }
}
