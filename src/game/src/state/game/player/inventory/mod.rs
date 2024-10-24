pub mod items;

use crate::state::SortKeyBuildable;
use items::ItemSortKeyBuilder;

#[derive(strum::Display, strum::EnumIter)]
pub enum InventorySortKey {
    #[strum(to_string = "Item#")]
    Item,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InventorySortKeyBuilder {
    pub item: Option<ItemSortKeyBuilder>,
}

impl InventorySortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: ItemSortKeyBuilder) -> Self {
        self.item = Some(item);
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from("Inventory#");
        if let Some(item) = self.item {
            result.push_str(&format!("{}", item.build().to_string()));
        }
        result
    }
}

impl From<Box<dyn SortKeyBuildable>> for InventorySortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(inventory) = skb.as_any().downcast_ref::<Self>() {
            return *inventory;
        }
        Self::default()
    }
}
