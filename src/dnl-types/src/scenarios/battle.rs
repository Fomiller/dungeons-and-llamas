use crate::dice::DiceExpression;
use crate::entity::EntityHealth;
use crate::tools::battle::*;
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
    fn from(source: DiceExpression) -> Self {
        let roll = source.roll();
        Self {
            current: roll,
            max: roll,
            expression: source,
        }
    }
}

impl From<BattleToolEnemyAttack> for BattleScenarioEnemyAttack {
    fn from(source: BattleToolEnemyAttack) -> Self {
        Self {
            expression: source.attack_expression,
            name: source.attack_name,
        }
    }
}

impl From<BattleToolEnemy> for BattleScenarioEnemy {
    fn from(source: BattleToolEnemy) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            health: EntityHealth::from(source.health.expression),
            r#type: source.enemy_type,
            armor_class: source.armor_class,
            attacks: BattleScenarioEnemyAttack::from(source.attack),
        }
    }
}

impl From<BattleToolOutput> for BattleScenario {
    fn from(source: BattleToolOutput) -> Self {
        Self {
            enemies: source.enemies.into_iter().map(Into::into).collect(),
            summary: source.summary,
            name: source.name,
            terrain: source.terrain,
        }
    }
}
