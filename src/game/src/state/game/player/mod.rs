use actions::ActionsSortKey;
use character::CharacterSortKey;
use inventory::InventorySortKey;
use stats::StatsSortKey;

pub mod actions;
pub mod character;
pub mod inventory;
pub mod stats;

#[derive(strum::Display)]
pub enum PlayerSortKey {
    #[strum(to_string = "#Inventory#{0}")]
    Inventory(InventorySortKey),
    #[strum(to_string = "#Character#{0}")]
    Character(CharacterSortKey),
    #[strum(to_string = "#Stats#{0}")]
    Stats(StatsSortKey),
    #[strum(to_string = "#Actions#{0}")]
    Actions(ActionsSortKey),
}
