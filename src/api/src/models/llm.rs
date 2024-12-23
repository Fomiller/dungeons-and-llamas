#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmConverseInput {
    pub model: String,
    pub prompt: String,
    pub system: String,
    pub instructions: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ScenarioInput {
    pub model: String,
    pub scenario: String,
    pub theme: String,
    pub level: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BattleToolResponse {
    pub enemies: Vec<Enemy>,
    pub summary: String,
    pub name: String,
    pub terrain: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Enemy {
    pub health: u8,
    pub enemy_type: String,
    pub attack_damage: String,
    pub attack_name: String,
}

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct EnemyAttack {
//     pub attack_damage: u8,
//     pub attack_name: String,
// }
