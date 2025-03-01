use super::MockData;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolOutput {
    pub enemies: Vec<BattleToolEnemy>,
    pub summary: String,
    pub name: String,
    pub terrain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolEnemy {
    pub health: String,
    pub enemy_type: String,
    pub attack: BattleToolEnemyAttack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolEnemyAttack {
    pub attack_damage: String,
    pub attack_name: String,
}

impl MockData for BattleToolOutput {
    fn mock() -> Self {
        let sword = BattleToolEnemyAttack {
            attack_damage: "1d6+2".to_string(),
            attack_name: "Rusted Sword".to_string(),
        };
        let bow = BattleToolEnemyAttack {
            attack_damage: "1d6+2".to_string(),
            attack_name: "ShortBow".to_string(),
        };
        let enemies = vec![
            BattleToolEnemy {
                health: "1d6+8".to_string(),
                enemy_type: "Goblin".to_string(),
                attack: sword,
            },
            BattleToolEnemy {
                health: "1d6+8".to_string(),
                enemy_type: "Goblin Archer".to_string(),
                attack: bow,
            },
        ];

        let summary = "summary of the scenario".to_string();
        let name = "a dangerous encounter".to_string();
        let terrain = "description of the terrain".to_string();

        Self {
            enemies,
            summary,
            name,
            terrain,
        }
    }
}

