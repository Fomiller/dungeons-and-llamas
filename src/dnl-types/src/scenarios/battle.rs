use crate::dice::DiceExpression;
use crate::tools::battle::*;
use crate::tools::MockData;
use crate::traits::DiscordMsg;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleScenario {
    pub enemies: Vec<BattleScenarioEnemy>,
    pub summary: String,
    pub name: String,
    pub terrain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleScenarioEnemy {
    pub id: String,
    pub health: EntityHealth,
    pub r#type: String,
    pub armor_class: u8,
    pub attacks: BattleScenarioEnemyAttack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleScenarioEnemyAttack {
    pub expression: DiceExpression,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityHealth {
    pub current: u8,
    pub max: u8,
    pub expression: DiceExpression,
}

impl MockData for BattleScenario {
    fn mock() -> Self {
        let sword = BattleScenarioEnemyAttack {
            expression: DiceExpression {
                die_count: 1,
                die_size: 6,
                modifier: 2,
            },
            name: "Rusted Sword".to_string(),
        };
        let bow = BattleScenarioEnemyAttack {
            expression: DiceExpression {
                die_count: 1,
                die_size: 6,
                modifier: 2,
            },
            name: "ShortBow".to_string(),
        };

        let health_expression = DiceExpression {
            die_count: 1,
            die_size: 6,
            modifier: 2,
        };
        let health_expression_2 = DiceExpression {
            die_count: 2,
            die_size: 8,
            modifier: 0,
        };

        let health = EntityHealth {
            current: 8,
            max: 8,
            expression: health_expression,
        };
        let health_2 = EntityHealth {
            current: 8,
            max: 8,
            expression: health_expression_2,
        };

        let enemies = vec![
            BattleScenarioEnemy {
                id: Uuid::new_v4().to_string(),
                health: health,
                r#type: "Goblin".to_string(),
                armor_class: 10,
                attacks: sword,
            },
            BattleScenarioEnemy {
                id: Uuid::new_v4().to_string(),
                health: health_2,
                r#type: "Goblin Archer".to_string(),
                armor_class: 12,
                attacks: bow,
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

impl DiscordMsg for BattleScenario {
    fn to_message(&self) -> String {
        let mut enemy_description = String::new();

        for enemy in &self.enemies {
            let description = format!(
                "- **{}**\n  - Id: {}\n  - Attack: {}\n  - Damage: {}\n  - Armor: {}\n  - Health: {}\n  - Current: {}\n  - Max: {}\n",
                enemy.r#type,
                enemy.id,
                enemy.attacks.name,
                enemy.attacks.expression,
                enemy.armor_class,
                enemy.health.expression,
                enemy.health.current,
                enemy.health.max
            );
            enemy_description.push_str(&description);
        }

        format!(
            "# *{}*\n## Description:\n{}\n\n## Terrain:\n{}\n\n## Enemies:\n{}\n\n## Summary:\n{}",
            self.name, self.summary, self.terrain, enemy_description, self.summary
        )
    }
}

impl From<DiceExpression> for EntityHealth {
    fn from(output: DiceExpression) -> Self {
        let roll = output.roll();
        Self {
            current: roll,
            max: roll,
            expression: output,
        }
    }
}

impl From<BattleToolEnemyAttack> for BattleScenarioEnemyAttack {
    fn from(output: BattleToolEnemyAttack) -> Self {
        Self {
            expression: output.attack_expression,
            name: output.attack_name,
        }
    }
}

impl From<BattleToolEnemy> for BattleScenarioEnemy {
    fn from(output: BattleToolEnemy) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            health: EntityHealth::from(output.health.expression),
            r#type: output.enemy_type,
            armor_class: output.armor_class,
            attacks: BattleScenarioEnemyAttack::from(output.attack),
        }
    }
}

impl From<BattleToolOutput> for BattleScenario {
    fn from(output: BattleToolOutput) -> Self {
        Self {
            enemies: output.enemies.into_iter().map(Into::into).collect(),
            summary: output.summary,
            name: output.name,
            terrain: output.terrain,
        }
    }
}
