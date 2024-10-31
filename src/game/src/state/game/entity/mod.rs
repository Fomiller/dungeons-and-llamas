pub mod actions;
pub mod character;
pub mod enemy;
pub mod inventory;
pub mod npc;
pub mod stats;

use super::super::buildable::SortKeyBuildable;
use actions::ActionsSortKeyBuilder;
use inventory::InventorySortKeyBuilder;
use stats::StatsSortKeyBuilder;

#[derive(strum::Display, strum::EnumIter)]
pub enum EntitySortKey {
    #[strum(to_string = "Inventory#")]
    Inventory,
    #[strum(to_string = "Character#")]
    Character,
    #[strum(to_string = "Stats#")]
    Stats,
    #[strum(to_string = "Actions#")]
    Actions,
}

#[derive(Debug, Copy, Clone, strum::Display, strum::EnumIter)]
pub enum Entity {
    #[strum(to_string = "Player")]
    Player,
    #[strum(to_string = "Enemy")]
    Enemy,
    #[strum(to_string = "NPC")]
    NPC,
}

#[derive(Debug, Clone, Copy)]
pub struct EntitySortKeyBuilder {
    entity: Entity,
    actions: Option<ActionsSortKeyBuilder>,
    character: Option<bool>,
    inventory: Option<InventorySortKeyBuilder>,
    stats: Option<StatsSortKeyBuilder>,
}

impl EntitySortKeyBuilder {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            actions: None,
            character: None,
            inventory: None,
            stats: None,
        }
    }

    pub fn actions(mut self, actions: ActionsSortKeyBuilder) -> Self {
        self.actions = Some(actions);
        self
    }
    pub fn character(mut self, character: bool) -> Self {
        self.character = Some(character);
        self
    }
    pub fn inventory(mut self, inventory: InventorySortKeyBuilder) -> Self {
        self.inventory = Some(inventory);
        self
    }
    pub fn stats(mut self, stats: StatsSortKeyBuilder) -> Self {
        self.stats = Some(stats);
        self
    }

    pub fn build(self) -> String {
        let mut result = format!("{}#", self.entity);
        if let Some(inventory) = self.inventory {
            result.push_str(&format!("{}", inventory.build().to_string()));
        } else if let Some(character) = self.character {
            result.push_str(&format!("{}", character.to_string()));
        } else if let Some(stats) = self.stats {
            result.push_str(&format!("{}", stats.build().to_string()));
        } else if let Some(actions) = self.actions {
            result.push_str(&format!("{}", actions.build().to_string()));
        }
        result
    }
}
