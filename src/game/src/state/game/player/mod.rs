use actions::ActionsSortKey;
use character::CharacterSortKey;
use inventory::InventorySortKey;
use stats::StatsSortKey;

use self::{
    actions::spells::{SpellSortKey, SpellTypeSortKey},
    inventory::items::{
        armor::ArmorSortKey, books_and_scrolls::BookAndScrollSortKey, clothing::ClothingSortKey,
        magic::MagicItemSortKey, tools::ToolSortKey, weapons::WeaponSortKey, ItemSortKey,
    },
    stats::{
        abilities::AbilitiesSortKey, conditions::ConditionsSortKey,
        core_attributes::CoreAttributesSortKey, saving_throws::SavingThrowsSortKey,
        skills::SkillsSortKey,
    },
};

pub mod actions;
pub mod character;
pub mod inventory;
pub mod stats;

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
    actions: Option<ActionsSortKey>,
    character: Option<bool>,
    inventory: Option<InventorySortKey>,
    stats: Option<StatsSortKey>,
}

impl PlayerSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn actions(mut self, actions: ActionsSortKey) -> Self {
        self.actions = Some(actions);
        self
    }
    pub fn character(mut self, character: bool) -> Self {
        self.character = Some(character);
        self
    }
    pub fn inventory(mut self, inventory: InventorySortKey) -> Self {
        self.inventory = Some(inventory);
        self
    }
    pub fn stats(mut self, stats: StatsSortKey) -> Self {
        self.stats = Some(stats);
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from("GameState#");
        if let Some(inventory) = self.inventory {
            result.push_str(&format!("#{}", inventory.to_string()));
        } else if let Some(character) = self.character {
            result.push_str(&format!("#{}", character.to_string()));
        } else if let Some(stats) = self.stats {
            result.push_str(&format!("#{}", stats.to_string()));
        } else if let Some(actions) = self.actions {
            result.push_str(&format!("#{}", actions.to_string()));
        }
        result
    }
}
