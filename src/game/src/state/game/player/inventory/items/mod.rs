pub mod armor;
pub mod books_and_scrolls;
pub mod clothing;
pub mod equipped;
pub mod magic;
pub mod tools;
pub mod weapons;

use crate::state::SortKeyBuildable;

use armor::ArmorSortKeyBuilder;
use books_and_scrolls::BookAndScrollSortKey;
use clothing::ClothingSortKey;
use magic::MagicItemSortKey;
use tools::ToolSortKey;
use weapons::WeaponSortKeyBuilder;

pub use serde::{Deserialize, Serialize};

use std::any::Any;

#[derive(strum::Display, strum::EnumIter)]
pub enum ItemSortKey {
    #[strum(to_string = "Weapons#")]
    Weapons,
    #[strum(to_string = "Armor#")]
    Armor,
    #[strum(to_string = "Tools#")]
    Tools,
    #[strum(to_string = "AdventuringGear")]
    AdventuringGear,
    #[strum(to_string = "Magic#")]
    Magical,
    #[strum(to_string = "Consumables")]
    Consumables,
    #[strum(to_string = "Clothing#")]
    Clothing,
    #[strum(to_string = "Currency")]
    Currency,
    #[strum(to_string = "BooksAndScrolls#")]
    BooksAndScrolls,
    #[strum(to_string = "Miscellaneous")]
    Miscellaneous,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ItemSortKeyBuilder {
    // inventory options
    adventuring_gear: Option<bool>,
    armor: Option<ArmorSortKeyBuilder>,
    books_and_scrolls: Option<BookAndScrollSortKey>,
    clothing: Option<ClothingSortKey>,
    consumables: Option<bool>,
    currency: Option<bool>,
    magical: Option<MagicItemSortKey>,
    miscellaneous: Option<bool>,
    tools: Option<ToolSortKey>,
    weapons: Option<WeaponSortKeyBuilder>,
}

impl SortKeyBuildable for ItemSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from("Item#");
        if let Some(_) = self.adventuring_gear {
            result.push_str(&format!("{}", ItemSortKey::AdventuringGear));
        } else if let Some(armor) = self.armor {
            result.push_str(&format!("{}", armor.build().to_string()));
        } else if let Some(books_and_scrolls) = self.books_and_scrolls {
            result.push_str(&format!(
                "{}{}",
                ItemSortKey::BooksAndScrolls,
                books_and_scrolls.to_string()
            ));
        } else if let Some(clothing) = self.clothing {
            result.push_str(&format!("{}", clothing.to_string()));
        } else if let Some(_) = self.consumables {
            result.push_str(&format!("{}", ItemSortKey::Consumables));
        } else if let Some(_) = self.currency {
            result.push_str(&format!("{}", ItemSortKey::Currency));
        } else if let Some(magical) = self.magical {
            result.push_str(&format!("{}{}", ItemSortKey::Magical, magical.to_string()));
        } else if let Some(_) = self.miscellaneous {
            result.push_str(&format!("{}", ItemSortKey::Miscellaneous));
        } else if let Some(tools) = self.tools {
            result.push_str(&format!("{}{}", ItemSortKey::Tools, tools.to_string()))
        } else if let Some(weapons) = self.weapons {
            result.push_str(&format!("{}", weapons.build().to_string()))
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for ItemSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(item) = skb.as_any().downcast_ref::<ItemSortKeyBuilder>() {
            return *item;
        }
        Self::default()
    }
}

impl ItemSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn adventuring_gear(mut self) -> Self {
        self.adventuring_gear = Some(true);
        self
    }

    pub fn armor(mut self, armor: ArmorSortKeyBuilder) -> Self {
        self.armor = Some(armor);
        self
    }

    pub fn books_and_scrolls(mut self, books_and_scrolls: BookAndScrollSortKey) -> Self {
        self.books_and_scrolls = Some(books_and_scrolls);
        self
    }

    pub fn clothing(mut self, clothing: ClothingSortKey) -> Self {
        self.clothing = Some(clothing);
        self
    }

    pub fn consumables(mut self) -> Self {
        self.consumables = Some(true);
        self
    }

    pub fn currency(mut self) -> Self {
        self.currency = Some(true);
        self
    }

    pub fn magical(mut self, magical: MagicItemSortKey) -> Self {
        self.magical = Some(magical);
        self
    }

    pub fn miscellaneous(mut self) -> Self {
        self.miscellaneous = Some(true);
        self
    }

    pub fn tools(mut self, tools: ToolSortKey) -> Self {
        self.tools = Some(tools);
        self
    }

    pub fn weapons(mut self, weapons: WeaponSortKeyBuilder) -> Self {
        self.weapons = Some(weapons);
        self
    }
}
