pub use serde::{Deserialize, Serialize};

pub mod armor;
pub mod books_and_scrolls;
pub mod clothing;
pub mod magic;
pub mod tools;
pub mod weapons;

use armor::ArmorEquippedStateSortKey;
use books_and_scrolls::BookAndScrollSortKey;
use clothing::ClothingSortKey;
use magic::MagicItemSortKey;
use tools::ToolSortKey;
use weapons::WeaponEquippedStateSortKey;

#[derive(strum::Display)]
pub enum ItemSortKey {
    #[strum(to_string = "#Weapons#{0}")]
    Weapons(WeaponEquippedStateSortKey),
    #[strum(to_string = "#Armor#{0}")]
    Armor(ArmorEquippedStateSortKey),
    #[strum(to_string = "#Tools#{0}")]
    Tools(ToolSortKey),
    #[strum(to_string = "#AdventuringGear")]
    AdventuringGear,
    #[strum(to_string = "#MagicItems#{0}")]
    MagicItems(MagicItemSortKey),
    #[strum(to_string = "#Consumables")]
    Consumables,
    #[strum(to_string = "#Clothing#{0}")]
    Clothing(ClothingSortKey),
    #[strum(to_string = "#Currency")]
    Currency,
    #[strum(to_string = "#BooksAndScrolls#{0}")]
    BooksAndScrolls(BookAndScrollSortKey),
    #[strum(to_string = "#Miscellaneous")]
    Miscellaneous,
}
