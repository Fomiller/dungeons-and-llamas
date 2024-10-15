pub mod actions;
pub mod character;
pub mod inventory;
pub mod stats;

use actions::ActionsSortKeyBuilder;
use inventory::InventorySortKeyBuilder;
use stats::StatsSortKeyBuilder;

#[derive(strum::Display)]
pub enum PlayerSortKey {
    #[strum(to_string = "Inventory#")]
    Inventory,
    #[strum(to_string = "Character#")]
    Character,
    #[strum(to_string = "Stats#")]
    Stats,
    #[strum(to_string = "Actions#")]
    Actions,
}

#[derive(Default)]
pub struct PlayerSortKeyBuilder {
    actions: Option<ActionsSortKeyBuilder>,
    character: Option<bool>,
    inventory: Option<InventorySortKeyBuilder>,
    stats: Option<StatsSortKeyBuilder>,
}

impl PlayerSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
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
        let mut result = String::from("GameState#");
        if let Some(inventory) = self.inventory {
            result.push_str(&format!("#{}", inventory.build().to_string()));
        } else if let Some(character) = self.character {
            result.push_str(&format!("#{}", character.to_string()));
        } else if let Some(stats) = self.stats {
            result.push_str(&format!("#{}", stats.build().to_string()));
        } else if let Some(actions) = self.actions {
            result.push_str(&format!("#{}", actions.build().to_string()));
        }
        result
    }
}
